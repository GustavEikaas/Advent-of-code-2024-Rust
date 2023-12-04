#[derive(Debug)]
struct CardScore {
    matches: usize,
    index: usize,
    instance_count: usize,
}

impl Clone for CardScore {
    fn clone(&self) -> Self {
        CardScore {
            matches: self.matches,
            index: self.index,
            instance_count: self.instance_count,
        }
    }
}

pub fn part2(contents: String) {
    let mut result = contents
        .split("\n")
        .enumerate()
        .map(|(i, x)| {
            let mut line_content = x.split(":").nth(1).unwrap().split("|");

            let win_numbers = process_part_line(line_content.next().to_owned().unwrap());
            let your_numbers = process_part_line(line_content.next().to_owned().unwrap());

            let res = your_numbers
                .iter()
                .filter(|x| win_numbers.contains(x))
                .count();

            return CardScore {
                index: i,
                matches: res,
                instance_count: 1,
            };
        })
        .collect::<Vec<CardScore>>();

    // use indices to avoid borrowing
    let len = result.len();

    for index in 0..len {
        let start_index = result[index].index + 1;
        let end_index = result[index].index + result[index].matches + 1;

        for _ in 0..result[index].instance_count {
            for i in start_index..end_index {
                // clone element to avoid borrowing
                let el = result[i].clone();
                result[i] = CardScore {
                    index: el.index,
                    instance_count: el.instance_count + 1,
                    matches: el.matches,
                };
            }
        }
    }

    println!(
        "Result: {}",
        result.iter().fold(0, |acc, curr| acc + curr.instance_count)
    );
}

fn process_part_line(content: &str) -> Vec<i32> {
    return content
        .split(" ")
        .map(|x| x.trim().parse::<i32>())
        .filter(|x| Result::is_ok(&x))
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();
}
