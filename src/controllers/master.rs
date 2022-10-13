use std::{thread::{self, JoinHandle}, time::{SystemTime, Duration}, sync::{mpsc, Arc}, rc::Rc, cell::RefCell, borrow::Borrow};
use std::sync::Mutex;
use log::warn;

use crate::{
    config::AppConfig,
    io::interface::{uart::DrivesUart, gpio::DrivesGpio, pwm::DrivesPwm},
    drivers::nucifera_driver::NuciferaDriver
};

use super::{motor::MotorController, api::ApiController};

fn spawn_task<F, T>(f: F, period: Duration,
                    name: &'static str) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static {
    thread::spawn(move || {
        loop {
            let start_time = SystemTime::now();
            // TODO: Call the function here.
            let stop_time = SystemTime::now();
            let delta = stop_time.duration_since(start_time).unwrap();
            if delta > period {
                warn!("{name} could not be completed in time.");
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

            api_controller: ApiController::new(),

            nucifera_driver: NuciferaDriver::new(app_cfg.nucifera),
            motor_controller: MotorController::new(app_cfg.mot_left,
                                                   app_cfg.mot_right)
        }
    }

    fn init(&self) {
    }

    fn spawn_tasks(&self) {
        // Channel definitions.

        // Positioning Task
        let pos_uart_driver = Arc::clone(&self.uart_driver);
        let positioning_task = spawn_task(move || {
            let uart_driver = pos_uart_driver.lock().unwrap();
        }, Duration::from_millis(1), "Positioning Task");

        // Idle task.
        loop { }
    }

    pub fn run(&self) {
        self.init();
        self.spawn_tasks();
    }
}
