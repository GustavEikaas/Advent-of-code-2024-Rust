pub mod part1;
pub mod part2;
pub mod utils;

use std::{fs::File, io::Read};

use part1::part1;
use part2::part2;

fn main() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    println!("Day 2");
    part1(contents.to_owned());
    part2(contents.to_owned());
}
