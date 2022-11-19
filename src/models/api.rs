use super::{led_color::LedColor, motor_power::MotorPower, position::Position};

#[derive(Debug)]
/// Represents data that is fed into the python api.
pub struct ApiTickInputMessage {
    /// The current bot position emitted to the API.
    pub bot_pos: Position,
}

#[derive(Debug)]
/// Represents data that the API controller spews out on a tick basis.
pub struct ApiTickOutputMessage {
    pub request_motor_power: Option<MotorPower>,
    pub request_led_color: Option<LedColor>,
}

impl ApiTickOutputMessage {
    pub fn motor(pow: MotorPower) -> Self {
        Self {
            request_led_color: None,
            request_motor_power: Some(pow),
        }
    }

    pub fn led(color: LedColor) -> Self {
        Self {
            request_led_color: Some(color),
            request_motor_power: None,
        }
    }

    pub fn none() -> Self {
        Self {
            request_led_color: None,
            request_motor_power: None,
        }
    }
}
