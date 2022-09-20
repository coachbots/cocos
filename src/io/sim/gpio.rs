use super::super::interface::{IODriver, IOError};
use super::super::interface::gpio::{DrivesGpio, PullMode};

enum PinDirection {
    Output,
    Input
}

const N_GPIO_PINS: usize = 26;

struct RpiGpioSimulator {
    pin_states: [bool; N_GPIO_PINS],
    pin_directions: [PinDirection; N_GPIO_PINS]
}

impl DrivesGpio for RpiGpioSimulator {
    fn set(&mut self, pin_bcm: u8) -> Result<(), IOError> {
        self.pin_states[pin_bcm as usize] = true;
        Result::Ok(())
    }

    fn clear(&mut self, pin_bcm: u8) -> Result<(), IOError> {
        self.pin_states[pin_bcm as usize] = false;
        Result::Ok(())
    }

    fn set_out(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        self.pin_directions[pin_bcm as usize] = PinDirection::Output;
        Result::Ok(())
    }

    fn set_inp(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        self.pin_directions[pin_bcm as usize] = PinDirection::Input;
        Result::Ok(())
    }
}
