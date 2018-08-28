use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    read_username_from_file("hello.txt").unwrap();
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
