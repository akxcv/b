use std::process::Command;

pub fn run_command<T: AsRef<str>>(cmd: T) {
    Command::new("sh").arg("-c").arg(cmd.as_ref()).spawn().unwrap().wait().unwrap();
}
