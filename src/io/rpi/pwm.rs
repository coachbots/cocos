use rppal::gpio::{self, Gpio};
use uom::si::f32::Frequency;
use crate::io::interface::pwm::{DrivesPwm, PwmError};

pub struct RpiPwmDriver {
    rpi_driver: Gpio,
}

impl RpiPwmDriver {
    pub fn new() -> Self {
        Self {
            rpi_driver: Gpio::new().unwrap()
        }
    }
}

impl DrivesPwm for RpiPwmDriver {
    fn set_freq_dc(&mut self, frequency: Frequency, duty_cycle: f32,
                   pin_bcm: u8) -> Result<(), PwmError> {
        match self.rpi_driver.get(pin_bcm) {
            Err(err) => { return Err(PwmError::IO); }
            Ok(pin) => {
                let freq_hz = frequency.value;
                if let Err(err) = pin.into_output().set_pwm_frequency(
                        freq_hz as f64, duty_cycle as f64) {
                    return Err(PwmError::IO);
                }
            }
        }
        Ok(())
    }
}
