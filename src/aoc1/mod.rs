use std::fs;

pub fn aoc1() -> Option<i32> {
    let str_nums = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let result = fs::read_to_string("assets/aoc1/aoc1.txt");

    let mut sum = 0;

    match result {
        Ok(file) => {
            for line in file.lines() {
                let mut num1 = 0;
                let mut act_line = line;

                'outer: while !act_line.is_empty() {
                    for x in &str_nums {
                        if act_line.starts_with(x) {
                            num1 = get_numeric_value(x).unwrap_or(0);
                            break 'outer;
                        }
                    }

                    for i in 0..10 {
                        if act_line.starts_with(i.to_string().as_str()) {
                            num1 = i;
                            break 'outer;
                        }
                    }

                    act_line = &act_line[1..act_line.len()];
                }

                let mut num2 = 0;
                let mut act_line = line;

                'outer: while !act_line.is_empty() {
                    for x in &str_nums {
                        if act_line.ends_with(x) {
                            num2 = get_numeric_value(x).unwrap_or(0);
                            break 'outer;
                        }
                    }

                    for i in 0..10 {
                        if act_line.ends_with(i.to_string().as_str()) {
                            num2 = i;
                            break 'outer;
                        }
                    }

                    act_line = &act_line[0..act_line.len()-1];
                }

                let new_num = format!("{num1}{num2}");

                sum += new_num.parse::<i32>().unwrap();
            }

            Some(sum)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn get_numeric_value(input: &str) -> Option<i32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}