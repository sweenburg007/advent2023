//! Functions for day 1 of Advent of Code 2023

// constants for string based digits
const NUM_STRS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUM_CHARS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const MAX_DIGIT_LEN: usize = 5;

pub fn get_part_2_config_value(line: &str) -> i32 {
    // setup buffer vector
    let mut buf: Vec<char> = Vec::new();

    // setup variables to hold the first and second digit
    let mut first_digit = '\0';
    let mut second_digit = '\0';

    for (_idx, cur_char) in line.chars().enumerate() {
        // print current setup
        // println!("cur char = {}", cur_char);
        // println!("cur buf = {:?}", buf);
        // println!("cur buf len = {}", buf.len());

        // hold the digit here
        let mut tmp_digit = '\0';

        // double check the size of the buffer
        if buf.len() == MAX_DIGIT_LEN {
            // println!("Buffer full, removing character");
            buf.remove(0);
        }

        // check to see if the current character is numeric
        if cur_char.is_numeric() {
            // println!("Found numeric character at {}, {}", idx, cur_char);
            buf.clear();
            // println!("cleared buf = {:?}", buf);
            tmp_digit = cur_char;
        } else {
            // add value to the vector
            buf.push(cur_char);
            // println!("updated buf = {:?}", buf);

            // test the buffer over each of the strings
            for (num_idx, num_str) in NUM_STRS.iter().enumerate() {
                // check to make sure that the number string is <= current buffer length
                if num_str.len() <= buf.len() {
                    // get the offset idx
                    let start = buf.len() - num_str.len();

                    // convert the buffer to a single string for comparison
                    let buf_str = String::from_iter(&buf[start..buf.len()]);

                    if buf_str == num_str.to_string() {
                        // println!("Found valid digit str!!!!, {}", NUM_CHARS[num_idx]);
                        let cur_end_buff = buf.last().copied().unwrap();
                        buf.clear();
                        buf.push(cur_end_buff);
                        tmp_digit = NUM_CHARS[num_idx];
                        break;
                    }
                }
            }
        }

        // update values if found a new digit
        if tmp_digit != '\0' {
            if first_digit == '\0' {
                first_digit = tmp_digit;
            }
            second_digit = tmp_digit;
        }
    }

    println!(
        "first_digit = {}, second_digit = {}",
        first_digit, second_digit
    );

    let char0 = first_digit.to_string();
    let char1 = second_digit.to_string();
    let final_num_str = char0 + &char1;
    final_num_str.parse::<i32>().unwrap()
}

// Get the configuration values from a provided line, will find the first
// and last numeric character and convert those to an integer output
// pub fn get_config_value_part_1(line: &str) -> i32 {
//     let mut first_char = '\0';
//     let mut last_char = '\0';
//
//     for char in line.chars() {
//         if char.is_numeric() {
//             if first_char == '\0' {
//                 first_char = char;
//             }
//             last_char = char;
//         }
//     }
//
//     if first_char == '\0' {
//         println!("How did you get here?");
//         0
//     } else if last_char == '\0' {
//         let char0 = first_char.to_string();
//         let char1 = first_char.to_string();
//         let num_str = char0 + &char1;
//         num_str.parse::<i32>().unwrap()
//     } else {
//         let char0 = first_char.to_string();
//         let char1 = last_char.to_string();
//         let num_str = char0 + &char1;
//         num_str.parse::<i32>().unwrap()
//     }
// }
