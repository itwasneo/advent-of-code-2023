use num::Integer;

const TILE_COLS: usize = 140;
const TILE_ROWS: usize = 140;

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

fn part_2(_path: Vec<(usize, usize)>) {
    println!("{:?}", "<RESULT>");
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_10.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
