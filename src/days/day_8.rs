use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;
use std::ops::ControlFlow;

pub fn solve() {
    let content = read_input();

    let mut lines = content.lines();
    let movements = lines.next().unwrap();

    // Keeping destinations from certain locations inside a HashMap
    let mut left_right_map: HashMap<&str, (&str, &str)> = HashMap::new();
    lines.skip(1).for_each(|line| {
        let (key, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair[1..pair.len() - 1].split(", ").collect_tuple().unwrap();
        left_right_map.insert(key, (left, right));
    });

    let mut result = 0;

    // Staring from "AAA"
    let mut current_location = "AAA";

    // Here I used try_for_each to break from inside of a closure
    movements.chars().cycle().try_for_each(|m| {
        return match m {
            'L' => {
                result += 1;
                current_location = left_right_map.get(current_location).unwrap().0;
                if current_location.eq("ZZZ") {
                    return ControlFlow::Break(m);
                }
                ControlFlow::Continue(())
            }
            'R' => {
                result += 1;
                current_location = left_right_map.get(current_location).unwrap().1;
                if current_location.eq("ZZZ") {
                    return ControlFlow::Break(m);
                }
                ControlFlow::Continue(())
            }
            _ => ControlFlow::Continue(()),
        };
    });

    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(input: String) {
    let mut lines = input.lines();
    let movements = lines.next().unwrap();

    let mut left_right_map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut ends_with_a: Vec<&str> = vec![];
    lines.skip(1).for_each(|line| {
        let (key, pair) = line.split_once(" = ").unwrap();
        let (left, right) = pair[1..pair.len() - 1].split(", ").collect_tuple().unwrap();

        // Determining starting points while traversing the rows, and keep them
        // in a vector
        if key.ends_with("A") {
            ends_with_a.push(key);
        }

        // Again keeping destionations in a HashMap
        left_right_map.insert(key, (left, right));
    });

    // If you don't want to throw your pc out the window, set the type
    // u64. lcm will be much bigger than upper limit of u32
    let mut results: Vec<u64> = vec![];

    // If you check your input carefully, you will realize that you visit your
    // final destination in a fixed size paths which by nature converts the
    // problem into finding the lcm (least common multiplier) each path.
    for loc in ends_with_a.iter_mut() {
        let mut individual_result = 0;
        let current_location = loc;
        movements.chars().cycle().try_for_each(|m| {
            return match m {
                'L' => {
                    individual_result += 1;
                    *current_location = left_right_map.get(current_location).unwrap().0;
                    if current_location.ends_with("Z") {
                        return ControlFlow::Break(m);
                    }
                    ControlFlow::Continue(())
                }
                'R' => {
                    individual_result += 1;
                    *current_location = left_right_map.get(current_location).unwrap().1;
                    if current_location.ends_with("Z") {
                        return ControlFlow::Break(m);
                    }
                    ControlFlow::Continue(())
                }
                _ => ControlFlow::Continue(()),
            };
        });
        results.push(individual_result);
    }

    // I used a 3rd party library for the lcm calculation called "num"
    let result = results
        .iter()
        .cloned()
        .reduce(|acc, x| lcm(acc, x))
        .unwrap();
    println!("Part 2: {result}");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_8.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
