use uom::si::f32::Frequency;

pub type LedColor = (f32, f32, f32);

pub trait DrivesLed {
    /// Sets the brightness of the LED.
    ///
    /// # Arguments
    ///
    /// * `value` - Must be between 0 and 1.
    fn set_brightness(&mut self, value: f32);

    fn set_color(&mut self, color: LedColor);

    /// Sets the PWM frequency of the LED.
    ///
    /// Can be used to achieve a pulsating effect if desired.
    fn set_freq(&mut self, freq: Frequency);
}

pub struct LedDriver {
}

impl LedDriver {
    pub fn new() -> Self {
        Self {
        }
    }
}
