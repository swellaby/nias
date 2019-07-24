use super::*;

#[cfg(test)]
mod get_command_runner_tests {
    use super::get_command_runner;

    #[test]
    fn returns_correct_closure() {
        fn validate(_run_command: fn(&str, Option<&str>) -> Result<String, String>) -> bool {
            true
        };
        let command_runner = get_command_runner();
        assert!(validate(command_runner));
    }
}

#[cfg(test)]
mod handle_command_output_tests {
    use super::*;
    use std::io::{Error, ErrorKind};
    use std::process::{ExitStatus, Output};

    #[test]
    #[should_panic(
        expected = "Command runner crashed in unrecoverable manner. Details: crashed hard"
    )]
    fn should_panic_on_command_error() {
        let error_details = "crashed hard";
        let error = Error::new(ErrorKind::Other, error_details);
        let _ = handle_command_output(Err(error));
    }

    #[cfg(target_family = "windows")]
    fn get_exit_status(raw: u32) -> ExitStatus {
        use std::os::windows::process::ExitStatusExt;
        ExitStatus::from_raw(raw)
    }

    #[cfg(target_family = "unix")]
    fn get_exit_status(raw: u32) -> ExitStatus {
        use std::os::unix::process::ExitStatusExt;
        ExitStatus::from_raw(raw as i32)
    }

    #[test]
    fn should_return_ok_with_stdout_on_command_success() {
        let original = "hello\nworld!\n\n";
        let exp = "hello\nworld!";
        let stdout_result = String::from(original);
        let stdout = stdout_result.as_bytes();
        let raw = 0;
        let output = Output {
            status: get_exit_status(raw),
            stdout: Vec::from(stdout),
            stderr: Vec::new(),
        };
        let result = handle_command_output(Ok(output));
        assert_eq!(&result.unwrap(), exp);
    }
}
