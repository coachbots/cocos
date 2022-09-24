use uom::si::f32::Length;
use uom::si::length::meter;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: Length,
    pub y: Length
}

impl Position {
    pub fn zero() -> Self {
        Self {
            x: Length::new::<meter>(0.0),
            y: Length::new::<meter>(0.0)
        }
    }
}
