use std::ops::Index;

struct Line {
    line: String,
    prev_line: Vec<usize>,
    next_line: Vec<usize>,
}

pub fn part1(contents: String) {
    let lines = contents.split("\n");
    let spec_char_map: Vec<Vec<usize>> = lines.to_owned().map(get_character_indices).collect();

    let argh = lines
        .enumerate()
        .map(|(i, line)| {
            let max_index = spec_char_map.len();
            let safe_prev: Vec<usize> = match i {
                0 => vec![],
                _ => spec_char_map.index(i - 1).to_owned(),
            };

            let safe_next: Vec<usize> = match i {
                _ if (i == max_index - 1) => vec![],
                _ => spec_char_map.index(i + 1).to_owned(),
            };

            return Line {
                next_line: safe_next.to_owned(),
                prev_line: safe_prev.to_owned(),
                line: line.to_owned(),
            };
        })
        .map(|val| {
            return map_valid_numbers(&val.line.trim(), val.prev_line, val.next_line, 0);
        });

    println!("{}", argh.fold(0, |acc, curr| acc + curr))
}

struct NumberFind {
    is_finished: bool,
    number: i32,
    initial_index: usize,
}

fn map_valid_numbers(
    line: &str,
    prev_line: Vec<usize>,
    next_line: Vec<usize>,
    carry_over: i32,
) -> i32 {
    // println!(
    //     "prev_line: {}",
    //     String::from_iter(prev_line.iter().map(|x| x.to_string()))
    // );
    // println!("line: {}", line);
    // println!(
    //     "next_line: {}\n",
    //     String::from_iter(next_line.iter().map(|x| x.to_string()))
    // );

    let mut number = carry_over;

    let a = find_number_in_line(line);

    if is_valid(&a, line, &prev_line, &next_line) {
        println!("isValid {}", a.number);
        number += a.number;
    }

    let replacement = String::from_iter((0..a.number.to_string().len()).map(|_| ".").into_iter());

    //more numbers
    let next_sequence = line
        .to_owned()
        .replace(a.number.to_string().as_str(), replacement.as_str());

    let blah = find_number_in_line(&next_sequence);
    if blah.number == 0 {
        return number;
    }

    // return number;
    return map_valid_numbers(
        &next_sequence,
        prev_line.to_owned(),
        next_line.to_owned(),
        number,
    );
}

fn is_valid(a: &NumberFind, line: &str, prev_line: &Vec<usize>, next_line: &Vec<usize>) -> bool {
    //check if spec char on same line

    // println!("Checking number: {}", a.number);
    let chars: Vec<char> = vec!['/', '$', '&', '%', '+', '-', '=', '@', '#', '*'];
    let start_index = a.initial_index;
    let end_index = a.initial_index + a.number.to_string().len() - 1;

    let has_preceding_char = match start_index {
        0 => {
            //checks if the last character on the preceding line is a spec char
            if prev_line.len() == 0 {
                false
            } else {
                let last_of_prev = prev_line.get(prev_line.len() - 1);
                let res = match last_of_prev {
                    Some(&i) => {
                        if line.len() - 1 == i {
                            // println!("{}", a.number);
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                };

                res
            }
        }
        _ => {
            let preceding_char = line.chars().nth(start_index - 1).unwrap();
            chars.contains(&preceding_char)
        }
    };

    if has_preceding_char {
        return true;
    }

    let has_trailing_char = match end_index {
        _ if end_index == line.len() - 1 => {
            if (next_line.len() == 0) {
                false
            } else {
                let first_of_next = next_line.get(0);
                let res = match first_of_next {
                    Some(&i) => {
                        if i == 0 {
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                };
                res
            }
        }
        _ => {
            let trailing_char = line.chars().nth(end_index + 1).unwrap();
            chars.contains(&trailing_char)
        }
    };

    if has_trailing_char {
        return true;
    }

    //check if spec char on prev line
    if check_line_valid(start_index, end_index, line, prev_line.to_owned()) {
        return true;
    }

    //check if spec char on next line
    if check_line_valid(start_index, end_index, line, next_line.to_owned()) {
        return true;
    }

    return false;
}

fn check_line_valid(
    start_index: usize,
    end_index: usize,
    line: &str,
    prev_line: Vec<usize>,
) -> bool {
    let preceding_index = match start_index {
        0 => 0,
        _ => start_index - 1,
    };

    let trailing_index = match end_index {
        _ if end_index == line.len() - 1 => end_index,
        _ => end_index + 1,
    };

    let scan: Vec<usize> = (preceding_index..trailing_index + 1).collect();

    let is_valid = prev_line.iter().any(|x| scan.contains(x));

    return is_valid;
}

fn find_number_in_line(line: &str) -> NumberFind {
    let agg = NumberFind {
        is_finished: false,
        number: 0,
        initial_index: 0,
    };

    let res = line.chars().enumerate().fold(agg, |a, (i, b)| {
        let maybe_number = b.to_string().parse::<i32>();
        if a.is_finished {
            return a;
        }

        match maybe_number {
            Ok(val) => {
                if a.number != 0 {
                    return NumberFind {
                        is_finished: false,
                        number: (a.number * 10) + val,
                        initial_index: a.initial_index,
                    };
                }
                return NumberFind {
                    is_finished: false,
                    number: val,
                    initial_index: i,
                };
            }
            Err(_) => {
                if a.number != 0 {
                    return NumberFind {
                        is_finished: true,
                        number: a.number,
                        initial_index: a.initial_index,
                    };
                }
                return NumberFind {
                    is_finished: a.is_finished,
                    number: a.number,
                    initial_index: a.initial_index,
                };
            }
        }
    });

    return res;
}

pub fn get_character_indices(line: &str) -> Vec<usize> {
    let chars: Vec<char> = vec!['/', '\\', '$', '&', '%', '+', '-', '=', '@', '#', '*'];

    let indices: Vec<usize> = vec![];

    return line
        .chars()
        .enumerate()
        .fold(indices, |mut acc, (i, curr)| {
            if chars.contains(&curr) {
                acc.push(i);
            }

            return acc;
        });
}

//characters: \ / $ & % + - = @ # *
