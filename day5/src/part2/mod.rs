pub fn part2(contents: String) {
    let processed = process_input(contents);
    println!("Result: {}", processed);
}

fn process_input(contents: String) -> usize {
    let mut categories = contents.split(":");

    let string_seeds = categories
        .nth(1)
        .unwrap()
        .split("\n")
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| x.trim().len() != 0)
        .collect::<Vec<&str>>();

    let seed_to_soil = process_map(categories.nth(0).unwrap());

    let soil_to_fertilizer = process_map(categories.nth(0).unwrap());

    let fertilizer_to_water = process_map(categories.nth(0).unwrap());

    let water_to_light = process_map(categories.nth(0).unwrap());

    let light_to_temperature = process_map(categories.nth(0).unwrap());
    let temperature_to_humidity = process_map(categories.nth(0).unwrap());

    let humidity_to_location = process_map(categories.nth(0).unwrap());

    let seeds = string_seeds
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .flat_map(|(num1, num2)| {
            let first = num1.trim().parse::<usize>().unwrap();
            let second = num2.trim().parse::<usize>().unwrap();
            return first..first + second;
        });

    println!("Looping numbers");
    let blah = seeds.reduce(|acc, curr| {
        let soil = process_seed(&curr, &seed_to_soil);
        let fertilizer = process_seed(&soil, &soil_to_fertilizer);
        let water = process_seed(&fertilizer, &fertilizer_to_water);
        let light = process_seed(&water, &water_to_light);
        let temp = process_seed(&light, &light_to_temperature);
        let humidity = process_seed(&temp, &temperature_to_humidity);
        let location = process_seed(&humidity, &humidity_to_location);

        if location < acc {
            return curr;
        }
        return acc;
    });

    return blah.unwrap();
}

struct F<'a> {
    isFound: bool,
    newSeed: &'a usize,
}

fn process_seed(seed_number: &usize, map: &Map) -> usize {
    let r = F {
        isFound: false,
        newSeed: &0,
    };
    let new_seed = map
        .lines
        .iter()
        .fold(r, |acc, curr| {
            if acc.isFound {
                return acc;
            }
            let new_val = curr.transform_index(seed_number);
            if new_val != *seed_number {
                return F {
                    isFound: true,
                    newSeed: seed_number,
                };
            }
            return acc;
        })
        .newSeed;
    if new_seed == &0 {
        return seed_number.to_owned();
    }
    return new_seed.to_owned();
}

fn process_map(map_lines: &str) -> Map {
    let s = map_lines.split("\n");

    let lines = s
        .filter(|x| {
            let is_maybe_number = x.trim();
            if is_maybe_number.len() == 0 {
                return false;
            }
            if is_maybe_number[0..1].parse::<i32>().is_err() {
                return false;
            }
            return true;
        })
        .map(get_triple_from_line);
    return Map {
        lines: lines.collect::<Vec<MapLine>>(),
    };
}

//12 82 2 -> MapLine
fn get_triple_from_line(line: &str) -> MapLine {
    let numbers: Vec<usize> = line
        .split(" ")
        .map(|x| x.trim())
        .map(|x| x.parse::<usize>())
        .filter(|x| Result::is_ok(&x))
        .map(|x| x.unwrap())
        .collect();

    if numbers.len() >= 3 {
        return MapLine {
            dest_range_start: numbers[0],
            src_range_start: numbers[1],
            range: numbers[2],
        };
    } else {
        // Handle the case where there are not enough numbers in the line
        panic!("Invalid input format in line: {}", line);
    }
}

impl MapLine {
    /** Transforms the seed */
    pub fn transform_index(&self, seed_number: &usize) -> usize {
        let seed_range = self.create_range();

        if seed_range.contains(&seed_number) {
            let diff = seed_number - self.src_range_start;
            let range = self.dest_range_start + diff;
            return range;
        }

        return *seed_number;
    }

    /** Checks if mapline affects seed */
    pub fn is_seed(&self, seed_number: &usize) -> bool {
        let number = self.transform_index(&seed_number);
        return number != *seed_number;
    }

    fn create_range(&self) -> std::ops::Range<usize> {
        return self.src_range_start..self.src_range_start + self.range;
    }
}

// #[derive(Debug)]
struct MapLine {
    dest_range_start: usize,
    src_range_start: usize,
    range: usize,
}

// #[derive(Debug)]
struct Map {
    lines: Vec<MapLine>,
}
