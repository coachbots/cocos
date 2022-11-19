use log;
use std::{
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle, sleep},
    time::{Duration, SystemTime},
};

use crate::{
    config::AppConfig,
    drivers::{led_driver::LedDriver, nucifera_driver::NuciferaDriver},
    io::interface::{gpio::DrivesGpio, pwm::DrivesPwm, uart::DrivesUart},
    models::{
        api::ApiTickInputMessage, led_color::LedColor, motor_power::MotorPower, position::Position,
    }, controllers::api,
};

use super::{api::ApiController, motor::MotorController};

fn spawn_task<F, T>(f: F, period: Duration, name: &'static str) -> JoinHandle<T>
where
    F: Fn() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::spawn(move || loop {
        let start_time = SystemTime::now();
        f();
        let stop_time = SystemTime::now();
        let delta = stop_time.duration_since(start_time).unwrap();
        if delta > period {
            log::warn!("{name} could not be completed in time.");
        }
        thread::sleep(period.saturating_sub(delta));
    })
}

pub struct MasterController<
    GpioDriver: DrivesGpio + Send + 'static,
    PwmDriver: DrivesPwm + Send + 'static,
    UartDriver: DrivesUart + Send + 'static,
> {
    gpio_driver: Arc<Mutex<GpioDriver>>,
    pwm_driver: Arc<Mutex<PwmDriver>>,
    uart_driver: Arc<Mutex<UartDriver>>,

    led_driver: LedDriver,
    nucifera_driver: NuciferaDriver,

    motor_controller: MotorController,
    api_controller: ApiController,

    // TODO: Does not need to be an ARC
    current_pos: Arc<RwLock<Position>>,
    // TODO: Does not need to be an ARC
    current_mot_pow: Arc<RwLock<MotorPower>>,
    // TODO: Does not need to be an ARC
    current_led_color: Arc<RwLock<LedColor>>,
}

impl<
        GpioDriver: DrivesGpio + Send + 'static,
        PwmDriver: DrivesPwm + Send + 'static,
        UartDriver: DrivesUart + Send + 'static,
    > MasterController<GpioDriver, PwmDriver, UartDriver>
{
    pub fn new(
        app_cfg: &AppConfig,
        gpio_driver: GpioDriver,
        pwm_driver: PwmDriver,
        uart_driver: UartDriver,
    ) -> Self {
        Self {
            gpio_driver: Arc::new(Mutex::new(gpio_driver)),
            pwm_driver: Arc::new(Mutex::new(pwm_driver)),
            uart_driver: Arc::new(Mutex::new(uart_driver)),
            led_driver: LedDriver::new(app_cfg.led),

            api_controller: ApiController::new("ipc:///tmp/cocos-api"),

            nucifera_driver: NuciferaDriver::new(app_cfg.nucifera),
            motor_controller: MotorController::new(app_cfg.mot_left, app_cfg.mot_right),

            current_pos: Arc::new(RwLock::new(Position::zero())),
            current_mot_pow: Arc::new(RwLock::new(MotorPower::zero())),
            current_led_color: Arc::new(RwLock::new(LedColor::off())),
        }
    }

    fn init(&mut self) {
        let gpio_driver_rc = self.gpio_driver.clone();
        let mut gpio_driver = gpio_driver_rc.lock().unwrap();

        self.motor_controller
            .block(&mut *gpio_driver)
            .expect("Could not block the motor controller on start.");

        self.api_controller.restart_api().expect("Could not spawn the child process.");

        log::debug!(target: "system.master", "Successfully initialized");
    }

    fn spawn_tasks(&mut self) {
        // Channel definitions.

        // Logging task.
        let current_pos = Arc::clone(&self.current_pos);
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let current_led_color = Arc::clone(&self.current_led_color);
        let logging_task = spawn_task(
            move || {
                // TODO: Potentially dangerous unwrap
                let pos = current_pos.read().unwrap().clone();
                let mot_pow = current_mot_pow.read().unwrap().clone();
                let led_color = current_led_color.read().unwrap().clone();
                log::info!(target: "system.master.position", "Current: {}", pos);
                log::info!(target: "system.master.motor_power", "Current: {}", mot_pow);
                log::info!(target: "system.master.led_color", "Current: {}", led_color)
            },
            Duration::from_millis(100),
            "Logging Task",
        );

        // Positioning Task
        let pos_uart_driver = Arc::clone(&self.uart_driver);
        let nucifera_driver = self.nucifera_driver; // TODO: Should be ref
        let current_pos = Arc::clone(&self.current_pos);
        let positioning_task = spawn_task(
            move || {
                let uart_driver = pos_uart_driver.lock().unwrap();
                let new_pos = nucifera_driver.read_current_position(&uart_driver);
                // TODO: Potentially dangerous unwrap
                *current_pos.write().unwrap() = new_pos;
            },
            Duration::from_millis(1),
            "Position Input Task",
        );

        // LED Task
        let current_led_color = Arc::clone(&self.current_led_color);
        let pwm_driver = Arc::clone(&self.pwm_driver);
        let led_controller = self.led_driver; // TODO: Should be ref
        let led_task = spawn_task(move || {
            let mut pwm = pwm_driver.lock().unwrap();
            let led = current_led_color.read().unwrap();
            led_controller.set_color(*led, &mut *pwm);
        }, Duration::from_millis(10), "LED Task",);

        // API Task
        let current_pos = Arc::clone(&self.current_pos);
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let current_led_color = Arc::clone(&self.current_led_color);
        let api_controller = &mut self.api_controller;
        thread::scope(|s| {
            s.spawn(move || {
                loop {
                    let pos = current_pos.read().unwrap().clone();
                    let tick_data = ApiTickInputMessage { bot_pos: pos };
                    match api_controller.run_tick(tick_data) {
                        Ok(api_data) => {
                            log::debug!("{:?}", api_data);
                            if let Some(mot_pow) = api_data.request_motor_power {
                                *current_mot_pow.write().unwrap() = mot_pow;
                            }
                            if let Some(led_color) = api_data.request_led_color {
                                *current_led_color.write().unwrap() = led_color;
                            }
                        }
                        Err(err) => {
                            log::error!(target: "system.master.api", "Received error: {:?}", err);
                        }
                    }

                    thread::sleep(Duration::from_millis(5));
                }
            });
        });

        // Driving Task
        let motor_controller = self.motor_controller; // TODO: Should be ref
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let gpio_driver = Arc::clone(&self.gpio_driver);
        let pwm_driver = Arc::clone(&self.pwm_driver);
        spawn_task(
            move || {
                let mut gpio = gpio_driver.lock().unwrap();
                let mut pwm = pwm_driver.lock().unwrap();
                let mot_pow = current_mot_pow.read().unwrap();
                motor_controller.set_vel(*mot_pow, &mut *gpio, &mut *pwm);
            },
            Duration::from_millis(10),
            "Motion Output Task",
        );

        log::debug!(target: "system.master", "Spawned tasks");

        // Idle task.
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn run(&mut self) {
        self.init();
        self.spawn_tasks();
    }

    pub fn run_with_script(&mut self, script: String) {
        self.api_controller.kill();
        self.api_controller.set_script(script.as_bytes().to_vec());
        self.run();
    }
}
