//! Functions for day 1 of Advent of Code 2023

use std::collections::linked_list;

// constants for string based digits
const NUM_STRS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn get_str_config_value(line: &str) -> i32 {
    // convert line into bytes
    let line_bytes = line.as_bytes();

    // setup first character and last character
    let mut first_char = '\0';
    let mut second_char = '\0';

    // loop over all the bytes
    for idx in 0..line_bytes.len() {}

    // FIXME: this is place holder so things will build
    0
}

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
