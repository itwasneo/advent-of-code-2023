use itertools::Itertools;

pub fn solve() {
    let content = read_input();
    let lines: Vec<&str> = content.lines().collect();
    let line_len = lines.iter().nth(0).unwrap().len();

    let mut empty_rows = vec![];
    lines.iter().enumerate().for_each(|(i, l)| {
        if l.chars().all(|c| c.eq(&'.')) {
            empty_rows.push(i);
        }
    });
    empty_rows.reverse();

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
    empty_cols.reverse();

    let mut new_data = vec![];
    for line in lines {
        let mut l_vec = line.chars().collect_vec();
        for empty_col in &empty_cols {
            l_vec.splice(empty_col..empty_col, vec!['.']);
        }
        new_data.push(l_vec);
    }

    let empty_line = vec!['.'; new_data.last().unwrap().len()];

    for empty_row in empty_rows {
        new_data.splice(empty_row..empty_row, vec![empty_line.clone()]);
    }

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for r in new_data.iter().enumerate() {
        for c in r.1.iter().enumerate() {
            if c.1.eq(&'#') {
                galaxies.push((r.0, c.0));
            }
        }
    }

    /*
    for r in new_data {
        println!("{:?}", r);
    }
        */
    //println!("{:?}", empty_cols);
    //println!("{:?}", galaxies);
    let mut result = 0;
    for (index, &current_galaxy) in galaxies.iter().enumerate() {
        let rest_of_vec = &galaxies[index + 1..]; // Get the remaining elements after the current index

        // Perform calculation with the current element and the rest of the vector
        for &other_galaxy in rest_of_vec {
            let distance = (current_galaxy.0 as i32 - other_galaxy.0 as i32).abs()
                + (current_galaxy.1 as i32 - other_galaxy.1 as i32).abs();
            result += distance;
        }
    }

    println!("{result}");
    println!("Part 1: {}", "<RESULT>");
    part_2(content);
}
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
        let rest_of_vec = &galaxies[index + 1..]; // Get the remaining elements after the current index

        // Perform calculation with the current element and the rest of the vector
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

            let mut x_distance = (current_galaxy.1 as i64 - other_galaxy.1 as i64).abs();
            if expand_col > 0 {
                x_distance += expand_col * 999999;
            }
            let mut y_distance = (current_galaxy.0 as i64 - other_galaxy.0 as i64).abs();
            if expand_row > 0 {
                y_distance += expand_row * 999999;
            }
            let tmp = x_distance + y_distance;
            result += x_distance + y_distance;
        }
    }

    println!("{:?}", galaxies.len());
    println!("{:?}", result);
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_11.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
