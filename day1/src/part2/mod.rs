use std::{collections::HashMap, fs::File, io::Read};

pub fn part2() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let lines = contents.split("\n").skip(990).take(10).map(get_numbers);

    // Display the contents of the file
    println!("Part 1 answer:{}", lines.sum::<i32>());
}

fn get_numbers(line: &str) -> i32 {
    let first_number = get_first_number_in_substring(line.to_owned(), false);
    let last_number = get_first_number_in_substring(line.to_owned(), true);

    print!("number is: {}{}\n", first_number, last_number);
    return first_number * 10 + last_number;
}

fn get_first_number_in_substring(substr: String, reverse: bool) -> i32 {
    let numbers: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    if reverse {
        let string_number_match = numbers
            .into_iter()
            .reduce(|acc, curr| {
                let acc_index = last_index_of(&substr, acc.0, 0).unwrap_or(0);
                let curr_index = last_index_of(&substr, curr.0, 0).unwrap_or(0);

                if acc_index < curr_index {
                    return curr;
                } else {
                    return acc;
                }
            })
            .unwrap_or(("", 0));

        let string_index = last_index_of(&substr, string_number_match.0, 0);

        //needs to either reverse iterate while reading ltr or find the last instance of
        // let string_index = substr.find(string_number_match.0).unwrap_or(0);

        //number as text that matched
        let integer = get_last_number_in_string(substr);

        // print!("stringIndex {}", string_index.unwrap());

        return match (integer, string_index) {
            (Some(integer_match), Some(str_index)) => {
                if integer_match.index > str_index {
                    return integer_match.value;
                } else {
                    return string_number_match.1;
                }
            }
            (Some(integer_match), None) => integer_match.value,
            (None, Some(_)) => string_number_match.1,
            (None, None) => 0,
        };
    } else {
        let string_number_match = numbers
            .into_iter()
            .reduce(|acc, curr| {
                let acc_index = substr.find(acc.0).unwrap_or(99);
                let curr_index = substr.find(curr.0).unwrap_or(99);

                if acc_index > curr_index {
                    return curr;
                } else {
                    return acc;
                }
            })
            .unwrap_or_default();

        let string_index = substr.find(string_number_match.0).unwrap_or(99);

        let integer = get_first_number_in_string(substr);

        if Option::is_some(&integer) {
            let val = integer.unwrap();
            if val.index < string_index {
                return val.value;
            }
            return string_number_match.1;
        } else {
            return string_number_match.1;
        }
    }
}

struct NumberMatch {
    index: usize,
    value: i32,
}

fn get_last_number_in_string(substr: String) -> Option<NumberMatch> {
    let integer_match = substr
        .chars()
        .rev()
        .find(|s| match s.to_string().parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        });

    if Option::is_some(&integer_match) {
        let integer = integer_match.unwrap().to_string();

        // print!("reversed: {}", substr.chars().rev().collect::<String>());
        let integer_index = substr.chars().rev().collect::<String>().find(&integer);

        if Option::is_some(&integer_index) {
            let len = substr.len();
            return Some(NumberMatch {
                index: len - integer_index.unwrap(),
                value: integer.parse::<i32>().unwrap(),
            });
        }
    }

    return None;
}

fn get_first_number_in_string(substr: String) -> Option<NumberMatch> {
    let integer_match = substr
        .split("")
        .find(|s| match s.to_string().parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        });

    if Option::is_some(&integer_match) {
        let integer = integer_match.unwrap().to_string();
        let integer_index = substr.find(&integer);

        if Option::is_some(&integer_index) {
            return Some(NumberMatch {
                index: integer_index.unwrap(),
                value: integer.parse::<i32>().unwrap(),
            });
        }
    }

    return None;
}

fn last_index_of(str: &str, substr: &str, carry_over: usize) -> Option<usize> {
    let strings = str.find(substr);

    if Option::is_none(&strings) {
        return None;
    }

    let sub_string_index = strings.unwrap();

    let sub_slice_length = sub_string_index + substr.len();

    if str.len() < sub_slice_length {
        return Some(sub_string_index);
    }

    let rest = str[sub_string_index + substr.len()..].to_string();
    if rest.contains(substr) {
        return last_index_of(rest.as_str(), substr, sub_string_index + substr.len());
    }
    return Some(sub_string_index + carry_over);
}
