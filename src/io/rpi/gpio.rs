use super::super::interface::gpio::{DrivesGpio, PullMode, GpioError};
use rppal::gpio::{Gpio, Error};

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
               pull_mode: PullMode) -> Result<(), GpioError> {
        let pin = self.rpi_driver.get(pin_bcm);
        if pin.is_err() && matches!(pin.err().unwrap(), Error::Io(pin)) {
            return Err(GpioError::IO);
        }

        let p = self.rpi_driver.get(pin_bcm).expect(
            "Could not retrieve pin {}");
        match pull_mode {
            PullMode::Up => { p.into_input_pullup(); }
            PullMode::Down => { p.into_input_pulldown(); }
            PullMode::Floating => { p.into_input(); }
        }
        Ok(())
    }

    fn set_out(&self, pin_bcm: u8,
               pull_mode: PullMode) -> Result<(), GpioError> {
        let p = self.rpi_driver.get(pin_bcm).expect(
            "Could not retrieve pin {}");
        match pull_mode {
            PullMode::Up => { p.into_output_low(); }
            PullMode::Down => { p.into_output_high(); }
            PullMode::Floating => { p.into_output(); }
        }
        Ok(())
    }

    fn set(&self, pin_bcm: u8) -> Result<(), GpioError> {
        let pin = self.rpi_driver.get(pin_bcm);
        if pin.is_err() && matches!(pin.err().unwrap(), Error::Io(pin)) {
            return Err(GpioError::IO);
        }

        let p = self.rpi_driver.get(pin_bcm).expect(
            "Could not retrieve pin {}");
        p.into_output().set_high();
        Ok(())
    }

    fn clear(&self, pin_bcm: u8) -> Result<(), GpioError> {
        let pin = self.rpi_driver.get(pin_bcm);
        if pin.is_err() && matches!(pin.err().unwrap(), Error::Io(pin)) {
            return Err(GpioError::IO);
        }

        let p = self.rpi_driver.get(pin_bcm).expect(
            "Could not retrieve pin {}");
        p.into_output().set_low();
        Ok(())
    }
}
