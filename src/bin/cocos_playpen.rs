use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;

fn main() {
    println!("Running");
    let mut api_proc = Popen::create(&["ps", "x"], PopenConfig {
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
