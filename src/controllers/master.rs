use crate::{config::AppConfig, io::interface::ProvidesIO};

use super::{
    api::ApiController,
    position::PositionController,
    peripheral::PeripheralController
};

pub struct MasterController<'a> {
    position_controller: PositionController<'a>,
    peripheral_controller: PeripheralController,
    api_controller: ApiController,
}

impl<'a> MasterController<'a> {
    fn new(app_cfg: AppConfig, io_provider: Box<dyn ProvidesIO>) -> Self {
        let peripheral_controller = PeripheralController::new(app_cfg,
                                                              io_provider);
        let position_controller = 
            PositionController::new(&peripheral_controller);
        let api_controller = ApiController::new();

        Self {
            position_controller,
            peripheral_controller, 
            api_controller,
        }
    }
}
