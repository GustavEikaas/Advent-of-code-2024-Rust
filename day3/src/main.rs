use std::{fs::File, io::Read};

mod part1;
use part1::part1;

fn main() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    println!("Day 3");
    part1(contents.to_owned());
    // part2(contents.to_owned());
}
