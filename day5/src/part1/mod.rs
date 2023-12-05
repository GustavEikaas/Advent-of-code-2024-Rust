pub fn part1(contents: String) {
    let processed = process_input(contents);
    println!("Result: {}", processed);
}

fn process_input(contents: String) -> usize {
    let mut categories = contents.split(":");

    let seeds = categories
        .nth(1)
        .unwrap()
        .split("\n")
        .nth(0)
        .unwrap()
        .split(" ")
        .filter(|x| x.trim().len() != 0)
        .map(|x| x.trim().parse::<usize>().unwrap());

    let seed_to_soil = process_map(categories.nth(0).unwrap(), "seed-to-soil".to_string());

    let soil_to_fertilizer =
        process_map(categories.nth(0).unwrap(), "soil-to-fertilizer".to_string());

    let fertilizer_to_water = process_map(
        categories.nth(0).unwrap(),
        "fertilizer-to-water".to_string(),
    );

    let water_to_light = process_map(categories.nth(0).unwrap(), "water-to-light".to_string());

    let light_to_temperature = process_map(
        categories.nth(0).unwrap(),
        "light-to-temperature".to_string(),
    );

    let temperature_to_humidity = process_map(
        categories.nth(0).unwrap(),
        "temperature-to-humidity".to_string(),
    );

    let humidity_to_location = process_map(
        categories.nth(0).unwrap(),
        "humidity-to-location".to_string(),
    );

    let locations = seeds.map(|seed_number| {
        let soil = process_seed(&seed_number, &seed_to_soil);
        let fertilizer = process_seed(&soil, &soil_to_fertilizer);
        let water = process_seed(&fertilizer, &fertilizer_to_water);
        let light = process_seed(&water, &water_to_light);
        let temp = process_seed(&light, &light_to_temperature);
        let humidity = process_seed(&temp, &temperature_to_humidity);
        let location = process_seed(&humidity, &humidity_to_location);
        return location;
    });

    return locations.min().unwrap();
}

fn process_seed(seed_number: &usize, map: &Map) -> usize {
    return map
        .lines
        .iter()
        .find(|x| x.is_seed(seed_number))
        .and_then(|map_line| return Some(map_line.transform_index(seed_number)))
        .unwrap_or(*seed_number);
}

fn process_map(map_lines: &str, _name: String) -> Map {
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
            return self.dest_range_start + diff;
        }

        return seed_number.to_owned();
    }

    /** Checks if mapline affects seed */
    pub fn is_seed(&self, seed_number: &usize) -> bool {
        let number = self.transform_index(&seed_number);
        return number != seed_number.to_owned();
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
