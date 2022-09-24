use std::{rc::Rc, cell::RefCell};

use crate::io::interface::{gpio::{DrivesGpio, PullMode}, IODriver, IOError};

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

#[derive(Clone, Copy)]
pub struct MotorDescriptor {
    pub pin_left_bcm: u8,
    pub pin_right_bcm: u8
}

pub struct MotorDriver {
    descriptor: MotorDescriptor,
    gpio_driver: Rc<RefCell<dyn DrivesGpio>>
}

impl MotorDriver {
    pub fn new(descriptor: MotorDescriptor,
               gpio_driver: Rc<RefCell<dyn DrivesGpio>>) -> Self {
        Self {
            descriptor,
            gpio_driver
        }
    }
}

impl IODriver for MotorDriver {
    fn init(&mut self) -> Result<(), IOError> {
        let mut gpio_driver = self.gpio_driver.borrow_mut();
        let l_out = gpio_driver.set_out(self.descriptor.pin_left_bcm,
                                        PullMode::Down);
        if l_out.is_err() { return l_out; }

        let r_out = gpio_driver.set_out(self.descriptor.pin_right_bcm,
                                        PullMode::Down);
        if r_out.is_err() { return r_out; }

        Ok(())
    }
}

impl DrivesMotor for MotorDriver {
    fn unblock(&mut self) {
        let mut gpio_driver = self.gpio_driver.borrow_mut();
        gpio_driver.clear(self.descriptor.pin_left_bcm);
        gpio_driver.clear(self.descriptor.pin_right_bcm);
    }

    fn block(&mut self) {
        let mut gpio_driver = self.gpio_driver.borrow_mut();
        gpio_driver.set(self.descriptor.pin_left_bcm);
        gpio_driver.set(self.descriptor.pin_right_bcm);
    }

    fn set_speed(&mut self, percent: f32) {
        // TODO: Implement with PWM driver
    }

    fn set_direction(&mut self, direction: MotorDirection) {
        let mut gpio_driver = self.gpio_driver.borrow_mut();
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
}
