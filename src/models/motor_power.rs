use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
/// Represents a motor power that can be delivered to the wheels.
///
/// TODO: This should be renamed to be WheelsPower or something along the lines
/// since the contrary implies that this has something to do with MotorDriver
pub struct MotorPower {
    /// The left motor power (-1 - +1).
    left: f32,
    /// The right motor power (-1 - +1)
    right: f32,
    /// Whether the motors are currently locked. It is not possible to lock
    /// motors individually.
    locked: bool,
}

/// An enumeration of possible motor power quadrants.
pub enum MotorPowerQuadrant {
    PLeftPRight,
    PLeftNRight,
    NLeftPRight,
    NLeftNRight,
}

impl MotorPower {
    /// Creates a zeroed-out, unlocked MotorPower.
    pub fn zero() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            locked: false,
        }
    }

    /// Creates a new MotorPower object.
    ///
    /// If the passed parameters are invalid, this returns a None
    pub fn new(left: f32, right: f32, locked: bool) -> Option<MotorPower> {
        if !(-1f32 <= left && left <= 1f32 && -1f32 <= right && right <= 1f32) {
            return None;
        }

        Some(MotorPower {
            left,
            right,
            locked,
        })
    }

    /// Returns which quadrant the motor power lies in. Imagining it as a
    /// vector (left, right), this function will return which motor is positive
    /// which is negative.
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

    /// Returns the absolute motor power of the left motor.
    pub fn pow_left(&self) -> f32 {
        self.left.abs()
    }

    /// Returns the absolute motor power of the right motor.
    pub fn pow_right(&self) -> f32 {
        self.right.abs()
    }

    /// Returns whether the motor is currently locked or not.
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

impl Display for MotorPower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lock_str = if self.is_locked() { "L" } else { "F" };
        write!(f, "[{}, {}, {}]", self.left, self.right, lock_str)
    }
}
