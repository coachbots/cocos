use crate::models::{
    position::Position,
    app_state::AppState,
    motor_power::MotorPower,
    led_color::LedColor
};
use super::interface::HandlesPyApi;

pub struct ApiController<'a> {
    app_state: &'a AppState
}

impl<'a> ApiController<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            app_state
        }
    }
}

impl<'a> HandlesPyApi for ApiController<'a> {
    fn on_get_pos(&mut self) -> Position {
        let app_state = self.app_state;
        let pos = app_state.position.lock_ref();
        *pos
    }

    fn on_set_rel_speed(&mut self, rel_speed: MotorPower) {
        let mut pow = self.app_state.target_motor_power.lock_mut();
        *pow = rel_speed;
    }

    fn on_set_led_color(&mut self, led_color: LedColor) {
        let mut led = self.app_state.target_led.lock_mut();
        *led = led_color;
    }
}
