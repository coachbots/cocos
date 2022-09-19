use crate::io::interface::gpio::{DrivesGpio, PullMode};

pub enum MotorDirection {
    Clockwise,
    CounterClockwise
}

pub trait DrivesMotor {
    /// Unblocks the motor.
    ///
    /// It is left to the implementation to decide whether the motor will be
    /// halted upon this function call.
    fn unblock(&mut self);

    /// Blocks the motor.
    ///
    /// This function must immediately halt the motor and block it from further
    /// operation.
    fn block(&mut self);

    /// Sets the relative speed of the motor.
    ///
    /// # Arguments
    ///
    /// * `percent` - The percentage speed of the motor. Value must be between
    ///               0 and 1.
    fn set_speed(&mut self, percent: f32);

    /// Sets the direction of the motor.
    ///
    /// # Arguments
    ///
    /// * `direction` - The motor direction.
    fn set_direction(&mut self, direction: MotorDirection);
}

pub struct MotorDescriptor {
    pub pin_left_bcm: u8,
    pub pin_right_bcm: u8
}

pub struct MotorDriver {
    descriptor: MotorDescriptor,
    gpio_driver: Box<dyn DrivesGpio>
}

impl MotorDriver {
    pub fn new(descriptor: MotorDescriptor,
               gpio_driver: Box<dyn DrivesGpio>) -> Self {
        Self {
            descriptor,
            gpio_driver
        }
    }

    fn init(&mut self) {
        self.gpio_driver.set_out(self.descriptor.pin_left_bcm, PullMode::Down);
        self.gpio_driver.set_out(self.descriptor.pin_right_bcm,
                                 PullMode::Down);
    }
}

impl DrivesMotor for MotorDriver {
    fn unblock(&mut self) {
        self.gpio_driver.clear(self.descriptor.pin_left_bcm);
        self.gpio_driver.clear(self.descriptor.pin_right_bcm);
    }

    fn block(&mut self) {
        self.gpio_driver.set(self.descriptor.pin_left_bcm);
        self.gpio_driver.set(self.descriptor.pin_right_bcm);
    }

    fn set_speed(&mut self, percent: f32) {
        // TODO: Implement with PWM driver
    }

    fn set_direction(&mut self, direction: MotorDirection) {
        match direction {
            MotorDirection::CounterClockwise => {
                self.gpio_driver.set(self.descriptor.pin_left_bcm);
                self.gpio_driver.clear(self.descriptor.pin_right_bcm);
            }
            MotorDirection::Clockwise => {
                self.gpio_driver.set(self.descriptor.pin_right_bcm);
                self.gpio_driver.clear(self.descriptor.pin_left_bcm);
            }
        }
    }
}
