use std::fs;

pub fn aoc1() -> Option<i32> {
    let result = fs::read_to_string("assets/aoc1/aoc1.txt");
    let mut sum = 0;

    match result {
        Ok(file) => {
            for line in file.lines() {
                let my_chars: Vec<_> = line.chars().collect();

                let mut num1 = 0;

                for x in &my_chars {
                    if x.is_numeric() {
                        num1 = x.to_string().parse().unwrap();
                        break;
                    }
                }

                let mut num2 = 0;

                for x in my_chars.iter().rev() {
                    if x.is_numeric() {
                        num2 = x.to_string().parse().unwrap();
                        break;
                    }
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