use std::fs;

pub fn aoc6() -> Option<i32> {
    let result = fs::read_to_string("assets/aoc6/aoc6.txt");

    match result {
        Ok(file) => {
            let mut times: Vec<i32> = vec![];
            let mut distances: Vec<i32> = vec![];

            for (index,line) in file.lines().enumerate() {
                let values: &str = line.split(":").skip(1).next().unwrap().trim();

                if index == 0 {
                    times = values.split_whitespace().filter_map(|value| value.parse().ok()).collect();
                } else {
                    distances = values.split_whitespace().filter_map(|value| value.parse().ok()).collect();
                }

            }

            let mut sum = 1;

            for (index, time) in times.iter().enumerate() {
                let mut count = 0;
                let mut distance = 1;

                for millis in 0..*time {
                    distance = millis * (time - millis);
                    if distance > distances[index] {
                        count += 1;
                    }
                }

                sum *= count;
            }

            Some(sum)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}