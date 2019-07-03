use super::*;

#[cfg(test)]
mod get_file_writer_tests {
    use super::*;

    #[test]
    fn returns_correct_closure() {
        fn validate(_write_file: fn(&str, &str, bool) -> Result<(), String>) -> bool {
            true
        };
        let writer = get_file_writer();
        assert!(validate(writer));
    }
}

#[cfg(test)]
mod get_file_existence_checker_tests {
    use super::*;

    #[test]
    fn returns_correct_closure() {
        fn validate(_exists: fn(&str) -> Result<bool, ()>) -> bool {
            true
        };
        let writer = get_file_existence_checker();
        assert!(validate(writer));
    }
}
