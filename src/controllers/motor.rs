use rppal::gpio::Gpio;

use crate::{drivers::motor_driver::{MotorDriver, MotorDescriptor}, models::motor_power::{MotorPower, MotorPowerQuadrant}, io::interface::gpio::DrivesGpio};

#[derive(Debug)]
pub enum MotorControllerError {
    IOError
}

pub struct MotorController {
    left_motor_driver: MotorDriver,
    right_motor_driver: MotorDriver,
}

impl MotorController {
    pub fn new(lmot: MotorDescriptor, rmot: MotorDescriptor) -> Self {
        Self {
            left_motor_driver: MotorDriver::new(lmot),
            right_motor_driver: MotorDriver::new(rmot)
        }
    }

    pub fn block(
        &self,
        gpio_driver: &impl DrivesGpio
    ) -> Result<(), MotorControllerError> {
        let l = self.left_motor_driver.block(gpio_driver);
        if l.is_err() { return Err(MotorControllerError::IOError); }

        let r = self.right_motor_driver.block(gpio_driver);
        if r.is_err() { return Err(MotorControllerError::IOError); }

        Ok(())
    }

    pub fn set_vel(&self, vel: MotorPower) {
        match vel.as_quadrant() {
            MotorPowerQuadrant::PLeftPRight => {
            }
            MotorPowerQuadrant::PLeftNRight => {
            }
            MotorPowerQuadrant::NLeftPRight => {
            }
            MotorPowerQuadrant::NLeftNRight => {
            }
        }
    }
}
