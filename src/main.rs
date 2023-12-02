use std::env;
use std::fs;
mod day_one;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Current running path is {}.", args[0]);
    println!(
        "Num cmdline args = {:?}, args = {:?}",
        args.len(),
        &args[1..]
    );

    // Check to make sure there are two cmd line args
    if args.len() != 3 {
        println!("Usage: ./run <day int> <file path>");
        std::process::exit(1);
    }

    // Read in file data
    let ref file_path = &args[2];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    // Start by splitting contexts by newlines
    let file_lines = contents.split("\n").collect::<Vec<&str>>();

    let mut total = 0;

    for line in file_lines {
        let config_value = day_one::get_config_value(line);

        total = total + config_value;

        println!("{} : {}: {}", total, config_value, line);
    }
}
