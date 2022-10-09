use crate::drivers::motor_driver::{MotorDriver, MotorDescriptor};

pub enum MotorControllerError {
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
}
