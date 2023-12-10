use std::collections::HashSet;

use num::Integer;

const TILE_SIZE: usize = 140;

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
    let mut seen = HashSet::new();
    let mut path_length = 1;
    while let Some(pos) = get_next(cur, &mut seen, prev_direction, &tile_map) {
        let cur_tile = tile_map[pos.0 .0][pos.0 .1];
        if cur_tile == Tile::Start {
            break;
        }
        cur = pos.0;
        prev_direction = pos.1;
        path_length += 1;
    }
    println!("Part 1: {}", path_length.div_ceil(&2));
    part_2(content);
}

fn is_reachable(
    cur: (usize, usize),
    nxt: (isize, isize),
    dir: Direction,
    tile_map: &[[Tile; TILE_SIZE]; TILE_SIZE],
) -> bool {
    let cur = tile_map[cur.0][cur.1];
    if nxt.0 > TILE_SIZE as isize - 1 || nxt.1 > TILE_SIZE as isize - 1 || nxt.0 < 0 || nxt.1 < 0 {
        return false;
    }
    let nxt = tile_map[nxt.0 as usize][nxt.1 as usize];
    match cur {
        Tile::Start => match dir {
            Direction::Up => {
                nxt == Tile::SouthWest || nxt == Tile::Vertical || nxt == Tile::SouthEast
            }
            Direction::Down => {
                nxt == Tile::NorthWest || nxt == Tile::Vertical || nxt == Tile::NorthEast
            }
            Direction::Right => {
                nxt == Tile::SouthWest || nxt == Tile::Horizontal || nxt == Tile::NorthWest
            }
            Direction::Left => {
                nxt == Tile::NorthEast || nxt == Tile::Horizontal || nxt == Tile::SouthEast
            }
        },
        Tile::Vertical => match dir {
            Direction::Left | Direction::Right => false,
            Direction::Up => {
                nxt == Tile::SouthWest || nxt == Tile::Vertical || nxt == Tile::SouthEast
            }
            Direction::Down => {
                nxt == Tile::NorthWest || nxt == Tile::Vertical || nxt == Tile::NorthEast
            }
        },
        Tile::Horizontal => match dir {
            Direction::Up | Direction::Down => false,
            Direction::Right => {
                nxt == Tile::SouthWest || nxt == Tile::Horizontal || nxt == Tile::NorthWest
            }
            Direction::Left => {
                nxt == Tile::NorthEast || nxt == Tile::Horizontal || nxt == Tile::SouthEast
            }
        },
        Tile::NorthEast => match dir {
            Direction::Down | Direction::Left => false,
            Direction::Right => {
                nxt == Tile::SouthWest || nxt == Tile::Horizontal || nxt == Tile::NorthWest
            }
            Direction::Up => {
                nxt == Tile::SouthWest || nxt == Tile::Vertical || nxt == Tile::SouthEast
            }
        },
        Tile::NorthWest => match dir {
            Direction::Down | Direction::Right => false,
            Direction::Left => {
                nxt == Tile::NorthEast || nxt == Tile::Horizontal || nxt == Tile::SouthEast
            }
            Direction::Up => {
                nxt == Tile::SouthWest || nxt == Tile::Vertical || nxt == Tile::SouthEast
            }
        },
        Tile::SouthWest => match dir {
            Direction::Up | Direction::Right => false,
            Direction::Down => {
                nxt == Tile::NorthWest || nxt == Tile::Vertical || nxt == Tile::NorthEast
            }
            Direction::Left => {
                nxt == Tile::NorthEast || nxt == Tile::Horizontal || nxt == Tile::SouthEast
            }
        },
        Tile::SouthEast => match dir {
            Direction::Up | Direction::Left => false,
            Direction::Right => {
                nxt == Tile::SouthWest || nxt == Tile::Horizontal || nxt == Tile::NorthWest
            }
            Direction::Down => {
                nxt == Tile::NorthWest || nxt == Tile::Vertical || nxt == Tile::NorthEast
            }
        },
        _ => false,
    }
}

fn get_next(
    cur: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
    prev_move: Direction,
    tile_map: &[[Tile; TILE_SIZE]; TILE_SIZE],
) -> Option<((usize, usize), Direction)> {
    let cur_tile = tile_map[cur.0][cur.1];
    match cur_tile {
        Tile::Vertical => {
            if prev_move == Direction::Up
                && is_reachable(
                    cur,
                    (cur.0 as isize - 1, cur.1 as isize),
                    Direction::Up,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            } else if prev_move == Direction::Down
                && is_reachable(
                    cur,
                    (cur.0 as isize + 1, cur.1 as isize),
                    Direction::Down,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else {
                None
            }
        }
        Tile::Horizontal => {
            if prev_move == Direction::Right
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize + 1),
                    Direction::Right,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            } else if prev_move == Direction::Left
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize - 1),
                    Direction::Left,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::NorthEast => {
            if prev_move == Direction::Down
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize + 1),
                    Direction::Right,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            } else if prev_move == Direction::Left
                && is_reachable(
                    cur,
                    (cur.0 as isize - 1, cur.1 as isize),
                    Direction::Up,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            }
            None
        }
        Tile::NorthWest => {
            if prev_move == Direction::Right
                && is_reachable(
                    cur,
                    (cur.0 as isize - 1, cur.1 as isize),
                    Direction::Up,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 - 1, cur.1), Direction::Up));
            } else if prev_move == Direction::Down
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize - 1),
                    Direction::Left,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::SouthWest => {
            if prev_move == Direction::Right
                && is_reachable(
                    cur,
                    (cur.0 as isize + 1, cur.1 as isize),
                    Direction::Down,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else if prev_move == Direction::Up
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize - 1),
                    Direction::Left,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 - 1), Direction::Left));
            }
            None
        }
        Tile::SouthEast => {
            if prev_move == Direction::Left
                && is_reachable(
                    cur,
                    (cur.0 as isize + 1, cur.1 as isize),
                    Direction::Down,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0 + 1, cur.1), Direction::Down));
            } else if prev_move == Direction::Up
                && is_reachable(
                    cur,
                    (cur.0 as isize, cur.1 as isize + 1),
                    Direction::Right,
                    &tile_map,
                )
            {
                seen.insert(cur);
                return Some(((cur.0, cur.1 + 1), Direction::Right));
            }
            None
        }
        _ => None,
    }
}

fn create_tile_map(m: &str, start: &mut (usize, usize)) -> [[Tile; TILE_SIZE]; TILE_SIZE] {
    let mut tile_map = [[Tile::Ground; TILE_SIZE]; TILE_SIZE];
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

fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_10.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
