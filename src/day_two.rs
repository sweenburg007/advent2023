//! Functions for day 2 of Advent of Code 2023

const COLORS: [&'static str; 3] = ["red", "blue", "green"];

/// Get a string containing the different game draws
pub fn get_line_draws_str(line: &str) -> &str {
    line.split(": ").collect::<Vec<&str>>()[1]
}

/// Get each of the draws from a game
pub fn get_each_draw(game_str: &str) -> Vec<&str> {
    game_str.split("; ").collect::<Vec<&str>>()
}

pub fn get_line_draws(line: &str) -> Vec<&str> {
    get_each_draw(get_line_draws_str(line))
}

// split the draw into just the colors and numerical values "(<num> <color>)"
pub fn get_draw_colors(draw_str: &str) -> Vec<&str> {
    draw_str.split(", ").collect::<Vec<&str>>()
}

pub fn convert_draw_color_to_int(draw_color: &str) -> i32 {
    draw_color.split(" ").collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap()
}

pub fn get_draw_color_vals(draw_str: &str) -> Vec<i32> {
    let draw_colors = get_draw_colors(draw_str);

    let mut color_vals = vec![0, 0, 0];

    // loop over all the colors in this draw
    for draw_color in draw_colors {
        // loop over the colors we care about
        for (cidx, color) in COLORS.iter().enumerate() {
            if draw_color.contains(color) {
                // println!("cur draw color '{}' has color {}", draw_color, color);
                color_vals[cidx] = convert_draw_color_to_int(draw_color);
            }
        }
    }

    color_vals
}

pub fn get_line_max_vals(line: &str) -> Vec<i32> {
    let mut game_maxes = vec![0, 0, 0];

    let line_draws = get_line_draws(line);
    println!("Current game = '{:?}'", line_draws);

    for (_idx, draw) in line_draws.iter().enumerate() {
        // println!("----");
        // println!("draw = {}, game = '{}'", idx, draw);

        // get the values for each of these
        let cur_draw_values = get_draw_color_vals(draw);

        // println!(
        //     "current values: red = {}, blue = {}, green = {}",
        //     cur_draw_values[0], cur_draw_values[1], cur_draw_values[2]
        // );

        for i in 0..game_maxes.len() {
            if game_maxes[i] <= cur_draw_values[i] {
                game_maxes[i] = cur_draw_values[i];
            }
        }

        // println!(
        //     "\tafter game maxes: red= {}, blue = {}, green = {}",
        //     game_maxes[0], game_maxes[1], game_maxes[2]
        // );
    }

    println!(
        "final game maxes: red= {}, blue = {}, green = {}",
        game_maxes[0], game_maxes[1], game_maxes[2]
    );

    game_maxes
}
