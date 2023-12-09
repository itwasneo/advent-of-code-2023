use std::collections::BinaryHeap;
use std::slice::Iter;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range(u64, u64, u64);

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

pub fn solve() {
    let content = read_input();
    let mut itr = content.lines();
    let seeds: Vec<u64> = itr
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| str::parse::<u64>(s).unwrap())
        .collect();

    // skip first empty line
    itr.next();

    // soil to fertilizer tuples
    // skip title
    itr.next();

    let mut groups: Vec<BinaryHeap<Range>> = vec![];
    let mut maps: BinaryHeap<Range> = BinaryHeap::new();

    while let Some(line) = itr.next() {
        if line.is_empty() {
            if !maps.is_empty() {
                groups.push(maps.clone());
            }
            maps.clear();
        } else if line.chars().next().unwrap().is_alphabetic() {
        } else {
            let a = line
                .split_whitespace()
                .map(|n| str::parse::<u64>(n).unwrap())
                .collect_tuple::<(u64, u64, u64)>()
                .unwrap();
            maps.push(Range(a.0, a.1, a.2));
        }
    }
    groups.push(maps);

    let result = seeds
        .iter()
        .map(|seed| {
            let mut current_value = *seed;
            for group in groups.iter() {
                let closest_range = group
                    .iter()
                    .skip_while(|range| {
                        current_value > range.1 + range.2 || current_value < range.1
                    })
                    .next();

                if let Some(closest_range) = closest_range {
                    current_value = current_value - closest_range.1 + closest_range.0;
                }
            }
            current_value
        })
        .min()
        .unwrap();

    println!("Part 1: {result}");
    part_2(content);
}

fn part_2(input: String) {
    let mut blocks = input.split("\n\n");
    let seed_ranges = blocks
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| str::parse::<u64>(s).unwrap())
        .tuples::<(u64, u64)>()
        .collect::<Vec<(u64, u64)>>();

    let maps = blocks
        .map(|b| {
            b.split("\n")
                .skip(1)
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| str::parse::<u64>(s).unwrap())
                        .collect_tuple::<(u64, u64, u64)>()
                        .unwrap()
                })
                .map(|(d, s, r)| (d, d + r, s, s + r))
                .collect_vec()
        })
        .collect_vec();

    let mut min_val = u64::MAX;
    for s in seed_ranges.into_iter() {
        min_val = min_val.min(process_seed_range(s, maps.clone().iter()));
    }

    println!("Part 2: {min_val}");
}

fn process_seed_range(s: (u64, u64), mut m: Iter<'_, Vec<(u64, u64, u64, u64)>>) -> u64 {
    if let Some(n_m) = m.next() {
        let mut min_val = u64::MAX;
        for m_r in n_m {
            if s.0 >= m_r.2 && s.0 + s.1 < m_r.3 {
                min_val = min_val.min(process_seed_range(
                    ((m_r.0 + s.0 - m_r.2), (m_r.0 + s.0 + s.1 - m_r.2)),
                    m.clone(),
                ));
            } else if s.0 >= m_r.2 && s.0 + s.1 > m_r.3 {
                min_val = min_val.min(process_seed_range((m_r.0 + s.0 - m_r.2, m_r.1), m.clone()));
            } else if s.0 < m_r.2 && s.0 + s.1 >= m_r.3 {
                min_val = min_val.min(process_seed_range((m_r.0, m_r.1), m.clone()));
            } else if s.0 <= m_r.2 && s.0 + s.1 < m_r.3 {
                min_val = min_val.min(process_seed_range(
                    (m_r.0, s.0 + s.1 - m_r.2 + m_r.0),
                    m.clone(),
                ));
            }
        }
        min_val
    } else {
        s.0
    }
}

fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_5.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
