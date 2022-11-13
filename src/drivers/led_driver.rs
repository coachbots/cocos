use uom::si::f32::Frequency;

use crate::{models::led_color::LedColor, io::interface::pwm::DrivesPwm};

#[derive(Clone, Copy)]
pub struct LedDescriptor {
    pub pin_r_bcm: u8,
    pub pin_g_bcm: u8,
    pub pin_b_bcm: u8,
    pub frequency: Frequency
}

pub enum LedError {
    IO
}

#[derive(Clone, Copy)]
pub struct LedDriver {
    descriptor: LedDescriptor
}

impl LedDriver {
    pub fn new(descriptor: LedDescriptor) -> Self {
        Self {
            descriptor
        }
    }

    /// Sets the LED color.
    pub fn set_color(&self, color: LedColor,
                     pwm_driver: &mut impl DrivesPwm) -> Result<(), LedError> {
        if let Err(err) = pwm_driver.set_freq_dc(
            self.descriptor.frequency, color.r, self.descriptor.pin_r_bcm) {
            return Err(LedError::IO);
        }
        if let Err(err) = pwm_driver.set_freq_dc(
            self.descriptor.frequency, color.g, self.descriptor.pin_g_bcm) {
            return Err(LedError::IO);
        }
        if let Err(err) = pwm_driver.set_freq_dc(
            self.descriptor.frequency, color.b, self.descriptor.pin_b_bcm) {
            return Err(LedError::IO);
        }

        Ok(())
    }
}
