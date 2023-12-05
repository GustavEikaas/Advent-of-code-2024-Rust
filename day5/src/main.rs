mod part1;

use std::{fs::File, io::Read};

use part1::part1;
fn main() {
    let mut file_contents: String = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();

    part1(file_contents);
}
