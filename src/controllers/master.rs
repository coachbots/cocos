use std::{thread::{self, JoinHandle}, time::{SystemTime, Duration}, sync::mpsc};
use log::warn;

use crate::{
    config::AppConfig,
    io::interface::{uart::DrivesUart, gpio::DrivesGpio, pwm::DrivesPwm}, drivers::nucifera_driver::NuciferaDriver, models::position::Position,
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
            f();
            let stop_time = SystemTime::now();
            let delta = stop_time.duration_since(start_time).unwrap();
            if delta > period {
                warn!("{name} could not be completed in time.");
            }
            thread::sleep(period.saturating_sub(delta));
        }
    })
}

pub struct MasterController<GpioDriver: DrivesGpio,
                            PwmDriver: DrivesPwm,
                            UartDriver: DrivesUart> {
    gpio_driver: GpioDriver,
    pwm_driver: PwmDriver,
    uart_driver: UartDriver,

    nucifera_driver: NuciferaDriver,

    motor_controller: MotorController,
    api_controller: ApiController,
}

impl<GpioDriver: DrivesGpio, PwmDriver: DrivesPwm,
     UartDriver: DrivesUart>
MasterController<GpioDriver, PwmDriver, UartDriver> {
    pub fn new(app_cfg: &AppConfig,
               gpio_driver: GpioDriver,
               pwm_driver: PwmDriver,
               uart_driver: UartDriver) -> Self {
        Self {
            gpio_driver,
            pwm_driver,
            uart_driver,

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
        let (position_tx, position_rx) = mpsc::channel();
        let (net_broadcast_tx, net_broadcast_rx) = mpsc::channel();
        let (net_recv_tx, net_recv_rx) = mpsc::channel();

        let nucifera = &self.nucifera_driver;
        let uart_driver = &self.uart_driver;

        // Positioning Task
        let task_position = spawn_task(move || {
            let pos = nucifera.read_current_position(uart_driver);
            position_tx.send(pos).unwrap();
        }, Duration::from_millis(1), "1kHz Task");

        // Network task.
        let task_nettx = thread::spawn(move || {
            let data_to_send = net_broadcast_rx.recv().unwrap();
            // TODO: Send the data over the network.
        });

        let task_netrx = thread::spawn(move || {
            // TODO: Wait until data received, push to channel.
        });

        // Idle task.
        loop { }
    }

    pub fn run(&self) {
        self.init();
        self.spawn_tasks();
    }
}
