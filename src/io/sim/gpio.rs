use super::super::interface::IOError;
use super::super::interface::gpio::{DrivesGpio, PullMode};

enum PinDirection {
    Output,
    Input
}

const N_GPIO_PINS: usize = 26;

pub struct RpiGpioSimulator {
}

impl DrivesGpio for RpiGpioSimulator {
    fn set(&self, pin_bcm: u8) -> Result<(), IOError> {
        Result::Ok(())
    }

    fn clear(&self, pin_bcm: u8) -> Result<(), IOError> {
        Result::Ok(())
    }

    fn set_out(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        Result::Ok(())
    }

    fn set_inp(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        Result::Ok(())
    }
}
