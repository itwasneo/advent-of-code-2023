pub fn solve() {
    let content = read_input();
    let result = content
        .lines()
        .map(|line| {
            let lf = line
                .chars()
                .filter(|c| c.is_numeric())
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10;
            let rf = line
                .chars()
                .rev()
                .filter(|c| c.is_numeric())
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap();
            lf + rf
        })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(input: String) {
    let result = input
        .lines()
        .map(|line| {
            let left = {
                let mut itr = line.chars().enumerate();
                let mut lf = 0;
                let left_most_index = line.len();
                while let Some((i, c)) = itr.next() {
                    if c.is_numeric() {
                        if i < left_most_index {
                            lf = c.to_digit(10).unwrap();
                            break;
                        }
                    } else {
                        match c {
                            'o' => {
                                if line.chars().nth(i + 1).unwrap().eq(&'n')
                                    && line.chars().nth(i + 2).unwrap().eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 1;
                                        break;
                                    }
                                }
                            }
                            't' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'h')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'r')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 3;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'w')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'o')
                                {
                                    if i < left_most_index {
                                        lf = 2;
                                        break;
                                    }
                                }
                            }
                            'f' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'o')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'u')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'r')
                                {
                                    if i < left_most_index {
                                        lf = 4;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'v')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 5;
                                        break;
                                    }
                                }
                            }
                            's' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'x')
                                {
                                    if i < left_most_index {
                                        lf = 6;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'v')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'n')
                                {
                                    if i < left_most_index {
                                        lf = 7;
                                        break;
                                    }
                                }
                            }
                            'e' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'g')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'h')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'t')
                                {
                                    if i < left_most_index {
                                        lf = 8;
                                        break;
                                    }
                                }
                            }
                            'n' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'n')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 9;
                                        break;
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                lf
            };
            let right = {
                let mut itr = line.chars().rev().enumerate();
                let mut lf = 0;
                let len = line.len();
                let left_most_index = line.len();
                while let Some((i, c)) = itr.next() {
                    let i = len - i - 1;
                    if c.is_numeric() {
                        if i < left_most_index {
                            lf = c.to_digit(10).unwrap();
                            break;
                        }
                    } else {
                        match c {
                            'o' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'n')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 1;
                                        break;
                                    }
                                }
                            }
                            't' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'h')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'r')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 3;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'w')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'o')
                                {
                                    if i < left_most_index {
                                        lf = 2;
                                        break;
                                    }
                                }
                            }
                            'f' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'o')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'u')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'r')
                                {
                                    if i < left_most_index {
                                        lf = 4;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'v')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 5;
                                        break;
                                    }
                                }
                            }
                            's' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'x')
                                {
                                    if i < left_most_index {
                                        lf = 6;
                                        break;
                                    }
                                } else if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'v')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'n')
                                {
                                    if i < left_most_index {
                                        lf = 7;
                                        break;
                                    }
                                }
                            }
                            'e' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'g')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'h')
                                    && line.chars().nth(i + 4).unwrap_or_else(|| '#').eq(&'t')
                                {
                                    if i < left_most_index {
                                        lf = 8;
                                        break;
                                    }
                                }
                            }
                            'n' => {
                                if line.chars().nth(i + 1).unwrap_or_else(|| '#').eq(&'i')
                                    && line.chars().nth(i + 2).unwrap_or_else(|| '#').eq(&'n')
                                    && line.chars().nth(i + 3).unwrap_or_else(|| '#').eq(&'e')
                                {
                                    if i < left_most_index {
                                        lf = 9;
                                        break;
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                lf
            };
            left * 10 + right
        })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Part 2: {result}");
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_1.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
