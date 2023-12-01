use std::{fs::File, io::Read};

pub fn day_1() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let lines = contents.split("\n").map(get_numbers);

    // Display the contents of the file
    println!("Part 1 answer:{}", lines.sum::<i32>());
}

fn get_numbers(line: &str) -> i32 {
    return get_first_number_in_substring(line.to_owned().split("")) * 10
        + get_first_number_in_substring(line.to_owned().rsplit(""));
}

fn get_first_number_in_substring<'a, I>(mut substrings: I) -> i32
where
    I: Iterator<Item = &'a str>,
{
    return substrings
        .find(|s| match s.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        })
        .unwrap()
        .parse::<i32>()
        .unwrap();
}
