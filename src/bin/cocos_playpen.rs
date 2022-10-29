use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;

enum ApiError {
    General
}

struct ApiController {
    running_process: Rc<RefCell<Option<Popen>>>,
    comm_file: &'static str
}

impl ApiController {
    pub fn new(comm_uri: &'static str) -> ApiController {
        ApiController {
            running_process: Rc::new(RefCell::new(Option::None)),
            comm_file: comm_uri
        }
    }

    pub fn run_tick(&mut self) {
    }

    pub fn restart_api(&mut self) -> Result<(), ApiError> {
        let api_proc = Popen::create(
            &["python", "-m", "cocos_py2", self.comm_file],
            PopenConfig {
                stdin: Redirection::Pipe,
                ..Default::default()
            }
        );
        if api_proc.is_err() {
            return Err(ApiError::General);
        }
        self.running_process = Rc::new(RefCell::new(Some(api_proc.unwrap())));

        Ok(())
    }

    pub fn kill(&mut self) -> Result<(), ApiError> {
        Ok(())
    }
}

fn main() {
    println!("Running");
    let mut api_proc = Popen::create(&["python", "-m", "cocos_py2",
                                       "ipc:///tmp/cocostx"],
        PopenConfig {
            stdout: Redirection::Pipe, ..Default::default()
    }).expect("Could not spawn");

    let (out, err) = api_proc.communicate(None).expect("Could not comm");

    api_proc.wait();

    if let Some(exit_status) = api_proc.poll() {
        println!("{:?}: {}", exit_status, out.expect("Stdout"));
    } else {
        api_proc.terminate().expect("Killed");
        println!("Succ killed");
    }
}
