use itertools::Itertools;

pub fn solve() {
    let content = read_input();
    let result = content
        .lines()
        .map(|l| {
            recursion(
                l.split_whitespace()
                    .map(|n| str::parse::<i32>(n).unwrap())
                    .collect_vec(),
            )
        })
        .reduce(|acc, a| acc + a)
        .unwrap();
    println!("Part 1: {result}");
    part_2(content);
}

fn recursion(numbers: Vec<i32>) -> i32 {
    if !(numbers.iter().all(|&x| x == 0)) {
        return numbers.iter().last().unwrap()
            + recursion(
                numbers
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect_vec(),
            );
    }
    0
}

fn recursion_2(numbers: Vec<i32>) -> i32 {
    if !(numbers.iter().all(|&x| x == 0)) {
        return numbers.iter().next().unwrap()
            - recursion_2(
                numbers
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect_vec(),
            );
    }
    0
}

fn part_2(input: String) {
    let result = input
        .lines()
        .map(|l| {
            recursion_2(
                l.split_whitespace()
                    .map(|n| str::parse::<i32>(n).unwrap())
                    .collect_vec(),
            )
        })
        .reduce(|acc, a| acc + a)
        .unwrap();
    println!("Part 2: {result}");
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_9.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
