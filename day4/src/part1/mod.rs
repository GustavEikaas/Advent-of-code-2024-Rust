pub fn part1(contents: String) {
    let result = contents
        .split("\n")
        .map(|x| {
            let mut line_content = x.split(":").nth(1).unwrap().split("|");

            let win_numbers = process_part_line(line_content.next().to_owned().unwrap());
            let your_numbers = process_part_line(line_content.next().to_owned().unwrap());

            let res = your_numbers
                .iter()
                .filter(|x| win_numbers.contains(x))
                .count();

            if res == 0 {
                return 0;
            }

            return 2i32.pow((res - 1).try_into().unwrap());
        })
        .fold(0, |agg, curr| agg + curr);

    println!("Result: {}", result);
}

fn process_part_line(content: &str) -> Vec<i32> {
    return content
        .split(" ")
        .map(|x| x.trim().parse::<i32>())
        .filter(|x| Result::is_ok(&x))
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();
}
