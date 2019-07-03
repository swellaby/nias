use std::fs::File;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

#[cfg(target_family = "unix")]
fn create_file(path: PathBuf, make_executable: bool) -> Result<File, ()> {
    let file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => return Err(()),
    };

    if make_executable {
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return Err(()),
        };
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        if std::fs::set_permissions(&path, permissions).is_err() {
            return Err(());
        };
    };

    Ok(file)
}

#[cfg(target_family = "windows")]
fn create_file(path: PathBuf, _make_executable: bool) -> Result<File, ()> {
    match File::create(&path) {
        Err(_) => Err(()),
        Ok(file) => Ok(file),
    }
}

pub fn get_file_writer(
) -> fn(file_path: &str, contents: &str, make_executable: bool) -> Result<(), String> {
    |file_path: &str, contents: &str, make_executable: bool| {
        let path = PathBuf::from(file_path);
        let mut file = match create_file(path, make_executable) {
            Ok(f) => f,
            Err(_) => return Err(format!("Failed to create file {}", file_path)),
        };

        match file.write_all(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Failed to write contents to {}", file_path)),
        }
    }
}

pub fn get_file_existence_checker() -> fn(file_path: &str) -> Result<bool, ()> {
    |file_path: &str| Ok(Path::new(file_path).exists())
}

#[cfg(test)]
#[path = "file_io_test.rs"]
mod file_io_tests;
