use std::{
    thread::{self, JoinHandle},
    time::{SystemTime, Duration},
    sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex}
};
use log;

use crate::{
    config::AppConfig,
    io::interface::{uart::DrivesUart, gpio::DrivesGpio, pwm::DrivesPwm},
    drivers::nucifera_driver::{NuciferaDriver, self}, models::position::Position
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

    nucifera_driver: NuciferaDriver,

    motor_controller: MotorController,
    api_controller: ApiController,
}

impl<GpioDriver: DrivesGpio + Send + 'static,
     PwmDriver: DrivesPwm + Send + 'static,
     UartDriver: DrivesUart + Send + 'static>
MasterController<GpioDriver, PwmDriver, UartDriver> {
    pub fn new(app_cfg: &AppConfig,
               gpio_driver: GpioDriver,
               pwm_driver: PwmDriver,
               uart_driver: UartDriver) -> Self {
        Self {
            gpio_driver: Arc::new(Mutex::new(gpio_driver)),
            pwm_driver: Arc::new(Mutex::new(pwm_driver)),
            uart_driver: Arc::new(Mutex::new(uart_driver)),

            api_controller: ApiController::new("ipc:///tmp/cocos-api"),

            nucifera_driver: NuciferaDriver::new(app_cfg.nucifera),
            motor_controller: MotorController::new(app_cfg.mot_left,
                                                   app_cfg.mot_right)
        }
    }

    fn init(&self) {
        let gpio_driver_rc = self.gpio_driver.clone();
        let gpio_driver = gpio_driver_rc.lock().unwrap();

        self.motor_controller.block(&*gpio_driver)
            .expect("Could not block the motor controller on start.");

        log::debug!(target: "system.master", "Successfully initialized");
    }

    fn spawn_tasks(&self) {
        // Channel definitions.
        let (tx_pos_rx_log_send, tx_pos_rx_log_recv):
            (Sender<Position>, Receiver<Position>) = mpsc::channel();

        // Logging task.
        let logging_task = spawn_task(move || {
            let current_pos = tx_pos_rx_log_recv.recv();
            log::debug!(target: "system.master.position",
                        "Measured position: {}", current_pos.unwrap())
        }, Duration::from_millis(100), "Logging Task");

        // Positioning Task
        let pos_uart_driver = Arc::clone(&self.uart_driver);
        let nucifera_driver = self.nucifera_driver; // TODO: Should be ref
        let positioning_task = spawn_task(move || {
            let uart_driver = pos_uart_driver.lock().unwrap();

            let current_pos = nucifera_driver.read_current_position(
                &uart_driver);
            tx_pos_rx_log_send.send(current_pos).unwrap();
        }, Duration::from_millis(1), "Position Input Task");

        // API Task
        spawn_task(move || {
            // TODO
        }, Duration::from_millis(1), "API Task");

        // Driving Task
        let motor_controller = self.motor_controller; // TODO: Should be ref
        spawn_task(move || {
        }, Duration::from_millis(1), "Motion Output Task");

        log::debug!(target: "system.master", "Spawned tasks");

        // Idle task.
        loop { }
    }

    pub fn run(&self) {
        self.init();
        self.spawn_tasks();
    }
}
