use uom::si::f32::Frequency;

pub enum PwmError {
}

pub trait DrivesPwm {
    /// Sets the pwm duty cycle.
    ///
    /// # Arguments
    ///
    /// * `value` - Must be between 0 and 1.
    fn set_dc(&self, value: f32) -> Result<(), PwmError>;

    /// Sets the pwm frequency.
    fn set_freq(&self, frequency: Frequency) -> Result<(), PwmError>;

    /// Starts PWM emission.
    fn start(&self) -> Result<(), PwmError>;

    /// Stops PWM emission.
    ///
    /// The implementation is free to decide whether the PWM should be pulled
    /// low or left floating.
    fn stop(&self) -> Result<(), PwmError>;
}
