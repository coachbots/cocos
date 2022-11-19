use super::super::interface::gpio::{DrivesGpio, GpioError, PullMode};
use rppal::gpio::Gpio;

pub struct RpiGpioDriver {
    rpi_driver: Gpio,
}

/// Defines a driver that interfaces with the Raspberry Pi GPIO Pins on Linux
impl RpiGpioDriver {
    pub fn new() -> Self { Self { rpi_driver: Gpio::new().unwrap(), } }
}

impl DrivesGpio for RpiGpioDriver {
    fn set_inp(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError> {
        let p = self.rpi_driver.get(pin_bcm).expect("Could not retrieve pin");
        match pull_mode {
            PullMode::Up => { p.into_input_pullup().set_reset_on_drop(false); }
            PullMode::Down => { p.into_input_pulldown().set_reset_on_drop(false); }
        }
        Ok(())
    }

    fn set_out(&mut self, pin_bcm: u8, pull_mode: PullMode) -> Result<(), GpioError> {
        let p = self.rpi_driver.get(pin_bcm).expect("Could not retrieve pin");
        match pull_mode {
            PullMode::Up => { p.into_output_low().set_reset_on_drop(false); }
            PullMode::Down => { p.into_output_high().set_reset_on_drop(false); }
        }
        Ok(())
    }

    fn set(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        let p = self.rpi_driver.get(pin_bcm).expect("Could not retrieve pin");
        let mut output_pin = p.into_output();
        output_pin.set_reset_on_drop(false); // TODO: Do we really need to set this flag all the
                                             // time?
        output_pin.set_high();
        Ok(())
    }

    fn clear(&mut self, pin_bcm: u8) -> Result<(), GpioError> {
        let p = self.rpi_driver.get(pin_bcm).expect("Could not retrieve pin");
        let mut output_pin = p.into_output();
        output_pin.set_reset_on_drop(false); // TODO: Do we really need to set this flag all the
                                             // time?
        output_pin.set_low();
        Ok(())
    }
}
