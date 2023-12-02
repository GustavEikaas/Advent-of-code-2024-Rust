use std::{fs::File, io::Read};

pub fn part2() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let lines = contents.split("\n");

    // Display the contents of the file
    // println!("Part 2 answer:{}", lines.sum::<i32>());
}
