use std::{fs::File, io::Read};

pub fn part2() {
    let mut contents = String::new();
    File::open("./src/input.txt")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let score = contents
        .split("\n")
        .map(map_to_game_line)
        .map(|x| x.blue * x.red * x.green)
        .fold(0, |acc, curr| acc + curr);

    println!("Part 1 answer:{}", score);
}

struct GameLine {
    red: i32,
    green: i32,
    blue: i32,
}

fn map_to_game_line(line: &str) -> GameLine {
    let mut line_array = line.split(":");

    let cube_sets = line_array.nth(1).unwrap().split(";");

    let cube_store = GameLine {
        blue: 0,
        green: 0,
        red: 0,
    };

    let cube_max = cube_sets.fold(cube_store, |mut acc, curr| {
        let cubes = curr.split(",");
        let init = GameLine {
            blue: 0,
            green: 0,
            red: 0,
        };

        let round_score = cubes.fold(init, |mut acc, curr| {
            let count: i32 = curr
                .replace("red", "")
                .replace("green", "")
                .replace("blue", "")
                .trim()
                .parse::<i32>()
                .unwrap();

            let cube_color = find_color(curr);

            match cube_color.as_str() {
                "red" => {
                    acc.red += count;
                }
                "blue" => {
                    acc.blue += count;
                }
                "green" => {
                    acc.green += count;
                }
                &_ => {}
            }

            return acc;
        });

        if round_score.red > acc.red {
            acc.red = round_score.red
        }
        if round_score.blue > acc.blue {
            acc.blue = round_score.blue
        }
        if round_score.green > acc.green {
            acc.green = round_score.green
        }

        return acc;
    });

    return cube_max;
}

fn find_color(line: &str) -> String {
    return match () {
        _ if line.contains("red") => "red".to_string(),
        _ if line.contains("green") => "green".to_string(),
        _ => "blue".to_string(),
    };
}
