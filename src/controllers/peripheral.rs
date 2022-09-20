use uom::si::f32::Length;
use uom::si::length::meter;

use crate::config::AppConfig;
use crate::drivers::led_driver::LedDriver;
use crate::drivers::motor_driver::MotorDriver;
use crate::io::interface::ProvidesIO;

use super::position::Position;

pub struct PeripheralController {
    left_motor_driver: MotorDriver,
    right_motor_driver: MotorDriver,
    led_driver: LedDriver,
}

impl PeripheralController {
    pub fn new(app_cfg: AppConfig, io_driver: Box<dyn ProvidesIO>) -> Self {
        Self {
            left_motor_driver: MotorDriver::new(app_cfg.mot_left,
                                                io_driver.get_gpio()),
            right_motor_driver: MotorDriver::new(app_cfg.mot_right,
                                                 io_driver.get_gpio()),
            led_driver: LedDriver::new()
        }
    }

    pub fn get_position(&self) -> Position {
        // TODO: Actually implement
        (Length::new::<meter>(0.0), Length::new::<meter>(0.0))
    }
}
