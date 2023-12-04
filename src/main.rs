// use std::env;
use std::fs;
// mod day_one;

const NUM_STRS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUM_CHARS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
// const MAX_DIGIT_LEN: i32 = 5;

fn main() {
    // Read in file data
    let ref file_path = "data/input_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    // Start by splitting contexts by newlines
    let file_lines = contents.split("\n").collect::<Vec<&str>>();

    // pull out a single line
    let first_line = &file_lines[0];


    // Need to reference string as bytes (making assumption that it's all ascii)
    let line_bytes = first_line.as_bytes();

    // Setup internal trackers for digits and indexing
    let mut first_digit = '\0';
    let mut second_digit;

    let mut cur_char_hist = 0;

    for idx in 0..line_bytes.len(){
        println!("-------------------------------------------------------");
        println!("cur line = {}", first_line);

        // setup temp strings and characters based on current indexes
        let tmp_char = line_bytes[idx] as char;
        let tmp_str = String::from_utf8((line_bytes[idx - cur_char_hist..idx]).to_vec()).unwrap();

        let mut tmp_digit = '\0';

        println!("idx = {}", idx);
        println!("cur_char_hist = {}", cur_char_hist);
        println!("tmp_str = {}", tmp_str);
        println!("tmp_char = {}", tmp_char);

        // figure out if tmp_str is one of the numeric digits
        let is_str = NUM_STRS.iter().position(|&r| r == tmp_str);

        // check current character for numeric
        if tmp_char.is_numeric() {
            println!("Current char is numeric");
            tmp_digit = tmp_char;
            cur_char_hist = 1;
        } else if is_str.is_some() {
            println!("Current String is valid");
            cur_char_hist = 0;
            tmp_digit = NUM_CHARS[is_str.unwrap()];
        } else {
            cur_char_hist += 1;
        }

        if tmp_digit != '\0' {
            println!("Found valid digit to use");
            if first_digit == '\0' {
                first_digit = tmp_digit;
            }
            second_digit = tmp_digit;

            println!(
                "first digit = {}, second digit = {}",
                first_digit, second_digit
            );
        }
    }
}
