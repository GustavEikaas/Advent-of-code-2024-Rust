use crate::utils::{extract_number_from_draw, find_color, GameLine};

pub fn part1(contents: String) {
    let score = contents
        .split("\n")
        .map(map_to_game_line)
        .fold(0, |acc, curr| acc + curr);

    println!("Part 1 answer:{}", score);
}

fn filter_valid_game(game: &GameLine) -> bool {
    return !(game.red > 12 || game.green > 13 || game.blue > 14);
}

fn map_to_game_line(line: &str) -> i32 {
    let mut line_array = line.split(":");

    let game_info = line_array.next().unwrap();
    let mut cube_sets = line_array.next().unwrap().split(";");

    let game_id: i32 = game_info.replace("Game ", "").parse().unwrap();

    let is_valid = cube_sets.all(|x| {
        let cubes = x.split(",");

        let init = GameLine {
            blue: 0,
            green: 0,
            red: 0,
        };

        let round_score = cubes.fold(init, |mut acc, curr| {
            let count = extract_number_from_draw(curr).unwrap();

            let cube_color = find_color(curr);

            if cube_color == "red".to_string() {
                acc.red += count;
            } else if cube_color == "green".to_string() {
                acc.green += count;
            } else {
                acc.blue += count;
            }

            return acc;
        });

        if filter_valid_game(&round_score) == false {
            return false;
        }
        return true;
    });

    if is_valid {
        return game_id;
    }

    return 0;
}
