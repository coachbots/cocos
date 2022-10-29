use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;
use std::borrow::Borrow;
use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;

struct ApiController {
    running_process: Option<Rc<RefCell<Popen>>>,
    api_messager: ApiMessager,
    script: Vec<u8>,
}

impl ApiController {
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

    pub fn run_tick(&mut self) {
        self.api_messager.run_tick()
    }

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
            }
        }

        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), ApiError> {
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

const TEST_SCRIPT: &'static [u8] = b"
def usr(bot):
    while True:
        bot.set_vel(100, 100)
";

fn main() {
    let mut api_controller = ApiController::new("ipc:///tmp/cocostx");
    api_controller.set_script(TEST_SCRIPT.to_vec());
    api_controller.restart_api().expect("Could not start the api");
    loop {
        api_controller.run_tick();
    }
}
