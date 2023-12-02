use std::str::Split;

use crate::utils::{extract_number_from_draw, find_color, GameLine};

pub fn part2(contents: String) {
    let score = contents
        .split("\n")
        .map(map_to_game_line)
        .map(|x| x.blue * x.red * x.green)
        .fold(0, |acc, curr| acc + curr);

    println!("Part 2 answer:{}", score);
}

fn map_to_game_line(line: &str) -> GameLine {
    let mut line_array = line.split(":");

    let cube_sets = line_array.nth(1).unwrap().split(";");

    let cube_init = GameLine {
        blue: 0,
        green: 0,
        red: 0,
    };

    let cube_max = cube_sets.fold(cube_init, |mut acc, curr| {
        let cubes = curr.split(",");
        let draw = calculate_draw(cubes);

        if draw.red > acc.red {
            acc.red = draw.red
        }
        if draw.blue > acc.blue {
            acc.blue = draw.blue
        }
        if draw.green > acc.green {
            acc.green = draw.green
        }

        return acc;
    });

    return cube_max;
}

fn calculate_draw<'a>(cubes: Split<&'a str>) -> GameLine {
    let init = GameLine {
        blue: 0,
        green: 0,
        red: 0,
    };

    let draw_score = cubes.fold(init, |mut acc, curr| {
        let count = extract_number_from_draw(curr).unwrap();

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
    return draw_score;
}
