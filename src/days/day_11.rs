pub fn solve() {
    let content = read_input();
    let lines: Vec<&str> = content.lines().collect();
    let line_len = lines.iter().nth(0).unwrap().len();

    // Store the empty rows
    let mut empty_rows = vec![];
    lines.iter().enumerate().for_each(|(i, l)| {
        if l.chars().all(|c| c.eq(&'.')) {
            empty_rows.push(i);
        }
    });

    // Store the empty columns
    let mut empty_cols = vec![];
    for col_index in 0..line_len {
        let mut col = vec![];
        for line in &lines {
            if let Some(c) = line.chars().nth(col_index) {
                col.push(c);
            }
        }

        if col.iter().all(|c| c.eq(&'.')) {
            empty_cols.push(col_index);
        }
    }

    // Store the galaxy indices
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for r in lines.iter().enumerate() {
        for c in r.1.chars().enumerate() {
            if c.1.eq(&'#') {
                galaxies.push((r.0, c.0));
            }
        }
    }

    // Calculate the Manhattan distances between two galaxies
    // If there are any empty rows or columns between two galaxies,
    // add the expanded distance on top of the Manhattan distance.
    let mut result = 0;
    for (index, &current_galaxy) in galaxies.iter().enumerate() {
        let rest_of_vec = &galaxies[index + 1..];
        for &other_galaxy in rest_of_vec {
            let mut expand_row = 0;
            // There can be multiple empty rows and cols
            // I believe this operation can be optimized with some memoization.
            for empty_row in &empty_rows {
                if empty_row > &current_galaxy.0.min(other_galaxy.0)
                    && empty_row < &current_galaxy.0.max(other_galaxy.0)
                {
                    expand_row += 1;
                }
            }

            let mut expand_col = 0;
            for empty_col in &empty_cols {
                if empty_col > &current_galaxy.1.min(other_galaxy.1)
                    && empty_col < &current_galaxy.1.max(other_galaxy.1)
                {
                    expand_col += 1;
                }
            }

            let mut x_distance = (current_galaxy.1 as i64 - other_galaxy.1 as i64).abs();
            if expand_col > 0 {
                x_distance += expand_col;
            }
            let mut y_distance = (current_galaxy.0 as i64 - other_galaxy.0 as i64).abs();
            if expand_row > 0 {
                y_distance += expand_row;
            }
            result += x_distance + y_distance;
        }
    }

    println!("Part 1: {result}");
    part_2(content);
}

// Part 2 is pretty much the same, just change the distance expansion multiplier
// by k - 1. If it is the multiplier is 2, it becomes 1, if 10 then 9
// In this case multiplier would be 1_000_000 - 1 = 999_999
fn part_2(input: String) {
    let lines: Vec<&str> = input.lines().collect();
    let line_len = lines.iter().nth(0).unwrap().len();

    let mut empty_rows = vec![];
    lines.iter().enumerate().for_each(|(i, l)| {
        if l.chars().all(|c| c.eq(&'.')) {
            empty_rows.push(i);
        }
    });

    let mut empty_cols = vec![];
    for col_index in 0..line_len {
        let mut col = vec![];
        for line in &lines {
            if let Some(c) = line.chars().nth(col_index) {
                col.push(c);
            }
        }

        if col.iter().all(|c| c.eq(&'.')) {
            empty_cols.push(col_index);
        }
    }

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for r in lines.iter().enumerate() {
        for c in r.1.chars().enumerate() {
            if c.1.eq(&'#') {
                galaxies.push((r.0, c.0));
            }
        }
    }

    let mut result = 0;
    for (index, &current_galaxy) in galaxies.iter().enumerate() {
        let rest_of_vec = &galaxies[index + 1..];
        for &other_galaxy in rest_of_vec {
            let mut expand_row = 0;
            for empty_row in &empty_rows {
                if empty_row > &current_galaxy.0.min(other_galaxy.0)
                    && empty_row < &current_galaxy.0.max(other_galaxy.0)
                {
                    expand_row += 1;
                }
            }

            let mut expand_col = 0;
            for empty_col in &empty_cols {
                if empty_col > &current_galaxy.1.min(other_galaxy.1)
                    && empty_col < &current_galaxy.1.max(other_galaxy.1)
                {
                    expand_col += 1;
                }
            }

            // Care the casting of i64
            let mut x_distance = (current_galaxy.1 as i64 - other_galaxy.1 as i64).abs();
            if expand_col > 0 {
                x_distance += expand_col * 999999;
            }
            let mut y_distance = (current_galaxy.0 as i64 - other_galaxy.0 as i64).abs();
            if expand_row > 0 {
                y_distance += expand_row * 999999;
            }
            result += x_distance + y_distance;
        }
    }

    println!("Part 2: {result}");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_11.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
