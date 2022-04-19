use std::{fs, io::Write, path::PathBuf};

// ====================
// function
// ====================

pub fn read_dir(dir_path: &str) -> fs::ReadDir {
    fs::read_dir(dir_path).unwrap()
}

pub fn read_file(file_path: &PathBuf) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn update(file_path: &PathBuf, byte_contents: &[u8]) {
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(byte_contents).unwrap();
}
