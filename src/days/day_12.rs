pub fn solve() {
    let content = read_input();

    gpt();

    println!("Part 1: {}", "<RESULT>");
    part_2(content);
}

fn count_combinations(
    pattern: &[char],
    lengths: &[usize],
    index: usize,
    current_segment: usize,
    question_marks: usize,
) -> usize {
    if index == pattern.len() {
        return if current_segment == lengths.len() && question_marks > 0 {
            1
        } else {
            0
        };
    }

    if current_segment >= lengths.len() {
        return 0;
    }

    let mut combinations = 0;

    if pattern[index] == '?' {
        if lengths[current_segment] > 0 {
            combinations += count_combinations(
                pattern,
                lengths,
                index + 1,
                current_segment,
                question_marks - 1,
            );
        }

        if current_segment < lengths.len() - 1 {
            combinations += count_combinations(
                pattern,
                lengths,
                index + 1,
                current_segment + 1,
                question_marks,
            );
        }
    } else {
        if lengths[current_segment] > 0 {
            combinations +=
                count_combinations(pattern, lengths, index + 1, current_segment, question_marks);
        }

        if index < pattern.len() - 1 {
            combinations += count_combinations(
                pattern,
                lengths,
                index + 1,
                current_segment + 1,
                question_marks,
            );
        }
    }

    combinations
}

fn gpt() {
    let patterns = vec![
        "???.###",
        ".??..??...?##.",
        "?#?#?#?#?#?#?#?",
        "????.#...#...",
        "????.######..#####.",
        "?###????????",
    ];
    let lengths = vec![
        vec![1, 1, 3],
        vec![1, 1, 3],
        vec![1, 3, 1, 6],
        vec![4, 1, 1],
        vec![1, 6, 5],
        vec![3, 2, 1],
    ];

    for (pattern_str, length) in patterns.iter().zip(lengths.iter()) {
        let pattern: Vec<char> = pattern_str.chars().collect();
        let combinations = count_combinations(
            &pattern,
            length,
            0,
            0,
            pattern_str.chars().filter(|&c| c == '?').count(),
        );
        println!("Pattern: {:?}, Combinations: {}", pattern, combinations);
    }
}

fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_12_sample.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
