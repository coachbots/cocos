#[derive(Clone, Copy)]
pub struct MotorPower {
    left: f32,
    right: f32
}

pub enum MotorPowerQuadrant {
    PLeftPRight,
    PLeftNRight,
    NLeftPRight,
    NLeftNRight
}

impl MotorPower {
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0
        }
    }

    pub fn as_quadrant(&self) -> MotorPowerQuadrant {
        if self.left >= 0.0 && self.right >= 0.0 {
            return MotorPowerQuadrant::PLeftPRight;
        }

        if self.left >= 0.0 && self.right < 0.0 {
            return MotorPowerQuadrant::PLeftNRight;
        }

        if self.left < 0.0 && self.right >= 0.0 {
            return MotorPowerQuadrant::NLeftPRight;
        }

        return MotorPowerQuadrant::NLeftNRight;
    }

    pub fn pow_left(&self) -> f32 { self.left.abs() }
    pub fn pow_right(&self) -> f32 { self.right.abs() }
}
