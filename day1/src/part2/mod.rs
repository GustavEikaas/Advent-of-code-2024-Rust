use std::{collections::HashMap, fs::File, io::Read};

use crate::part1::get_numbers;

pub fn part2() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let lines = contents
        .split("\n")
        .map(replace_with_numbers)
        .map(|x| get_numbers(&x));

    println!("Part 2 answer:{}", lines.sum::<i32>());
}

struct StringMatch {
    index: usize,
    value: String,
    number: i32,
}

fn replace_with_numbers(line: &str) -> String {
    let string_number_match: Option<StringMatch> = get_sub_string(line);

    return match string_number_match {
        Some(val) => {
            let new_line = line.replacen(val.value.as_str(), val.number.to_string().as_str(), 1);
            let maybe_more = get_sub_string(&new_line);
            return match maybe_more {
                Some(_) => replace_with_numbers(&new_line),
                None => new_line.to_owned(),
            };
        }
        None => line.to_owned(),
    };
}

fn get_sub_string(line: &str) -> Option<StringMatch> {
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

    let string_number_match: Option<StringMatch> = numbers.into_iter().fold(None, |x, a| {
        let curr_index = line.find(a.0);

        if Option::is_none(&curr_index) {
            return x;
        }

        let word_index = curr_index.unwrap();
        let maybe_index = x.unwrap_or(StringMatch {
            index: 99,
            value: "".to_string(),
            number: 0,
        });

        if word_index < maybe_index.index {
            if (word_index + a.0.len() - 1) == maybe_index.index {
                return Some(StringMatch {
                    index: word_index,
                    value: format!("{}{}", a.0, maybe_index.value[1..].to_string()),
                    number: a.1 * 10 + maybe_index.number,
                });
            }

            return Some(StringMatch {
                index: word_index,
                number: a.1,
                value: a.0.to_owned(),
            });
        }

        if maybe_index.index == 99 {
            return None;
        }

        if (maybe_index.index + maybe_index.value.len() - 1 == word_index) {
            return Some(StringMatch {
                index: maybe_index.index,
                value: format!("{}{}", maybe_index.value, a.0[1..].to_string()),
                number: maybe_index.number * 10 + a.1,
            });
        }
        return Some(maybe_index);
    });

    return string_number_match;
}
