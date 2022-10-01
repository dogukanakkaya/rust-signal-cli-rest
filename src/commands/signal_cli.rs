use std::process::Command;

// @todo check this out https://docs.rs/duct/0.13.4/duct/
pub fn command() -> Command {
    let mut command = Command::new("docker");
    command.arg("exec").arg("-i").arg("signal-cli");
    command
}
