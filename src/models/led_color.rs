#[derive(Clone, Copy)]
pub struct LedColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
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
            b,
            a: 1f32
        })
    }

    pub fn off() -> Self {
        Self { r: 0f32, g: 0f32, b: 0f32, a: 0f32 }
    }
}
