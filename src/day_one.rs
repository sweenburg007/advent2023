//! Functions for day 1 of Advent of Code 2023

/// Get the configuration values from a provided line, will find the first
/// and last numeric character and convert those to an integer output
pub fn get_config_value(line: &str) -> i32 {
    let mut first_char = '\0';
    let mut last_char = '\0';

    for char in line.chars() {
        if char.is_numeric() {
            if first_char == '\0' {
                first_char = char;
            }
            last_char = char;
        }
    }

    if first_char == '\0' {
        println!("How did you get here?");
        0
    } else if last_char == '\0' {
        let char0 = first_char.to_string();
        let char1 = first_char.to_string();
        let num_str = char0 + &char1;
        num_str.parse::<i32>().unwrap()
    } else {
        let char0 = first_char.to_string();
        let char1 = last_char.to_string();
        let num_str = char0 + &char1;
        num_str.parse::<i32>().unwrap()
    }
}
