use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
enum Hands {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn card_to_idx(card: &char) -> usize {
    match card {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("UNKNOWN CARD"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct IndexedHand {
    array: [usize; 5],
}

impl Ord for IndexedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.array.cmp(&other.array)
    }
}

impl PartialOrd for IndexedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve() {
    let content = read_input();
    let rows: Vec<(&str, u32)> = content
        .lines()
        .map(|l| {
            let tmp = l.split_once(" ").unwrap();
            (tmp.0, str::parse::<u32>(tmp.1).unwrap())
        })
        .collect_vec();

    let mut high_cards: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut one_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut two_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut three_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut full_houses: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut four_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut five_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();

    let mut indexed_reprs: Vec<(Hands, IndexedHand, u32)> = vec![];
    for row in rows.iter() {
        let mut arr = [0_u8; 13];
        let mut index_repr = [0_usize; 5];
        row.0.chars().enumerate().for_each(|(i, c)| {
            let idx = card_to_idx(&c);
            index_repr[i] = idx;
            arr[idx] += 1;
        });
        let mut current_hand = Hands::HighCard;
        for v in arr.into_iter() {
            if v == 0 || v == 1 {
            } else if v == 2 {
                if current_hand == Hands::HighCard {
                    current_hand = Hands::OnePair;
                } else if current_hand == Hands::OnePair {
                    current_hand = Hands::TwoPair;
                } else if current_hand == Hands::ThreeOfAKind {
                    current_hand = Hands::FullHouse;
                }
            } else if v == 3 {
                if current_hand == Hands::HighCard {
                    current_hand = Hands::ThreeOfAKind;
                } else if current_hand == Hands::OnePair {
                    current_hand = Hands::FullHouse;
                }
            } else if v == 4 {
                current_hand = Hands::FourOfAKind;
            } else if v == 5 {
                current_hand = Hands::FiveOfAKind;
            }
        }

        let indexed_hand = IndexedHand { array: index_repr };
        match current_hand {
            Hands::HighCard => high_cards.push(indexed_hand.clone()),
            Hands::OnePair => one_pairs.push(indexed_hand.clone()),
            Hands::TwoPair => two_pairs.push(indexed_hand.clone()),
            Hands::ThreeOfAKind => three_of_a_kinds.push(indexed_hand.clone()),
            Hands::FullHouse => full_houses.push(indexed_hand.clone()),
            Hands::FourOfAKind => four_of_a_kinds.push(indexed_hand.clone()),
            Hands::FiveOfAKind => five_of_a_kinds.push(indexed_hand.clone()),
        }
        indexed_reprs.push((current_hand, indexed_hand, row.1));
    }

    let high_cards = high_cards.into_sorted_vec();
    let one_pairs = one_pairs.into_sorted_vec();
    let two_pairs = two_pairs.into_sorted_vec();
    let three_of_a_kinds = three_of_a_kinds.into_sorted_vec();
    let full_houses = full_houses.into_sorted_vec();
    let four_of_a_kinds = four_of_a_kinds.into_sorted_vec();
    let five_of_a_kinds = five_of_a_kinds.into_sorted_vec();

    let one_pairs_offset = high_cards.len() as u32;
    let two_pairs_offset = one_pairs_offset + one_pairs.len() as u32;
    let three_of_a_kinds_offset = two_pairs_offset + two_pairs.len() as u32;
    let full_houses_offset = three_of_a_kinds_offset + three_of_a_kinds.len() as u32;
    let four_of_a_kinds_offset = full_houses_offset + full_houses.len() as u32;
    let five_of_a_kinds_offset = four_of_a_kinds_offset + four_of_a_kinds.len() as u32;

    let result: u32 = indexed_reprs
        .iter()
        .map(|(h, i, b)| {
            //println!("{:?} - {:?}", h, i);
            match h {
                Hands::HighCard => {
                    let pos = high_cards.iter().position(|r| r == i).unwrap();
                    (pos as u32 + 1) * b
                }
                Hands::OnePair => {
                    let pos = one_pairs.iter().position(|r| r == i).unwrap();
                    (pos as u32 + one_pairs_offset + 1) * b
                }
                Hands::TwoPair => {
                    let pos = two_pairs.iter().position(|r| r == i).unwrap();
                    (pos as u32 + two_pairs_offset + 1) * b
                }
                Hands::ThreeOfAKind => {
                    let pos = three_of_a_kinds.iter().position(|r| r == i).unwrap();
                    (pos as u32 + three_of_a_kinds_offset + 1) * b
                }
                Hands::FullHouse => {
                    let pos = full_houses.iter().position(|r| r == i).unwrap();
                    (pos as u32 + full_houses_offset + 1) * b
                }
                Hands::FourOfAKind => {
                    let pos = four_of_a_kinds.iter().position(|r| r == i).unwrap();
                    (pos as u32 + four_of_a_kinds_offset + 1) * b
                }
                Hands::FiveOfAKind => {
                    let pos = five_of_a_kinds.iter().position(|r| r == i).unwrap();
                    (pos as u32 + five_of_a_kinds_offset + 1) * b
                }
            }
        })
        .sum();

    println!("Part 1: {result}");
    part_2(content);
}
fn part_2(_input: String) {
    println!("Part 2: {}", "<RESULT>");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_7.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
