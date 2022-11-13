use std::{
    thread::{self, JoinHandle},
    time::{SystemTime, Duration},
    sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex}
};
use log;

use crate::{
    config::AppConfig,
    io::interface::{uart::DrivesUart, gpio::DrivesGpio, pwm::DrivesPwm},
    drivers::{nucifera_driver::NuciferaDriver, led_driver::{LedDriver, self}},
    models::{position::Position, api::ApiTickMessage, motor_power::MotorPower, led_color::LedColor}, controllers::api
};

use super::{motor::MotorController, api::ApiController};

fn spawn_task<F, T>(f: F, period: Duration,
                    name: &'static str) -> JoinHandle<T>
    where
        F: Fn() -> T,
        F: Send + 'static,
        T: Send + 'static {
    thread::spawn(move || {
        loop {
            let start_time = SystemTime::now();
            f();
            let stop_time = SystemTime::now();
            let delta = stop_time.duration_since(start_time).unwrap();
            if delta > period {
                log::warn!("{name} could not be completed in time.");
            }
            thread::sleep(period.saturating_sub(delta));
        }
    })
}

pub struct MasterController<GpioDriver: DrivesGpio + Send + 'static,
                            PwmDriver: DrivesPwm + Send + 'static,
                            UartDriver: DrivesUart + Send + 'static> {
    gpio_driver: Arc<Mutex<GpioDriver>>,
    pwm_driver: Arc<Mutex<PwmDriver>>,
    uart_driver: Arc<Mutex<UartDriver>>,

    led_driver: LedDriver,
    nucifera_driver: NuciferaDriver,

    motor_controller: MotorController,
    // TODO: Does not need to be a mutex
    api_controller: Arc<Mutex<ApiController>>,

    // TODO: Does not need to be an ARC
    current_pos: Arc<Mutex<Position>>,
    // TODO: Does not need to be an ARC
    current_mot_pow: Arc<Mutex<MotorPower>>,
    // TODO: Does not need to be an ARC
    current_led_color: Arc<Mutex<LedColor>>
}

impl<GpioDriver: DrivesGpio + Send + 'static,
     PwmDriver: DrivesPwm + Send + 'static,
     UartDriver: DrivesUart + Send + 'static>
MasterController<GpioDriver, PwmDriver, UartDriver> {
    pub fn new(app_cfg: &AppConfig,
               gpio_driver: GpioDriver,
               pwm_driver: PwmDriver,
               uart_driver: UartDriver,) -> Self {
        Self {
            gpio_driver: Arc::new(Mutex::new(gpio_driver)),
            pwm_driver: Arc::new(Mutex::new(pwm_driver)),
            uart_driver: Arc::new(Mutex::new(uart_driver)),
            led_driver: LedDriver::new(app_cfg.led),

            api_controller: Arc::new(
                Mutex::new(ApiController::new("ipc:///tmp/cocos-api"))),

            nucifera_driver: NuciferaDriver::new(app_cfg.nucifera),
            motor_controller: MotorController::new(app_cfg.mot_left,
                                                   app_cfg.mot_right),

            current_pos: Arc::new(Mutex::new(Position::zero())),
            current_mot_pow: Arc::new(Mutex::new(MotorPower::zero())),
            current_led_color: Arc::new(Mutex::new(LedColor::off())),
        }
    }

    fn init(&self) {
        let gpio_driver_rc = self.gpio_driver.clone();
        let mut gpio_driver = gpio_driver_rc.lock().unwrap();

        self.motor_controller.block(&mut *gpio_driver)
            .expect("Could not block the motor controller on start.");

        self.api_controller.lock().unwrap().restart_api()
            .expect("Could not spawn the child process.");

        log::debug!(target: "system.master", "Successfully initialized");
    }

    fn spawn_tasks(&self) {
        // Channel definitions.

        // Logging task.
        let current_pos = Arc::clone(&self.current_pos);
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let logging_task = spawn_task(move || {
            // TODO: Potentially dangerous unwrap
            let pos = current_pos.lock().unwrap().clone();
            let mot_pow = current_mot_pow.lock().unwrap().clone();
            log::info!(target: "system.master.position",
                       "Current position: {}", pos);
            log::info!(target: "system.master.motor_power",
                       "Current motor power: {}", mot_pow);
        }, Duration::from_millis(100), "Logging Task");

        // Positioning Task
        let pos_uart_driver = Arc::clone(&self.uart_driver);
        let nucifera_driver = self.nucifera_driver; // TODO: Should be ref
        let current_pos = Arc::clone(&self.current_pos);
        let positioning_task = spawn_task(move || {
            let uart_driver = pos_uart_driver.lock().unwrap();
            let new_pos = nucifera_driver.read_current_position(&uart_driver);
            // TODO: Potentially dangerous unwrap
            *current_pos.lock().unwrap() = new_pos;
        }, Duration::from_millis(1), "Position Input Task");

        // LED Task
        let current_led_color = Arc::clone(&self.current_led_color);
        let pwm_driver = Arc::clone(&self.pwm_driver);
        let led_controller = self.led_driver; // TODO: Should be ref
        let led_task = spawn_task(move || {
            let mut pwm = pwm_driver.lock().unwrap();
            let led = current_led_color.lock().unwrap();
            led_controller.set_color(*led, &mut *pwm);
        }, Duration::from_millis(10), "LED Task");

        // API Task
        let api_controller = Arc::clone(&self.api_controller);
        let current_pos = Arc::clone(&self.current_pos);
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let api_task = spawn_task(move || {
            let pos = current_pos.lock().unwrap().clone();
            let tick_data = ApiTickMessage {
                bot_pos: pos
            };
            match api_controller.lock().unwrap().run_tick(tick_data) {
                Ok(api_output) => {
                    if let Some(mot_pow) = api_output.request_motor_power {
                        *current_mot_pow.lock().unwrap() = mot_pow;
                    }
                    if let Some(led_col) = api_output.request_led_color {
                    }
                }
                Err(error) => {
                    // TODO: Handle this case
                }
            };
        }, Duration::from_millis(1), "API Task");

        // Driving Task
        let motor_controller = self.motor_controller; // TODO: Should be ref
        let current_mot_pow = Arc::clone(&self.current_mot_pow);
        let gpio_driver = Arc::clone(&self.gpio_driver);
        spawn_task(move || {
            let mut gpio = gpio_driver.lock().unwrap();
            let mot_pow = current_mot_pow.lock().unwrap();
            motor_controller.set_vel(*mot_pow, &mut *gpio);
        }, Duration::from_millis(10), "Motion Output Task");

        log::debug!(target: "system.master", "Spawned tasks");

        // Idle task.
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn run(&self) {
        self.init();
        self.spawn_tasks();
    }
}
