use uom::si::{f32::Length, length::meter};

pub type RelativeSpeed = (f32, f32);
pub type LedColor = (f32, f32, f32);
pub type Position = (Length, Length);

pub trait HandlesPyApi {
    /// Called by the python API to set the current speed.
    ///
    /// # Arguments:
    ///
    /// * `rel_speed` - The relative speed of the two motors. Values must be
    ///                 between -1 and 1.
    fn set_rel_speed(&mut self, rel_speed: RelativeSpeed);

    /// Called by the python API to set the current LED color.
    ///
    /// # Arguments:
    ///
    /// * `led_color` - The color of the LED. Values must fit 0-1.
    fn set_led_color(&mut self, led_color: LedColor);

    /// Called by the python API to retrieve the current bot position.
    fn get_pos(&mut self) -> Position;
}

struct PythonApiController {
    child_pid: u32
}

impl PythonApiController {
    pub fn new() -> Self {
        Self {
            child_pid: 0
        }
    }
}

impl HandlesPyApi for PythonApiController {
    fn set_rel_speed(&mut self, rel_speed: RelativeSpeed) {
        
    }

    fn set_led_color(&mut self, led_color: LedColor) {
        
    }

    fn get_pos(&mut self) -> Position {
        (Length::new::<meter>(0.0), Length::new::<meter>(0.0))
    }
}
