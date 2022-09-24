#[derive(Clone, Copy)]
pub struct LedColor {
    r: f32,
    g: f32,
    b: f32
}

impl LedColor {
    pub fn new(r: f32, g: f32, b: f32) -> Result<Self, &'static str> {
        if !(0.0 <= r && r <= 1.0 &&
             0.0 <= g && g <= 1.0 &&
             0.0 <= b && b <= 1.0) {
            return Err("Attmepting to construct LED color with invalid args.");
        }

        Ok(Self {
            r,
            g,
            b
        })
    }

    pub fn off() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }
}
