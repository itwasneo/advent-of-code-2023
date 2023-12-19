use std::collections::HashSet;

use num::Integer;

const TILE_COLS: usize = 20;
const TILE_ROWS: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn solve() {
    let content = read_input();
    let mut start = (0, 0);
    let tile_map = create_tile_map(&content, &mut start);

    let mut cur = (start.0 + 1, start.1);
    let mut prev_direction = Direction::Down;
    let mut path_length = 1;
    let mut pipe_loop = vec![start];
    while let Some(pos) = get_next(cur, prev_direction, &tile_map) {
        let cur_tile = tile_map[pos.0 .0][pos.0 .1];
        pipe_loop.push(cur);
        path_length += 1;
        if cur_tile == Tile::Start {
            break;
        }
        cur = pos.0;
        prev_direction = pos.1;
    }
    println!("Part 1: {}", path_length.div_ceil(&2));
    part_2(pipe_loop);
}

fn get_next(
    cur: (usize, usize),
    prev_move: Direction,
    tile_map: &[[Tile; TILE_COLS]; TILE_ROWS],
) -> Option<((usize, usize), Direction)> {
    let cur_tile = tile_map[cur.0][cur.1];
    match cur_tile {
        Tile::Vertical => {
            if prev_move == Direction::Up {
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            } else if prev_move == Direction::Down {
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else {
                None
            }
        }
        Tile::Horizontal => {
            if prev_move == Direction::Right {
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            } else if prev_move == Direction::Left {
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::NorthEast => {
            if prev_move == Direction::Down {
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            } else if prev_move == Direction::Left {
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            }
            None
        }
        Tile::NorthWest => {
            if prev_move == Direction::Right {
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            } else if prev_move == Direction::Down {
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::SouthWest => {
            if prev_move == Direction::Right {
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else if prev_move == Direction::Up {
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::SouthEast => {
            if prev_move == Direction::Left {
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else if prev_move == Direction::Up {
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            }
            None
        }
        _ => None,
    }
}

fn create_tile_map(m: &str, start: &mut (usize, usize)) -> [[Tile; TILE_COLS]; TILE_ROWS] {
    let mut tile_map = [[Tile::Ground; TILE_COLS]; TILE_ROWS];
    m.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '|' => tile_map[i][j] = Tile::Vertical,
            '-' => tile_map[i][j] = Tile::Horizontal,
            'L' => tile_map[i][j] = Tile::NorthEast,
            'J' => tile_map[i][j] = Tile::NorthWest,
            '7' => tile_map[i][j] = Tile::SouthWest,
            'F' => tile_map[i][j] = Tile::SouthEast,
            '.' => {}
            'S' => {
                tile_map[i][j] = Tile::Start;
                *start = (i, j);
            }
            _ => panic!("UNKNOWN TILE TYPE"),
        })
    });
    tile_map
}

fn part_2(path: Vec<(usize, usize)>) {
    let min_x = path.iter().map(|x| x.1).min().unwrap();
    let min_y = path.iter().map(|x| x.0).min().unwrap();
    let max_x = path.iter().map(|x| x.1).max().unwrap();
    let max_y = path.iter().map(|x| x.0).max().unwrap();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let border: HashSet<(usize, usize)> = HashSet::from_iter(path.into_iter());
    let mut inside: HashSet<(usize, usize)> = HashSet::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !border.contains(&(y, x))
                && dfs((y, x), &border, &mut visited, min_x, min_y, max_x, max_y)
            {
                inside.insert((y, x));
            }
        }
    }

    //println!("min_x: {min_x}, max_x: {max_x}, min_y: {min_y}, max_y: {max_y}");
    println!(
        "inside_len: {:?}, border_len: {:?}, visited_len: {:?}",
        inside.len(),
        border.len(),
        visited.len()
    );
    //println!("{:?}", inside.len());
    println!("{:?}", "<RESULT>");
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn dfs(
    next: (usize, usize),
    border: &HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> bool {
    if next.1 < min_x || next.1 > max_x || next.0 < min_y || next.0 > max_y {
        return false;
    }

    if border.contains(&next) {
        return true;
    }

    visited.insert(next);
    let mut is_inside = false;
    for i in 0..4 {
        let next_1 = (
            (next.0 as isize + DIRECTIONS[i].0),
            (next.1 as isize + DIRECTIONS[i].1),
        );
        if next_1.0 >= min_y as isize
            && next_1.0 <= max_y as isize
            && next_1.1 >= min_x as isize
            && next_1.1 <= max_x as isize
        {
            is_inside = dfs(
                (next_1.0 as usize, next_1.1 as usize),
                border,
                visited,
                min_x,
                min_y,
                max_x,
                max_y,
            );
        }
    }

    return is_inside;
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_10_sample.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
