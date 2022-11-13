use super::{position::Position, motor_power::MotorPower, led_color::LedColor};

/// Represents data that is fed into the python api.
pub struct ApiTickMessage {
    /// The current bot position emitted to the API.
    pub bot_pos: Position
}

/// Represents data that the API controller spews out on a tick basis.
pub struct ApiTickOutputMessage {
    pub request_motor_power: Option<MotorPower>,
    pub request_led_color: Option<LedColor>
}
