mod part1;

use std::{fs::File, io::Read};

use part1::part1;

fn main() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    println!("Day 4");
    part1(contents.to_owned());
}
