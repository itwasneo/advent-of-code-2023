pub fn solve() {
    let content = read_input();
    let result = content
        .split("\n\n")
        .enumerate()
        .map(|chunk| {
            //println!("Checking chunk {}", chunk.0);
            println!("{}", chunk.1);
            let chunk_lines: Vec<&str> = chunk.1.lines().collect();
            let n = chunk_lines.len();

            let mut u = 0;
            let mut d = 0;
            let mut horizontal_sym_idx = 0;
            // Check Horizontal
            for (i, line_window) in chunk_lines.windows(2).enumerate() {
                if line_window[0].eq(line_window[1]) {
                    println!("{} is equal to {}", line_window[0], line_window[1]);
                    horizontal_sym_idx = i;
                    u = i - 1;
                    d = i + 2;
                    break;
                }
            }

            let mut check_vertical = horizontal_sym_idx == 0;
            while u as isize >= 0 && d < n {
                if chunk_lines
                    .iter()
                    .nth(u)
                    .unwrap()
                    .eq(chunk_lines.iter().nth(d).unwrap())
                {
                    u -= 1;
                    d += 1;
                } else if u != 0 || d != n - 1 {
                    check_vertical = true;
                    break;
                } else {
                    break;
                }
            }

            // println!("Check vertical: {check_vertical}");
            if !check_vertical {
                let r = (horizontal_sym_idx + 1) * 100;
                println!(
                    "CHUNK {}, horizontal_sym_point: {}, score: {}",
                    chunk.0, horizontal_sym_idx, r
                );
                return (horizontal_sym_idx + 1) * 100;
            } else {
                // Check Vertical
                let m = chunk_lines.first().unwrap().len();
                let mut vertical_sym_idx = 0;
                for col in 0..m {
                    let mut is_sym = true;
                    for l in chunk_lines.iter() {
                        if let Some(ch_1) = l.chars().nth(col) {
                            if let Some(ch_2) = l.chars().nth(col + 1) {
                                if !ch_1.eq(&ch_2) {
                                    is_sym = false;
                                    break;
                                }
                            }
                        }
                    }
                    if is_sym {
                        if check_vertical_sym(col, m, &chunk_lines) {
                            vertical_sym_idx = col;
                            break;
                        }
                    }
                }
                println!(
                    "CHUNK {}, vertical_sym_point: {}",
                    chunk.0, vertical_sym_idx
                );
                return vertical_sym_idx + 1;
            }
        })
        .sum::<usize>();
    println!("Part 1: {result}");
    part_2(content);
}

fn check_vertical_sym(vertical_sym_idx: usize, m: usize, chunk_lines: &Vec<&str>) -> bool {
    let mut lft = vertical_sym_idx - 1;
    let mut rgt = vertical_sym_idx + 2;
    // println!("Checking col: {} and {}", lft, rgt);
    while lft as isize >= 0 && rgt < m {
        let mut is_sym = true;
        for l in chunk_lines.iter() {
            if let Some(ch_1) = l.chars().nth(lft) {
                if let Some(ch_2) = l.chars().nth(rgt) {
                    if !ch_1.eq(&ch_2) {
                        is_sym = false;
                        break;
                    }
                }
            }
        }
        if !is_sym {
            return false;
        }
        lft -= 1;
        rgt += 1;
    }

    /*
    if lft != 0 || rgt != m - 1 {
        panic!("WE HAVE A PROBLEM 2 col: {lft} and {rgt}");
    }
    */

    true
}
fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_13.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
