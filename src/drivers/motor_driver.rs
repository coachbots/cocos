use uom::si::{f32::Frequency, frequency::hertz};

use crate::io::interface::{
    gpio::{DrivesGpio, PullMode},
    IOError, pwm::DrivesPwm,
};

pub enum MotorDirection {
    Clockwise,
    CounterClockwise,
}

pub enum MotorError {
    IOError,
}

#[derive(Clone, Copy)]
pub struct MotorDescriptor {
    pub pin_in1: u8,
    pub pin_in2: u8,
    pub pin_pwm: u8,
    pub pin_stdby: u8,
}

#[derive(Clone, Copy)]
/// Represents a single motor controller. This driver can control any arbitrary
/// motors connected via any GPIO and PWM pins.
pub struct MotorDriver {
    descriptor: MotorDescriptor,
}

impl MotorDriver {
    pub fn new(descriptor: MotorDescriptor) -> Self {
        Self { descriptor }
    }

    /// Unblocks the motor.
    ///
    /// It is left to the implementation to decide whether the motor will be
    /// halted upon this function call.
    pub fn unblock(&self, gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        if gpio_driver.clear(self.descriptor.pin_in1).is_err() {
            return Err(MotorError::IOError);
        }

        if gpio_driver.clear(self.descriptor.pin_in2).is_err() {
            return Err(MotorError::IOError);
        }

        // TODO: Figure out if this should clear or set.
        if gpio_driver.clear(self.descriptor.pin_stdby).is_err() {
            return Err(MotorError::IOError);
        }

        Ok(())
    }

    /// Blocks the motor.
    ///
    /// This function must immediately halt the motor and block it from further
    /// operation.
    pub fn block(&self, gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        if gpio_driver.set(self.descriptor.pin_in1).is_err() {
            return Err(MotorError::IOError);
        }

        if gpio_driver.set(self.descriptor.pin_in2).is_err() {
            return Err(MotorError::IOError);
        }

        // TODO: Set or clear?
        if gpio_driver.set(self.descriptor.pin_stdby).is_err() {
            return Err(MotorError::IOError);
        }

        Ok(())
    }

    /// Sets the relative speed of the motor.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage speed of the motor. Value must be between
    ///               0 and 1.
    pub fn set_speed(&self, percent: f32,
                     pwm_driver: &mut impl DrivesPwm) -> Result<(), MotorError> {
        pwm_driver.set_freq_dc(Frequency::new::<hertz>(600f32),
                               percent, self.descriptor.pin_pwm);
        Ok(())
    }

    /// Sets the direction of the motor.
    ///
    /// # Arguments
    ///
    /// * `direction` - The motor direction.
    pub fn set_direction(
        &self,
        direction: MotorDirection,
        gpio_driver: &mut impl DrivesGpio,
    ) -> Result<(), MotorError> {
        match direction {
            MotorDirection::CounterClockwise => {
                if gpio_driver.set(self.descriptor.pin_in1).is_err() {
                    return Err(MotorError::IOError);
                }

                if gpio_driver.clear(self.descriptor.pin_in2).is_err() {
                    return Err(MotorError::IOError);
                }
            }
            MotorDirection::Clockwise => {
                if gpio_driver.set(self.descriptor.pin_in1).is_err() {
                    return Err(MotorError::IOError);
                }

                if gpio_driver.clear(self.descriptor.pin_in2).is_err() {
                    return Err(MotorError::IOError);
                }
            }
        }
        Ok(())
    }

    pub fn init(&self, gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        if gpio_driver.set_out(self.descriptor.pin_in1, PullMode::Down).is_err()
            || gpio_driver.set_out(self.descriptor.pin_in2, PullMode::Down).is_err()
            || gpio_driver.set_out(self.descriptor.pin_pwm, PullMode::Down).is_err()
            || gpio_driver.set_out(self.descriptor.pin_stdby, PullMode::Down).is_err()
            || gpio_driver.clear(self.descriptor.pin_in1).is_err()
            || gpio_driver.clear(self.descriptor.pin_in2).is_err()
            || gpio_driver.clear(self.descriptor.pin_pwm).is_err()
            || gpio_driver.clear(self.descriptor.pin_stdby).is_err()
        {
            return Err(MotorError::IOError);
        }

        Ok(())
    }
}
