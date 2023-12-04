use std::collections::HashSet;

pub fn solve() {
    let content = read_input();
    let result = content
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let mut winner_numbers = HashSet::<&str>::new();
            let mut game_numbers = HashSet::<&str>::new();
            let (w, m) = numbers.trim().split_once(" | ").unwrap();
            winner_numbers.extend(w.trim().split_whitespace());
            game_numbers.extend(m.trim().split_whitespace());
            let matches = winner_numbers.intersection(&game_numbers).count();

            if matches > 0 {
                i32::pow(2, (matches - 1) as u32)
            } else {
                0
            }
        })
        .reduce(|acc, a| acc + a)
        .unwrap();
    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(input: String) {
    let mut mem = [1_u32; 205];
    mem[0] = 1;
    input.lines().for_each(|line| {
        let (game, numbers) = line.split_once(": ").unwrap();
        let id = str::parse::<usize>(game.split_whitespace().nth(1).unwrap()).unwrap();
        let mut winner_numbers = HashSet::<&str>::new();
        let mut game_numbers = HashSet::<&str>::new();
        let (w, m) = numbers.trim().split_once(" | ").unwrap();
        winner_numbers.extend(w.trim().split_whitespace());
        game_numbers.extend(m.trim().split_whitespace());
        let matches = winner_numbers.intersection(&game_numbers).count();
        let clamped_end = (id + matches).min(205);
        let add_amount = mem[id - 1];
        mem[id..clamped_end]
            .iter_mut()
            .for_each(|m| *m += add_amount);
    });
    let result = mem.iter().sum::<u32>();
    println!("Part 2: {result}");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_4.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
