use std::fmt::Display;

use uom::si::f32::{Length, Angle};
use uom::si::length::meter;
use uom::si::angle::radian;
use uom::fmt::DisplayStyle::Abbreviation;

#[derive(Clone, Copy)]
/// This sturcture defines a position which encodes the XY position of a
/// coachbot, as well as the angle it is facing in 2 dimensions.
///
/// Note that this structure uses [uom] to encode these values regardless of
/// their units.
pub struct Position {
    /// The x position of the coachbot.
    pub x: Length,
    /// The y position of the coachbot.
    pub y: Length,
    /// The angle in standard orientation (ie. CCW from X axis).
    pub theta: Angle
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]",
               self.x.into_format_args(meter, Abbreviation),
               self.y.into_format_args(meter, Abbreviation))
    }
}

impl Position {
    /// Creates a zeroed out position.
    pub fn zero() -> Self {
        Self {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0),
            theta: Angle::new::<radian>(0.0)
        }
    }
}
