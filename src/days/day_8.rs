use std::collections::HashMap;

use itertools::Itertools;

pub fn solve() {
    let content = read_input();

    let mut lines = content.lines();
    let movements = lines.next().unwrap();

    let mut left_right_map: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.skip(1).for_each(|line| {
        let (key, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair[1..pair.len() - 1].split(", ").collect_tuple().unwrap();
        left_right_map.insert(key, (left, right));
    });

    let mut result = 0;
    let mut current_location = "AAA";
    movements.chars().cycle().try_for_each(|m| {
        return match m {
            'L' => {
                result += 1;
                current_location = left_right_map.get(current_location).unwrap().0;
                if current_location.eq("ZZZ") {
                    None
                } else {
                    Some(())
                }
            }
            'R' => {
                result += 1;
                current_location = left_right_map.get(current_location).unwrap().1;
                if current_location.eq("ZZZ") {
                    None
                } else {
                    Some(())
                }
            }
            _ => None,
        };
    });

    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_8.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
