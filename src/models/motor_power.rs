#[derive(Clone, Copy)]
pub struct MotorPower {
    pub left: f32,
    pub right: f32
}

impl MotorPower {
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0
        }
    }
}
