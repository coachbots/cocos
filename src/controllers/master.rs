use crate::{
    config::AppConfig,
    io::interface::IOProvider,
    models::app_state::AppState
};

use super::{
    api::ApiController,
    position::PositionController
};

pub struct MasterController<'a> {
    app_state: &'a AppState,
    position_controller: PositionController<'a>,
    api_controller: ApiController<'a>,
}

impl<'a> MasterController<'a> {
    pub fn new(app_state: &'a AppState,
               app_cfg: &AppConfig,
               io_provider: &IOProvider) -> Box<Self> {
        Box::new(Self {
            app_state,
            position_controller: PositionController::new(app_state, app_cfg,
                                                         io_provider),
            api_controller: ApiController::new(app_state),
        })
    }

    fn run(&mut self) {
    }
}
