use std::borrow::BorrowMut;
use async_std::task::sleep;
use futures_signals::signal::SignalExt;

use crate::drivers::nucifera_driver::{NuciferaDriver, DrivesNucifera};
use crate::io::interface::IOProvider;
use crate::models::motor_power::{self, MotorPower, MotorPowerQuadrant};
use crate::models::{
    position::Position,
    app_state::AppState
};
use crate::config::AppConfig;
use super::interface::HandlesTick1Ms;
use crate::drivers::motor_driver::{MotorDriver, DrivesMotor, MotorDescriptor, MotorDirection};

pub enum PositionControllerError {
}

pub struct PositionController<'a> {
    app_state: &'a AppState,
    left_motor_driver: MotorDriver,
    right_motor_driver: MotorDriver,
    nucifera_driver: NuciferaDriver
}

impl<'a> PositionController<'a> {
    pub fn new<'b>(app_state: &'a AppState,
                   app_cfg: &AppConfig,
                   io_driver: &'b IOProvider) -> Self {
        Self {
            app_state,
            left_motor_driver: MotorDriver::new(
                app_cfg.mot_left, io_driver.gpio_driver.clone()),
            right_motor_driver: MotorDriver::new(
                app_cfg.mot_left, io_driver.gpio_driver.clone()),
            nucifera_driver: NuciferaDriver::new(
                app_cfg.nucifera, io_driver.uart_driver.clone())
        }
    }

    pub fn init(&mut self, app_cfg: &AppConfig) {
        // Hook into the target_motor_power signal to call the motor update
        // coro.
        let motor_update_future = self.app_state
            .target_motor_power
            .signal()
            .throttle(|| sleep(app_cfg.drive_update_period))
            .for_each(|value| {
                // TODO: Replace this with a call for motor updating. Probably
                // need some sort of mutex.
                sleep(app_cfg.drive_update_period)
            });
    }

    async fn write_motor_update(&mut self, motor_power: MotorPower) {
        let lmot = self.left_motor_driver.borrow_mut();
        let rmot = self.right_motor_driver.borrow_mut();

        match motor_power.as_quadrant() {
            MotorPowerQuadrant::PLeftPRight => {
                lmot.set_direction(MotorDirection::CounterClockwise);
                rmot.set_direction(MotorDirection::Clockwise);
            }
            MotorPowerQuadrant::PLeftNRight => {
                lmot.set_direction(MotorDirection::CounterClockwise);
                rmot.set_direction(MotorDirection::CounterClockwise);
            }
            MotorPowerQuadrant::NLeftPRight => {
                lmot.set_direction(MotorDirection::Clockwise);
                rmot.set_direction(MotorDirection::Clockwise);
            }
            MotorPowerQuadrant::NLeftNRight => {
                rmot.set_direction(MotorDirection::CounterClockwise);
                lmot.set_direction(MotorDirection::Clockwise);
            }
        };

        lmot.set_speed(motor_power.pow_left());
        rmot.set_speed(motor_power.pow_right());
    }

    /// Calls the underlying driver to retrieve the current position.
    fn read_pos_from_uart(&mut self) -> Position {
        let driver = self.nucifera_driver.borrow_mut();
        driver.read_current_position()
    }
}

impl<'a> HandlesTick1Ms for PositionController<'a> {
    fn on_tick1(&mut self) {
        // TODO: Reconsider this lock.
        let mut pos_lock = self.app_state.position.lock_mut();
        *pos_lock = self.read_pos_from_uart();
    }
}
