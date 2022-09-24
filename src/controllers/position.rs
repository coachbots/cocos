use crate::io::interface::IOProvider;
use crate::models::{
    position::Position,
    app_state::AppState
};
use crate::config::AppConfig;
use super::interface::HandlesTick1Ms;
use crate::drivers::motor_driver::MotorDriver;

pub enum PositionControllerError {
}

pub struct PositionController<'a> {
    app_state: &'a AppState,
    left_motor_driver: MotorDriver,
}

impl<'a> PositionController<'a> {
    pub fn new<'b>(app_state: &'a AppState,
                   app_cfg: &AppConfig,
                   io_driver: &'b IOProvider) -> Self {
        Self {
            app_state,
            left_motor_driver: MotorDriver::new(app_cfg.mot_left,
                                                io_driver.gpio_driver.clone())
        }
    }

    fn read_pos_from_uart(&self) -> Position {
        // TODO: Actually implement
        Position::zero()
    }
}

impl<'a> HandlesTick1Ms for PositionController<'a> {
    fn on_tick1(&mut self) {
        // TODO: Reconsider this lock.
        let mut pos_lock = self.app_state.position.lock_mut();
        *pos_lock = self.read_pos_from_uart();
    }
}
