use std::collections::HashMap;
use std::fs;

pub fn aoc8() -> Option<i64> {
    let result = fs::read_to_string("assets/aoc8/aoc8.txt");

    match result {
        Ok(file) => {
            let mut left_right_instructs: Vec<char> = vec![];
            let mut positions: HashMap<String, Position> = HashMap::new();

            for (index, line) in file.lines().enumerate() {
                if index == 0 {
                    left_right_instructs = line.chars().collect();
                    continue;
                }

                if index == 1 {
                    continue;
                }

                if index > 1 {
                    let splitted1: Vec<&str> = line.split("=").collect();
                    let current = splitted1.get(0).unwrap().trim().to_string();

                    let splitted2 = splitted1.get(1).unwrap().trim();
                    let left = splitted2[1..4].to_string();
                    let right = splitted2[6..9].to_string();

                    let position = Position { left, right };

                    positions.insert(current, position);
                }
            }

            let mut start_postions: Vec<String> = vec![];
            let mut cnt_for_postitions: Vec<i64> = vec![];

            for key in positions.keys() {
                if key.ends_with("A") {
                    start_postions.push(key.clone());
                }
            }

            for mut start_pos in start_postions {
                let mut cnt = 0;

                'outer: loop {
                    for instruction in &left_right_instructs {
                        if instruction == &'R' {
                            start_pos =
                                positions.get(&start_pos.to_string()).unwrap().right.clone();
                        } else {
                            start_pos = positions.get(&start_pos.to_string()).unwrap().left.clone();
                        }

                        cnt += 1;

                        if start_pos.ends_with("Z") {
                            break 'outer;
                        }
                    }
                }

                cnt_for_postitions.push(cnt);
            }

            Some(lcm_multiple(cnt_for_postitions))
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn lcm_multiple(mut numbers: Vec<i64>) -> i64 {
    match numbers.pop() {
        Some(last) => {
            let mut result = last;
            while let Some(num) = numbers.pop() {
                result = lcm(result, num);
            }
            result
        }
        None => 1,
    }
}

#[derive(Debug)]
struct Position {
    left: String,
    right: String,
}
