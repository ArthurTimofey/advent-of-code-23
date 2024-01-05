use std::fs;
use std::io;

pub fn run(file_name: String) -> io::Result<String> {
    fs::read_to_string(format!("./input/{}.txt", file_name))
}
