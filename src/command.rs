use std::io::Result as IOResult;
use std::process::{Command, Output};

#[cfg(target_family = "unix")]
const PROGRAM: &str = "sh";
#[cfg(target_family = "unix")]
const SWITCH: &str = "-c";

#[cfg(target_family = "windows")]
const PROGRAM: &str = "cmd";
#[cfg(target_family = "windows")]
const SWITCH: &str = "/C";

fn handle_command_output(output: IOResult<Output>) -> Result<String, String> {
    match output {
        Err(details) => panic!(
            "Command runner crashed in unrecoverable manner. Details: {}",
            details
        ),
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8(output.stdout)
                    .unwrap()
                    .trim_end_matches('\n')
                    .to_string())
            } else {
                Err(format!(
                    "{}\n{}",
                    String::from_utf8(output.stderr).unwrap(),
                    String::from_utf8(output.stdout).unwrap(),
                ))
            }
        }
    }
}

pub fn get_command_runner() -> fn(cmd: &str, dir: Option<&str>) -> Result<String, String> {
    |cmd: &str, dir: Option<&str>| {
        let target_dir = match dir {
            Some(d) => d,
            None => ".",
        };

        handle_command_output(
            Command::new(PROGRAM)
                .current_dir(target_dir)
                .args(&[SWITCH, cmd])
                .output(),
        )
    }
}

#[cfg(test)]
#[path = "command_test.rs"]
mod command_tests;
