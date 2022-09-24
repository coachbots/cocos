use crate::models::{
    position::Position,
    motor_power::MotorPower,
    led_color::LedColor
};

pub trait HandlesTick1Ms { fn on_tick1(&mut self); }
pub trait HandlesTick10Ms { fn on_tick10(&mut self); }
pub trait HandlesTick100Ms { fn on_tick100(&mut self); }
pub trait HandlesTick1000Ms { fn on_tick1000(&mut self); }

pub trait HandlesPyApi {
    /// Called by the python API to set the current speed.
    ///
    /// # Arguments:
    ///
    /// * `rel_speed` - The relative speed of the two motors. Values must be
    ///                 between -1 and 1.
    fn on_set_rel_speed(&mut self, rel_speed: MotorPower);

    /// Called by the python API to set the current LED color.
    ///
    /// # Arguments:
    ///
    /// * `led_color` - The color of the LED. Values must fit 0-1.
    fn on_set_led_color(&mut self, led_color: LedColor);

    /// Called by the python API to retrieve the current bot position.
    fn on_get_pos(&mut self) -> Position;
}
