// use std::env;
use std::fs;
mod day_one;

fn main() {
    // Read in file data
    let ref file_path = "data/input_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    // Start by splitting contexts by newlines
    let file_lines = contents.split("\n").collect::<Vec<&str>>();

    let mut total = 0;

    for (idx, cur_line) in file_lines.iter().enumerate() {
        println!("-------------------------------------------------------------------------------");
        println!("Current line = {}", cur_line);

        if "" != cur_line.to_string() {
            let config_val = day_one::get_part_2_config_value(cur_line);

            total += config_val;

            println!(
                "line = {}, config_val = {}, total = {}",
                idx, config_val, total
            );
        }

        // println!("Wating to start again: ");
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).expect("Huh?");
    }
    println!("Final Total = {}", total);
}
