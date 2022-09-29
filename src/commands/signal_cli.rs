use std::process::Command;

pub fn command() -> Command {
    let mut command = Command::new("docker");
    command.arg("exec").arg("-i").arg("signal-cli");
    command
}
