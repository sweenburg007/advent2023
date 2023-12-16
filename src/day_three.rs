pub struct PartData {
    val: i32,
    val_str: String,
    row: i32,
    start_col: i32,
    end_col: i32,
}

pub fn get_line_part_data(row: i32, line_chars: Vec<char>) -> PartData {

    let mut part_num_buff = Vec<char>::new();

    for ( idx, cur_char ) in line_chars.iter().enumerate() {

    }


    PartData {
        val: 1,
        val_str: "",
        row: 1,
        start_col: 1,
        end_col: 1,
    }
}
