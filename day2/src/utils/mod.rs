pub fn find_color(line: &str) -> String {
    return match () {
        _ if line.contains("red") => "red".to_string(),
        _ if line.contains("green") => "green".to_string(),
        _ => "blue".to_string(),
    };
}

pub fn extract_number_from_draw(curr: &str) -> Result<i32, std::num::ParseIntError> {
    return curr
        .replace("red", "")
        .replace("green", "")
        .replace("blue", "")
        .trim()
        .parse::<i32>();
}
