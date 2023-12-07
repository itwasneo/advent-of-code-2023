use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
    let mut high_cards: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut one_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut two_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut three_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut full_houses: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut four_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut five_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();

    // Collecting each processed data in one vector.
    // I'm using this vector, when I traverse the hands second time.
    let mut processed: Vec<(Hands, IndexedHand, u32)> = vec![];

    content.lines().for_each(|l| {
        let (hand, bid) = l.split_once(" ").unwrap();
        let mut index_repr = [0_usize; 5];
        let mut arr = [0_u8; 13];

        // Converting each card to their corresponding index numbers and store
        // it in a fixed sized array(index_repr)
        // Instead of some kind of HashMap, I also again used a fixed sized
        // for the "seen" cards(arr)
        hand.chars().enumerate().for_each(|(i, c)| {
            let idx = card_to_idx(&c);
            index_repr[i] = idx;
            arr[idx] += 1;
        });

        // Determining which hand type first using arr
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
        processed.push((current_hand, indexed_hand, str::parse::<u32>(bid).unwrap()));
    });

    // Converting BinaryHeaps into sorted_vecs so that "position" function
    // gives the accurate positions.
    let high_cards = high_cards.into_sorted_vec();
    let one_pairs = one_pairs.into_sorted_vec();
    let two_pairs = two_pairs.into_sorted_vec();
    let three_of_a_kinds = three_of_a_kinds.into_sorted_vec();
    let full_houses = full_houses.into_sorted_vec();
    let four_of_a_kinds = four_of_a_kinds.into_sorted_vec();
    let five_of_a_kinds = five_of_a_kinds.into_sorted_vec();

    // These are used during ranking calculations
    let one_pairs_offset = high_cards.len() as u32;
    let two_pairs_offset = one_pairs_offset + one_pairs.len() as u32;
    let three_of_a_kinds_offset = two_pairs_offset + two_pairs.len() as u32;
    let full_houses_offset = three_of_a_kinds_offset + three_of_a_kinds.len() as u32;
    let four_of_a_kinds_offset = full_houses_offset + full_houses.len() as u32;
    let five_of_a_kinds_offset = four_of_a_kinds_offset + four_of_a_kinds.len() as u32;

    // Second Traverse
    // Since each hand is inside its own BinaryHeap > Sorted Vecs, it is
    // O(n) to find its position where n is not the overall Hand count but the
    // count of the hands that blong to single HandType.
    let result: u32 = processed
        .iter()
        .map(|(h, i, b)| match h {
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
        })
        .sum();

    println!("Part 1: {result}");
    part_2(content);
}

// For the second part, I modified idx finder function, so that J is now in the
// lowest index.
fn card_to_idx_2(card: &char) -> usize {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("UNKNOWN CARD"),
    }
}

fn part_2(input: String) {
    let mut high_cards: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut one_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut two_pairs: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut three_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut full_houses: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut four_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut five_of_a_kinds: BinaryHeap<IndexedHand> = BinaryHeap::new();
    let mut indexed_reprs: Vec<(Hands, IndexedHand, u32)> = vec![];

    input.lines().for_each(|l| {
        let (hand, bid) = l.split_once(" ").unwrap();
        let mut arr = [0_u8; 13];
        let mut index_repr = [0_usize; 5];
        hand.chars().enumerate().for_each(|(i, c)| {
            let idx = card_to_idx_2(&c);
            index_repr[i] = idx;
            arr[idx] += 1;
        });

        // It is important to determine first what is the HandType, before
        // handling new Joker rule.
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

        // Here I handle the Joker cards. arr has the info of which cards we
        // have.
        // arr[0] is represents Joker count.
        // Handle each HandType
        if arr[0] > 0 {
            let joker_cnt = arr[0];
            match current_hand {
                Hands::HighCard => {
                    // Care the rev() method while iterating
                    if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 1) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::OnePair => {
                    if arr[0] == 2 {
                        if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 1) {
                            *elem += joker_cnt;
                        }
                    } else if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 2) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::TwoPair => {
                    if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 2) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::ThreeOfAKind => {
                    if arr[0] == 3 {
                        if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 1) {
                            *elem += joker_cnt;
                        }
                    } else if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 3) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::FullHouse => {
                    if arr[0] == 3 {
                        if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 2) {
                            *elem += joker_cnt;
                        }
                    } else if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 3) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::FourOfAKind => {
                    if arr[0] == 4 {
                        if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 1) {
                            *elem += joker_cnt;
                        }
                    } else if let Some(elem) = arr.iter_mut().rev().find(|&&mut i| i == 4) {
                        *elem += joker_cnt;
                    }
                    arr[0] = 0;
                }
                Hands::FiveOfAKind => {}
            }

            // Then update the current_hand again for Joker-rule-applied hand
            current_hand = Hands::HighCard;
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
        }

        // Rest is the same ---------------------------------------------------
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
        indexed_reprs.push((current_hand, indexed_hand, str::parse::<u32>(bid).unwrap()));
    });

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
        .map(|(h, i, b)| match h {
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
        })
        .sum();
    println!("Part 2: {result}");
}
fn read_input() -> String {
    let current_dir = std::env::current_dir().expect("Failed to get current_dir");
    let file_path = current_dir.join("input/input_7.txt");
    let content = std::fs::read_to_string(file_path).expect("Failed read the content of the file");
    content.trim().to_owned()
}
