use futures_signals::signal::Mutable;

use super::{led_color::LedColor, motor_power::MotorPower, position::Position};

pub struct AppState {
    pub position: Mutable<Position>,

    pub target_motor_power: Mutable<MotorPower>,
    pub target_led: Mutable<LedColor>,
}

impl AppState {
    pub fn zero() -> Self {
        Self {
            position: Mutable::new(Position::zero()),
            target_motor_power: Mutable::new(MotorPower::zero()),
            target_led: Mutable::new(LedColor::off()),
        }
    }
}
