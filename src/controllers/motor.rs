use rppal::gpio::Gpio;

use crate::{
    drivers::{
        led_driver,
        motor_driver::{MotorDescriptor, MotorDirection, MotorDriver, MotorError},
    },
    io::interface::gpio::DrivesGpio,
    models::motor_power::{MotorPower, MotorPowerQuadrant},
};

#[derive(Debug)]
pub enum MotorControllerError {
    IOError,
}

#[derive(Clone, Copy)]
pub struct MotorController {
    left_motor_driver: MotorDriver,
    right_motor_driver: MotorDriver,
}

impl MotorController {
    pub fn new(lmot: MotorDescriptor, rmot: MotorDescriptor) -> Self {
        Self {
            left_motor_driver: MotorDriver::new(lmot),
            right_motor_driver: MotorDriver::new(rmot),
        }
    }

    pub fn block(&self, gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorControllerError> {
        let l = self.left_motor_driver.block(gpio_driver);
        if l.is_err() {
            return Err(MotorControllerError::IOError);
        }

        let r = self.right_motor_driver.block(gpio_driver);
        if r.is_err() {
            return Err(MotorControllerError::IOError);
        }

        Ok(())
    }

    pub fn unblock(&self, gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorControllerError> {
        let l = self.left_motor_driver.unblock(gpio_driver);
        if l.is_err() {
            return Err(MotorControllerError::IOError);
        }

        let r = self.right_motor_driver.unblock(gpio_driver);
        if r.is_err() {
            return Err(MotorControllerError::IOError);
        }

        Ok(())
    }

    pub fn set_vel(
        &self,
        vel: MotorPower,
        gpio_driver: &mut impl DrivesGpio,
    ) -> Result<(), MotorControllerError> {
        if vel.is_locked() {
            return self.block(gpio_driver);
        }

        if let Err(err) = self.unblock(gpio_driver) {
            return Err(err);
        }

        match vel.as_quadrant() {
            MotorPowerQuadrant::PLeftPRight => {
                if let Err(err) = self
                    .left_motor_driver
                    .set_direction(MotorDirection::CounterClockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
                if let Err(err) = self
                    .right_motor_driver
                    .set_direction(MotorDirection::Clockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
            }
            MotorPowerQuadrant::PLeftNRight => {
                if let Err(err) = self
                    .left_motor_driver
                    .set_direction(MotorDirection::CounterClockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
                if let Err(err) = self
                    .right_motor_driver
                    .set_direction(MotorDirection::CounterClockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
            }
            MotorPowerQuadrant::NLeftPRight => {
                if let Err(err) = self
                    .left_motor_driver
                    .set_direction(MotorDirection::Clockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
                if let Err(err) = self
                    .right_motor_driver
                    .set_direction(MotorDirection::Clockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
            }
            MotorPowerQuadrant::NLeftNRight => {
                if let Err(err) = self
                    .left_motor_driver
                    .set_direction(MotorDirection::Clockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
                if let Err(err) = self
                    .right_motor_driver
                    .set_direction(MotorDirection::CounterClockwise, gpio_driver)
                {
                    return Err(MotorControllerError::IOError);
                }
            }
        }

        if let Err(err) = self.left_motor_driver.set_speed(vel.pow_left()) {
            return Err(MotorControllerError::IOError);
        };
        if let Err(err) = self.left_motor_driver.set_speed(vel.pow_right()) {
            return Err(MotorControllerError::IOError);
        };

        Ok(())
    }
}
