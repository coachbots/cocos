use crate::io::interface::{gpio::{DrivesGpio, PullMode}, IOError};

pub enum MotorDirection {
    Clockwise,
    CounterClockwise
}

pub enum MotorError {
    IOError
}

#[derive(Clone, Copy)]
pub struct MotorDescriptor {
    pub pin_left_bcm: u8,
    pub pin_right_bcm: u8
}

#[derive(Clone, Copy)]
/// Represents a single motor controller. This driver can control any arbitrary
/// motors connected via any GPIO and PWM pins.
pub struct MotorDriver {
    descriptor: MotorDescriptor,
}

impl MotorDriver {
    pub fn new(descriptor: MotorDescriptor) -> Self {
        Self {
            descriptor,
        }
    }

    /// Unblocks the motor.
    ///
    /// It is left to the implementation to decide whether the motor will be
    /// halted upon this function call.
    pub fn unblock(&self,
                   gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        let l_out = gpio_driver.clear(self.descriptor.pin_left_bcm);
        if l_out.is_err() { return Err(MotorError::IOError); }

        let r_out = gpio_driver.clear(self.descriptor.pin_right_bcm);
        if r_out.is_err() { return Err(MotorError::IOError); }

        Ok(())
    }

    /// Blocks the motor.
    ///
    /// This function must immediately halt the motor and block it from further
    /// operation.
    pub fn block(&self,
                 gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        let l_out = gpio_driver.set(self.descriptor.pin_left_bcm);
        if l_out.is_err() { return Err(MotorError::IOError); }

        let r_out = gpio_driver.set(self.descriptor.pin_right_bcm);
        if r_out.is_err() { return Err(MotorError::IOError); }

        Ok(())
    }

    /// Sets the relative speed of the motor.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage speed of the motor. Value must be between
    ///               0 and 1.
    pub fn set_speed(&self, percent: f32) -> Result<(), MotorError> {
        // TODO: Implement with PWM driver
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
        gpio_driver: &mut impl DrivesGpio
    ) -> Result<(), MotorError> {
        match direction {
            MotorDirection::CounterClockwise => {
                let l_out = gpio_driver.set(self.descriptor.pin_left_bcm);
                if l_out.is_err() { return Err(MotorError::IOError); }

                let r_out = gpio_driver.clear(self.descriptor.pin_right_bcm);
                if r_out.is_err() { return Err(MotorError::IOError); }
            }
            MotorDirection::Clockwise => {
                let l_out = gpio_driver.set(self.descriptor.pin_right_bcm);
                if l_out.is_err() { return Err(MotorError::IOError); }

                let r_out = gpio_driver.clear(self.descriptor.pin_left_bcm);
                if r_out.is_err() { return Err(MotorError::IOError); }
            }
        }
        Ok(())
    }

    pub fn init(&self,
                gpio_driver: &mut impl DrivesGpio) -> Result<(), MotorError> {
        let l_out = gpio_driver.set_out(self.descriptor.pin_left_bcm,
                                        PullMode::Down);
        if l_out.is_err() { return Err(MotorError::IOError); }

        let r_out = gpio_driver.set_out(self.descriptor.pin_right_bcm,
                                        PullMode::Down);
        if r_out.is_err() { return Err(MotorError::IOError); }

        Ok(())
    }
}
