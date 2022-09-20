use rxrust::subject::{LocalBehaviorSubject};

use uom::si::f32::Length;
use uom::si::length::meter;

use super::{HandlesTick1Ms, peripheral::PeripheralController};

pub type Position = (Length, Length);
pub enum PositionControllerError {
}

pub struct PositionController<'a> {
    position_subject: LocalBehaviorSubject<'a, Position,
                                           PositionControllerError>,
    peripheral_controller: &'a PeripheralController,
}

impl<'a> PositionController<'a> {
    pub fn new(peripheral_controller: &'a PeripheralController) -> Self {
        Self {
            position_subject: LocalBehaviorSubject::new(
                                  (Length::new::<meter>(0.0),
                                   Length::new::<meter>(0.0))),
            peripheral_controller
        }
    }

    fn read_pos_from_uart(&self) -> Position {
        self.peripheral_controller.get_position()
    }
}

impl<'a> HandlesTick1Ms for PositionController<'a> {
    fn on_tick1(&mut self) {
        let new_pos = self.read_pos_from_uart();
        // TODO: Emit from subjet
    }
}
