pub mod ipc_requests;
pub mod ipc_responses;
pub mod messager;
pub mod errors;

use std::rc::Rc;
use std::cell::RefCell;
use subprocess::{
    Popen,
    PopenConfig,
    Redirection
};
use messager::ApiMessager;
use errors::ApiError;

/// Exposes the API controller that controlls spawning and messaging the API
/// child process.
pub struct ApiController {
    running_process: Option<Rc<RefCell<Popen>>>
    api_messager: ApiMessager,
    script: Vec<u8>,
}

impl ApiController {
    /// Spawns a new API Controller.
    pub fn new(comm_uri: &'static str) -> ApiController {
        ApiController {
            running_process: Option::None,
            api_messager: ApiMessager::new(comm_uri),
            script: vec![]
        }
    }

    /// Sets the currently executed script.
    ///
    /// This function can only be called when the script is paused and returns
    /// ApiError::General otherwise.
    pub fn set_script(&mut self, string: Vec<u8>) -> Result<(), ApiError> {
        if self.running_process.is_none() { return Err(ApiError::General); }
        self.script = string;
        Ok(())
    }

    /// Runs a tick of the controller. Must be called in a looped task.
    pub fn run_tick(&mut self) {
        self.api_messager.run_tick()
    }

    /// Restarts the API process. Can be called to initally start the process.
    pub fn restart_api(&mut self) -> Result<(), ApiError> {
        let kill_err = self.kill();
        if kill_err.is_err() { return Err(kill_err.unwrap_err()); }

        let api_proc = Popen::create(
            &["python", "-m", "cocos_py2", self.api_messager.comm_file],
            PopenConfig {
                stdin: Redirection::Pipe,
                detached: true,
                ..Default::default()
            }
        );

        if api_proc.is_err() {
            return Err(ApiError::ProcessError);
        }
        self.running_process = Some(Rc::new(RefCell::new(api_proc.unwrap())));
        match &self.running_process {
            None => { return Err(ApiError::ProcessError); }
            Some(proc_rc) => {
                let mut proc = proc_rc.borrow_mut();
                match &mut proc.stdin {
                    None => { return Err(ApiError::IO); }
                    Some (stdin) => {
                        if stdin.write_all(self.script.borrow()).is_err() {
                            return Err(ApiError::IO);
                        }
                    }
                }
            }
        }

        let messager = &mut self.api_messager;
        match messager.start() {
            Ok(()) => {
            }
            Err(err) => {
                return Err(err);
            }
        }

        Ok(())
    }

    /// Kills the child process and cleans up resources.
    pub fn kill(&mut self) -> Result<(), ApiError> {
        let messager = &mut self.api_messager;
        messager.stop();

        match &self.running_process {
            Some(proc) => {
                let mut borrowed = proc.borrow_mut();
                if borrowed.kill().is_err() {
                    return Err(ApiError::ProcessError);
                }
                Ok(())
            }
            None => {
                Ok(())
            }
        }
    }
}
