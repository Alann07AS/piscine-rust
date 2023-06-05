use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn open_or_create(file_name: &str, content: &str) {
    OpenOptions::new()
    .write(true)
    .create(true)
    .open(file_name)
    .unwrap()
    .write_all(content.as_bytes()).unwrap()
}
