use crate::io::interface::{gpio::{DrivesGpio, PullMode}, IOError};

pub enum MotorDirection {
    Clockwise,
    CounterClockwise
}

#[derive(Clone, Copy)]
pub struct MotorDescriptor {
    pub pin_left_bcm: u8,
    pub pin_right_bcm: u8
}

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
    fn unblock(&self, gpio_driver: &impl DrivesGpio) {
        gpio_driver.clear(self.descriptor.pin_left_bcm);
        gpio_driver.clear(self.descriptor.pin_right_bcm);
    }

    /// Blocks the motor.
    ///
    /// This function must immediately halt the motor and block it from further
    /// operation.
    fn block(&self, gpio_driver: &impl DrivesGpio) {
        gpio_driver.set(self.descriptor.pin_left_bcm);
        gpio_driver.set(self.descriptor.pin_right_bcm);
    }

    /// Sets the relative speed of the motor.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage speed of the motor. Value must be between
    ///               0 and 1.
    pub fn set_speed(&self, percent: f32) {
        // TODO: Implement with PWM driver
    }

    /// Sets the direction of the motor.
    ///
    /// # Arguments
    ///
    /// * `direction` - The motor direction.
    pub fn set_direction(&self, direction: MotorDirection,
                         gpio_driver: &impl DrivesGpio) {
        match direction {
            MotorDirection::CounterClockwise => {
                gpio_driver.set(self.descriptor.pin_left_bcm);
                gpio_driver.clear(self.descriptor.pin_right_bcm);
            }
            MotorDirection::Clockwise => {
                gpio_driver.set(self.descriptor.pin_right_bcm);
                gpio_driver.clear(self.descriptor.pin_left_bcm);
            }
        }
    }

    pub fn init(&self, gpio_driver: &impl DrivesGpio) -> Result<(), IOError> {
        let l_out = gpio_driver.set_out(self.descriptor.pin_left_bcm,
                                        PullMode::Down);
        if l_out.is_err() { return l_out; }

        let r_out = gpio_driver.set_out(self.descriptor.pin_right_bcm,
                                        PullMode::Down);
        if r_out.is_err() { return r_out; }

        Ok(())
    }
}
