pub fn solve() {
    let content = read_input();
    let result = content
        .lines()
        .map(|line| {
            let (game, cubes) = line.split_once(":").unwrap();
            let (_, game_id) = game.split_once(" ").unwrap();
            let current_game_id = game_id.parse::<u32>().unwrap();
            let mut rounds = cubes.split(";");
            while let Some(round) = rounds.next() {
                let mut cube_infos = round.trim().split(",");
                while let Some(cube_info) = cube_infos.next() {
                    let (amount, color) = cube_info.trim().split_once(" ").unwrap();
                    if !match color {
                        "red" => amount.parse::<u32>().unwrap() <= 12,
                        "green" => amount.parse::<u32>().unwrap() <= 13,
                        "blue" => amount.parse::<u32>().unwrap() <= 14,
                        _ => false,
                    } {
                        return 0;
                    }
                }
            }
            current_game_id
        })
        .reduce(|acc, a| acc + a)
        .unwrap();
    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_2.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
