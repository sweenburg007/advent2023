use std::env;
use std::fs;
mod day_one;
mod day_two;

fn get_file(day: i32) -> String {
    let mut file_path = "";
    match day {
        1 => file_path = "data/input_1.txt",
        2 => file_path = "data/test_2.txt",
        _ => println!("Invalid day"),
    }

    file_path.to_string()
}

fn main() {
    // Setup cmd line args for which day to run
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <day>");
        return;
    }

    let day_str = &args[1].to_string();
    let day = day_str.parse::<i32>().unwrap();
    let file_path = get_file(day);
    println!("Current day = {}, file = {}", day, file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    match day {
        // ----------------------------------------------------------------------------------------
        // Day one
        1 => {
            // Start by splitting contexts by newlines
            let file_lines = contents.split("\n").collect::<Vec<&str>>();

            let mut total = 0;

            for (idx, cur_line) in file_lines.iter().enumerate() {
                println!("---------------------------------------------------------------------");
                println!("Current line = {}", cur_line);

                if "" != cur_line.to_string() {
                    let config_val = day_one::get_part_2_config_value(cur_line);

                    total += config_val;

                    println!(
                        "line = {}, config_val = {}, total = {}",
                        idx, config_val, total
                    );
                }
            }
            println!("Final Total = {}", total);
        }

        // ----------------------------------------------------------------------------------------
        // Day 2
        2 => {
            // create a vector of vectors to hold all the color values
            let mut game_values: Vec<Vec<i32>> = Vec::new();
            // get first line for testing
            let file_lines = contents.split("\n").collect::<Vec<&str>>();

            for (i, line) in file_lines.iter().enumerate() {
                if line.to_string() != "" {
                    println!("-------------------------------------------------------------");
                    println!("idx = {}", i);
                    let max_vals = day_two::get_line_max_vals(line);
                    game_values.push(max_vals);
                }
            }

            // Setup values for each of the colors
            let red_num = 12;
            let blue_num = 14;
            let green_num = 13;

            // const COLORS: [&'static str; 3] = ["red", "blue", "green"];
            let cmp_color_vec = vec![red_num, blue_num, green_num];

            // loop over each line and see if the game is valid
            for (idx, cur_vals) in game_values.iter().enumerate() {

                let x = cmp_color_vec.iter().all(|item| cur_vals.le(*item));

            }

        }

        // ----------------------------------------------------------------------------------------
        // Default edge case
        _ => println!("Invalid day provided"),
    }
}
