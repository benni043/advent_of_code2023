use std::collections::HashMap;
use std::fs;

pub fn aoc8() -> Option<i32> {
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

            let mut pos = "AAA".to_string();
            let mut cnt = 0;

            'outer: loop {
                for instruction in &left_right_instructs {
                    if instruction == &'R' {
                        pos = positions.get(&pos.to_string()).unwrap().right.clone();
                    } else {
                        pos = positions.get(&pos.to_string()).unwrap().left.clone();
                    }

                    cnt += 1;

                    if pos == "ZZZ" {
                        break 'outer;
                    }
                }
            }

            Some(cnt)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[derive(Debug)]
struct Position {
    left: String,
    right: String,
}
