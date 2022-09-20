use uom::si::f32::Frequency;

use super::IOError;

pub trait DrivesPwm {
    /// Sets the pwm duty cycle.
    ///
    /// # Arguments
    ///
    /// * `value` - Must be between 0 and 1.
    fn set_dc(&self, value: f32) -> Result<(), IOError>;

    /// Sets the pwm frequency.
    fn set_freq(&self, frequency: Frequency) -> Result<(), IOError>;

    /// Starts PWM emission.
    fn start(&self) -> Result<(), IOError>;

    /// Stops PWM emission.
    ///
    /// The implementation is free to decide whether the PWM should be pulled
    /// low or left floating.
    fn stop(&self) -> Result<(), IOError>;
}
