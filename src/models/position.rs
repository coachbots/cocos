use std::fmt::Display;

use uom::si::f32::Length;
use uom::si::length::meter;
use uom::fmt::DisplayStyle::Abbreviation;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: Length,
    pub y: Length
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]",
               self.x.into_format_args(meter, Abbreviation),
               self.y.into_format_args(meter, Abbreviation))
    }
}

impl Position {
    pub fn zero() -> Self {
        Self {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0)
        }
    }
}
