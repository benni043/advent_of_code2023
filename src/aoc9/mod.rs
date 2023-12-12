use std::fs;
use std::process::id;

pub fn aoc9() -> Option<i32> {
    let result = fs::read_to_string("assets/aoc9/aoc9.txt");
    let mut sum = 0;

    match result {
        Ok(file) => {
            for line in file.lines() {
                let mut hist_vals_of_line: Vec<Vec<i32>> = vec![];

                let history_values: Vec<i32> =
                    line.split(" ").filter_map(|val| val.parse().ok()).collect();

                hist_vals_of_line.push(history_values);

                let mut index = 0;

                while !is_finished(&hist_vals_of_line.get(index).unwrap()) {
                    let mut temp_hist_vals: Vec<i32> = vec![];

                    for i in 0..hist_vals_of_line.get(index).unwrap().len() - 1 {
                        temp_hist_vals.push(
                            hist_vals_of_line.get(index).unwrap().get(i + 1).unwrap()
                                - hist_vals_of_line.get(index).unwrap().get(i).unwrap(),
                        );
                    }

                    hist_vals_of_line.push(temp_hist_vals);

                    temp_hist_vals = vec![];
                    index += 1;
                }

                let mut num = 0;
                for x in (1..hist_vals_of_line.len()).rev() {
                    let num2 = *hist_vals_of_line
                        .get(x - 1)
                        .unwrap()
                        .get(0)
                        .unwrap();

                    num = num2 - num;
                }

                sum += num;
            }

            Some(sum)
        }
        Err(err) => None,
    }
}

fn is_finished(list: &Vec<i32>) -> bool {
    let mut cnt = 0;

    for elem in list {
        if elem == &0 {
            cnt += 1;
        }
    }

    if cnt == list.len() {
        true
    } else {
        false
    }
}
