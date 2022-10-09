use super::super::interface::{IODriver, IOError};
use super::super::interface::gpio::{DrivesGpio, PullMode};
use rppal::gpio::Gpio;

pub struct RpiGpioDriver {
    rpi_driver: Gpio,
}

impl RpiGpioDriver {
    pub fn new() -> Self {
        Self {
            rpi_driver: Gpio::new().unwrap()
        }
    }
}

impl DrivesGpio for RpiGpioDriver {
    fn set_inp(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {

        match self.rpi_driver.get(pin_bcm) {
            Ok(p) => {
                match pull_mode {
                    PullMode::Up => { p.into_input_pullup(); }
                    PullMode::Down => { p.into_input_pulldown(); }
                    PullMode::Floating => { p.into_input(); }
                }
                Result::Ok(())
            }
            Err(err) => {
                Err(IOError::Unknown) // TODO: Better handling
            }
        }
    }

    fn set_out(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {

        match self.rpi_driver.get(pin_bcm) {
            Ok(p) => {
                match pull_mode {
                    PullMode::Up => { p.into_output_low(); }
                    PullMode::Down => { p.into_output_high(); }
                    PullMode::Floating => { p.into_output(); }
                }
                Result::Ok(())
            }
            Err(err) => {
                Err(IOError::Unknown) // TODO: Better handling
            }
        }
    }

    fn set(&self, pin_bcm: u8) -> Result<(), IOError> {
        match self.rpi_driver.get(pin_bcm) {
            Ok(p) => {
                p.into_output().set_high();
                Result::Ok(())
            }
            Err(err) => {
                Err(IOError::Unknown) // TODO: Better handling
            }
        }
    }

    fn clear(&self, pin_bcm: u8) -> Result<(), IOError> {
        match self.rpi_driver.get(pin_bcm) {
            Ok(p) => {
                p.into_output().set_low();
                Result::Ok(())
            }
            Err(err) => {
                Err(IOError::Unknown) // TODO: Better handling
            }
        }
    }
}
