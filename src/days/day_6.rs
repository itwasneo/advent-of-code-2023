use itertools::Itertools;

pub fn solve() {
    let content = read_input();
    let (times_raw, dists_raw) = content.split_once("\n").unwrap();

    // Creating an iterator for Times
    let times = times_raw
        .split_whitespace()
        .skip(1)
        .map(|s| str::parse::<i32>(s).unwrap());

    // Creating an iterator for Distances
    let dists = dists_raw
        .split_whitespace()
        .skip(1)
        .map(|s| -(str::parse::<i32>(s).unwrap()));

    // The problem by nature becomes finding the roots of a quadratic function.
    // In order to find the roots of the function we can apply the following
    // formula where b is TIME and c is -DISTANCE (ps. Care the minus sign here)
    // x = (-b ± √ (b2 - 4ac) )/2a
    let result = times
        .zip(dists)
        .map(|(b, c)| {
            // We can calculate this part once
            let com = (b * b + 4 * c) as f64;

            // First root
            let x_1 = (-b as f64 + com.sqrt()) / -2_f64;

            // Second root
            let x_2 = (-b as f64 - com.sqrt()) / -2_f64;

            // For the left-side of the range we should find the closest bigger
            // integer. If the root is already an integer we should take the
            // next integer
            let l = if x_1.fract() == 0.0 {
                x_1 as i64 + 1
            } else {
                // Round up with ceil
                x_1.ceil() as i64
            };
            let r = if x_2.fract() == 0.0 {
                x_2 as i64 - 1
            } else {
                // Round down with floor
                x_2.floor() as i64
            };
            r - l + 1
        })
        .reduce(|acc, a| acc * a)
        .unwrap();

    println!("Part 1: {result}");
    part_2(content);
}

// Second part is basically the same calculation executed just once.
fn part_2(input: String) {
    let (times_raw, dists_raw) = input.split_once("\n").unwrap();
    let time = str::parse::<f64>(&times_raw.split_whitespace().skip(1).join("")).unwrap();
    let dist = -str::parse::<f64>(&dists_raw.split_whitespace().skip(1).join("")).unwrap();
    let com = (time * time + 4_f64 * dist) as f64;
    let x_1 = (-time + com.sqrt()) / -2_f64;
    let x_2 = (-time - com.sqrt()) / -2_f64;

    let l = if x_1.fract() == 0.0 {
        x_1 as i64 + 1
    } else {
        x_1.ceil() as i64
    };
    let r = if x_2.fract() == 0.0 {
        x_2 as i64 - 1
    } else {
        x_2.floor() as i64
    };
    let result = r - l + 1;
    println!("Part 2: {result}");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_6.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
