use std::fs;

pub fn aoc4() -> Option<i32> {
    let result = fs::read_to_string("assets/aoc4/aoc4.txt");
    let mut sum: i32 = 0;

    match result {
        Ok(file) => {
            for line in file.lines() {
                let card_and_nums = line.split(":");
                let nums = card_and_nums.skip(1).next().unwrap().trim();

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

                if count != 0 {
                    sum += multiply(count);
                }

            }

            Some(sum)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn multiply(count: i32) -> i32 {
    let mut sum: i32 = 1;

    for x in 0..count-1 {
        sum = sum * 2;
    }

    sum
}
