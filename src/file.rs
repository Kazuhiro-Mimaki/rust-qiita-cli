use std::{fs, io::Write};

pub fn read(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

pub fn update(file_path: &str, byte_contents: &[u8]) {
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(byte_contents).unwrap();
}
