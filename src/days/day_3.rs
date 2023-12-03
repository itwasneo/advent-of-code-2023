use std::collections::HashMap;

pub fn solve() {
    let content = read_input();

    // Creating a 2D matrix from the input ------------------------------------
    let lines: Vec<&str> = content.trim().split('\n').collect();
    let number_of_rows = lines.len();
    let number_of_cols = lines[0].len();
    let mut matrix: Vec<Vec<char>> = vec![vec!['*'; number_of_cols]; number_of_rows];

    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        for (j, &ch) in chars.iter().enumerate() {
            matrix[i][j] = ch;
        }
    }
    // ------------------------------------------------------------------------

    let mut possible_machine_part: Vec<char> = vec![];
    let mut is_machine_part = false;
    let mut result = 0;

    // Traverse the inside the border
    for i in 0..number_of_rows {
        for j in 0..number_of_cols {
            let current_element = matrix[i][j];
            if current_element.is_numeric() {
                if j > 0 && !matrix[i][j - 1].is_numeric() && !matrix[i][j - 1].eq(&'.') // LEFT
                    || j < number_of_cols - 1 && !matrix[i][j + 1].is_numeric() && !matrix[i][j + 1].eq(&'.') // RIGHT
                    || i > 0 && j < number_of_cols - 1 && !matrix[i - 1][j + 1].is_numeric() && !matrix[i - 1][j + 1].eq(&'.') // UP-RIGHT
                    || i < number_of_rows - 1 && j < number_of_cols - 1 &&!matrix[i + 1][j + 1].is_numeric() && !matrix[i + 1][j + 1].eq(&'.') // DOWN-RIGHT
                    || i > 0 && j > 0 && !matrix[i - 1][j - 1].is_numeric() && !matrix[i - 1][j - 1].eq(&'.') // UP-LEFT
                    || i < number_of_rows - 1 && j > 0 && !matrix[i + 1][j - 1].is_numeric() && !matrix[i + 1][j - 1].eq(&'.') // DOWN-LEFT
                    || i < number_of_rows - 1 && !matrix[i + 1][j].is_numeric() && !matrix[i + 1][j].eq(&'.') // DOWN
                    || i > 0 && !matrix[i - 1][j].is_numeric() && !matrix[i - 1][j].eq(&'.')
                // UP
                {
                    is_machine_part = true;
                }
                possible_machine_part.push(current_element);
            } else {
                if is_machine_part {
                    let str_representation: String = possible_machine_part.iter().collect();
                    let u32_value = str_representation.parse::<u32>().unwrap();
                    result += u32_value;
                }
                is_machine_part = false;
                possible_machine_part.clear();
            }
        }
        if is_machine_part {
            let str_representation: String = possible_machine_part.iter().collect();
            let u32_value = str_representation.parse::<u32>().unwrap();
            result += u32_value;
        }
        is_machine_part = false;
        possible_machine_part.clear();
    }

    println!("Part 1: {result}");
    part_2(&matrix, number_of_rows, number_of_cols);
}
fn part_2(matrix: &Vec<Vec<char>>, number_of_rows: usize, number_of_cols: usize) {
    let mut possible_gear_part: Vec<char> = vec![];
    let mut result = 0;
    let mut ast_map = HashMap::<(usize, usize), u32>::new();

    let mut ast_location: Option<(usize, usize)> = None;
    // Traverse the inside the border
    for i in 0..number_of_rows {
        for j in 0..number_of_cols {
            let current_element = matrix[i][j];
            if current_element.is_numeric() {
                if j > 0 && matrix[i][j - 1].eq(&'*') {
                    // LEFT
                    ast_location = Some((i, j - 1));
                } else if j < number_of_cols - 1 && matrix[i][j + 1].eq(&'*') {
                    // RIGHT
                    ast_location = Some((i, j + 1));
                } else if i > 0 && j < number_of_cols - 1 && matrix[i - 1][j + 1].eq(&'*') {
                    // UP-RIGHT
                    ast_location = Some((i - 1, j + 1));
                } else if i < number_of_rows - 1
                    && j < number_of_cols - 1
                    && matrix[i + 1][j + 1].eq(&'*')
                {
                    // DOWN-RIGHT
                    ast_location = Some((i + 1, j + 1));
                } else if i > 0 && j > 0 && matrix[i - 1][j - 1].eq(&'*') {
                    // UP-LEFT
                    ast_location = Some((i - 1, j - 1));
                } else if i < number_of_rows - 1 && j > 0 && matrix[i + 1][j - 1].eq(&'*') {
                    // DOWN-LEFT
                    ast_location = Some((i + 1, j - 1));
                } else if i < number_of_rows - 1 && matrix[i + 1][j].eq(&'*') {
                    // DOWN
                    ast_location = Some((i + 1, j));
                } else if i > 0 && matrix[i - 1][j].eq(&'*') {
                    // UP
                    ast_location = Some((i - 1, j));
                }
                possible_gear_part.push(current_element);
            } else {
                if let Some(ast_location) = ast_location {
                    let str_representation: String = possible_gear_part.iter().collect();
                    let u32_value = str_representation.parse::<u32>().unwrap();
                    if let Some(prev) = ast_map.get(&ast_location) {
                        result += u32_value * prev;
                    } else {
                        ast_map.insert(ast_location, u32_value);
                    }
                }
                ast_location = None;
                possible_gear_part.clear();
            }
        }
        if let Some(ast_location) = ast_location {
            let str_representation: String = possible_gear_part.iter().collect();
            let u32_value = str_representation.parse::<u32>().unwrap();
            if let Some(prev) = ast_map.get(&ast_location) {
                result += u32_value * prev;
            } else {
                ast_map.insert(ast_location, u32_value);
            }
        }
        ast_location = None;
        possible_gear_part.clear();
    }

    println!("Part 2: {result}");
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_3.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
