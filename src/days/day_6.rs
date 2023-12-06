pub fn solve() {
    let content = read_input();
    let (times_raw, dists_raw) = content.split_once("\n").unwrap();
    let times = times_raw
        .split_whitespace()
        .skip(1)
        .map(|s| str::parse::<i32>(s).unwrap());
    let dists = dists_raw
        .split_whitespace()
        .skip(1)
        .map(|s| -(str::parse::<i32>(s).unwrap()));

    // Appliying x = (-b ± √ (b2 - 4ac) )/2a
    let result = times
        .zip(dists)
        .map(|(b, c)| {
            let com = (b * b + 4 * c) as f64;
            let x_1 = (-b as f64 + com.sqrt()) / -2_f64;
            let x_2 = (-b as f64 - com.sqrt()) / -2_f64;

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
            r - l + 1
        })
        .reduce(|acc, a| acc * a)
        .unwrap();

    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_6.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
