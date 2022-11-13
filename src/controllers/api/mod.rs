pub mod errors;
pub mod ipc_requests;
pub mod ipc_responses;
pub mod messager;

use errors::ApiError;
use messager::ApiMessager;
use std::io::Write;
use std::{
    borrow::Borrow,
    sync::{Arc, Mutex},
    time::Duration,
};
use subprocess::{Popen, PopenConfig, Redirection};

use crate::models::api::{ApiTickInputMessage, ApiTickOutputMessage};

/// Exposes the API controller that controlls spawning and messaging the API
/// child process.
pub struct ApiController {
    running_process: Option<Arc<Mutex<Popen>>>,
    api_messager: Mutex<ApiMessager>,
    script: Vec<u8>,
}

impl ApiController {
    /// Spawns a new API Controller.
    pub fn new(comm_uri: &'static str) -> ApiController {
        ApiController {
            running_process: Option::None,
            api_messager: Mutex::new(ApiMessager::new(comm_uri)),
            script: vec![],
        }
    }

    /// Sets the currently executed script.
    ///
    /// This function can only be called when the script is paused and returns
    /// ApiError::General otherwise.
    pub fn set_script(&mut self, string: Vec<u8>) -> Result<(), ApiError> {
        if !self.running_process.is_none() {
            return Err(ApiError::General);
        }
        self.script = string;
        Ok(())
    }

    /// Runs a tick of the controller. Must be called in a looped task.
    pub fn run_tick(
        &mut self,
        data: ApiTickInputMessage,
    ) -> Result<ApiTickOutputMessage, ApiError> {
        // TODO: Dangerous unwrap
        self.api_messager.lock().unwrap().run_tick(data)
    }

    /// Restarts the API process. Can be called to initally start the process.
    pub fn restart_api(&mut self) -> Result<(), ApiError> {
        let kill_err = self.kill();
        if kill_err.is_err() {
            return Err(kill_err.unwrap_err());
        }
        let mut messager = self.api_messager.lock().unwrap();

        let api_proc = Popen::create(
            &["python2", "-m", "cocos_py2", messager.comm_file],
            PopenConfig {
                stdin: Redirection::Pipe,
                detached: true,
                ..Default::default()
            },
        );

        if api_proc.is_err() {
            return Err(ApiError::ProcessError);
        }

        self.running_process = Some(Arc::new(Mutex::new(api_proc.unwrap())));
        match &self.running_process {
            None => {
                return Err(ApiError::ProcessError);
            }
            Some(proc_rc) => {
                let proc_arc = proc_rc.clone();
                let mut proc = proc_arc.lock().unwrap();
                match &mut proc.stdin {
                    None => {
                        return Err(ApiError::IO);
                    }
                    Some(stdin) => {
                        if stdin.write_all(self.script.borrow()).is_err() {
                            return Err(ApiError::IO);
                        }
                    }
                }
            }
        }

        match messager.start() {
            Ok(()) => {}
            Err(err) => {
                return Err(err);
            }
        }

        Ok(())
    }

    /// Kills the child process and cleans up resources.
    pub fn kill(&mut self) -> Result<(), ApiError> {
        let messager = &mut self.api_messager.lock().unwrap();
        messager.stop();

        // Kill the process of whose Popen we hold. Note that a malicious actor
        // could hook into SIGTERM and prevent us from shutting down, so we
        // must, after a while, run kill if the process is not done.
        match &self.running_process {
            Some(proc) => {
                let mut borrowed = proc.lock().unwrap();
                if borrowed.terminate().is_err() {
                    return Err(ApiError::ProcessError);
                }

                // Kill if the process is alive after some time.
                // Hook into this you dirty python snake!
                // TODO: Magic number
                // TODO: Dangerous unwrap
                match borrowed.wait_timeout(Duration::from_secs(2)).unwrap() {
                    Some(_) => {
                        return Err(ApiError::ProcessError);
                    }
                    None => {
                        borrowed.kill(); // TODO: Handle error
                    }
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
