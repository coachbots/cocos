use uom::si::f32::Frequency;

use crate::{io::interface::pwm::DrivesPwm, models::led_color::LedColor};

#[derive(Clone, Copy)]
/// Defines and describes a single LED.
pub struct LedDescriptor {
    /// The red Rpi BCM Pin
    pub pin_r_bcm: u8,
    /// The green Rpi BCM Pin
    pub pin_g_bcm: u8,
    /// The blue Rpi BCM Pin
    pub pin_b_bcm: u8,
    /// The target frequency to operate at.
    ///
    /// Anything above 120Hz should be fine.
    pub frequency: Frequency
}

/// Thrown upon an LED control error.
pub enum LedError {
    /// Thrown when the IO layer cannot modify the state of a pin.
    IO,
}

#[derive(Clone, Copy)]
/// Represents a singular LED driver that controls a specific LED.
pub struct LedDriver {
    descriptor: LedDescriptor
}

impl LedDriver {
    pub fn new(descriptor: LedDescriptor) -> Self { Self { descriptor } }

    /// Sets the LED color.
    ///
    /// Arguments:
    /// * `color` - The color to set the LED to.
    /// * `pwm_driver` - A PWM driver to use.
    pub fn set_color(&self, color: LedColor,
                     pwm_driver: &mut impl DrivesPwm) -> Result<(), LedError> {
        if let Err(err) = pwm_driver.set_freq_dc(self.descriptor.frequency, color.r,
                                                 self.descriptor.pin_r_bcm) {
            return Err(LedError::IO);
        }
        if let Err(err) = pwm_driver.set_freq_dc(self.descriptor.frequency, color.g,
                                                 self.descriptor.pin_g_bcm) {
            return Err(LedError::IO);
        }
        if let Err(err) = pwm_driver.set_freq_dc(self.descriptor.frequency, color.b,
                                                 self.descriptor.pin_b_bcm) {
            return Err(LedError::IO);
        }

        Ok(())
    }
}
