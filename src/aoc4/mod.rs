use std::collections::{BTreeMap, HashMap};
use std::fs;

pub fn aoc4() -> Option<i32> {
    let result = fs::read_to_string("assets/aoc4/aoc4.txt");
    let mut card_count: i32 = 0;

    match result {
        Ok(file) => {
            let mut map: BTreeMap<i32, i32> = BTreeMap::new();

            for line in file.lines() {
                let mut card_and_nums = line.split(":");

                let id: i32 = card_and_nums
                    .next()
                    .unwrap()
                    .trim()
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                // println!("{}", id);
                let nums = card_and_nums.next().unwrap().trim();

                let mut winning_cards_and_my_cards = nums.split("|");
                let winning_cards = winning_cards_and_my_cards.next().unwrap().trim();
                let my_cards = winning_cards_and_my_cards.next().unwrap().trim();

                let winning_numbers: Vec<i32> = winning_cards
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .filter_map(Result::ok)
                    .collect();

                let my_numbers: Vec<i32> = my_cards
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .filter_map(Result::ok)
                    .collect();

                let count: i32 = winning_numbers
                    .iter()
                    .filter(|&x| my_numbers.contains(x))
                    .count() as i32;

                if !map.contains_key(&id) {
                    map.insert(id, 1);
                } else {
                    map.insert(id, map.get(&id).unwrap() + 1);
                }

                if map.contains_key(&id) {
                    let cnt = *map.get(&id).unwrap();

                    for x in id + 1..(id + 1 + count) {
                        if !map.contains_key(&x) {
                            map.insert(x, cnt);
                        } else {
                            map.insert(x, map.get(&x).unwrap() + cnt);
                        }
                    }
                }
            }

            for x in map.values() {
                card_count += x;
            }

            Some(card_count)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn multiply(count: i32) -> i32 {
    let mut sum: i32 = 1;

    for x in 0..count - 1 {
        sum = sum * 2;
    }

    sum
}
