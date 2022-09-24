use super::super::interface::{IODriver, IOError};
use super::super::interface::gpio::{DrivesGpio, PullMode};
use rppal::gpio::Gpio;

pub struct RpiGpioDriver {
    rpi_driver: Gpio,
    initialized: bool
}

impl RpiGpioDriver {
    pub fn new() -> Self {
        let rpi_driver = Gpio::new();
        match rpi_driver {
            Ok(v) => Self {
                rpi_driver: v,
                initialized: false
            },
            Err(_err) => panic!("Cannot initialize RPI Gpio Driver.")
        }
    }
}

impl IODriver for RpiGpioDriver {
    fn init(&mut self) -> Result<(), IOError> {
        if self.initialized {
            return Err(IOError::Reinitialization);
        }

        self.initialized = true;
        Result::Ok(())
    }
}

impl DrivesGpio for RpiGpioDriver {
    fn set_inp(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        if !self.initialized {
            return Err(IOError::Uninitialized);
        }

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

    fn set_out(&mut self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), IOError> {
        if !self.initialized {
            return Err(IOError::Uninitialized);
        }

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

    fn set(&mut self, pin_bcm: u8) -> Result<(), IOError> {
        if !self.initialized {
            return Err(IOError::Uninitialized);
        }

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

    fn clear(&mut self, pin_bcm: u8) -> Result<(), IOError> {
        if !self.initialized {
            return Err(IOError::Uninitialized);
        }

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
