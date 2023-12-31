use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn aoc11() -> Option<usize> {
    let result = fs::read_to_string("assets/aoc11/aoc11.txt");

    match result {
        Ok(file) => {
            let mut grid: Vec<Vec<char>> = vec![];
            for line in file.lines() {
                grid.push(line.chars().collect());
            }

            let mut empty_rows: Vec<i32> = vec![];
            for (index, elem) in grid.iter().enumerate() {
                if !elem.contains(&'#') {
                    empty_rows.push(index as i32);
                }
            }

            let mut empty_cols: Vec<i32> = vec![];
            'outer: for (index, elem) in grid.iter().enumerate() {
                for i in 0..grid.len() {
                    if grid.get(i).unwrap().get(index).unwrap() == &'#' {
                        continue 'outer;
                    }
                }
                empty_cols.push(index as i32);
            }

            // for row in empty_rows.iter().rev() {
            //     grid.insert(*row as usize, ".".repeat(grid[0].len()).chars().collect());
            // }
            //
            // for col in empty_cols.iter().rev() {
            //     for i in (0..grid.len()).rev() {
            //         let mut chars: Vec<char> = grid.get(i).unwrap().clone();
            //         chars.insert(*col as usize, '.');
            //
            //         grid.remove(i);
            //         grid.push(chars);
            //     }
            // }
            //
            // grid.reverse();

            let mut cords: Vec<Cord> = vec![];
            for (index, row) in grid.iter().enumerate() {
                for (index2, cell) in row.iter().enumerate() {
                    if cell == &'#' {
                        cords.push((Cord { x: index2, y: index, x_shifts: 0, y_shifts: 0 }));
                    }
                }
            }

            let mut connections: Vec<(Cord, Cord)> = Vec::new();

            for i in 0..cords.len() {
                for j in (i + 1)..cords.len() {
                    let mut cord1: Cord = cords[i].clone();
                    let mut cord2: Cord = cords[j].clone();

                    connections.push((cord1, cord2));
                }
            }

            let mut sum = 0;
            for (from, to) in connections {
                let mut part1 = 0;
                if from.x > to.x {
                    part1 = from.x - to.x;

                    for row in empty_rows.clone() {
                        if to.x >= row as usize && from.x <= row as usize {
                            sum += 1;
                        }
                    }
                } else {
                    part1 = to.x - from.x;

                    for row in empty_rows.clone() {
                        if from.x >= row as usize && to.x <= row as usize {
                            sum += 1;
                        }
                    }
                }

                let mut part2 = 0;
                if from.y > to.y {
                    part2 = from.y - to.y;

                    for col in empty_cols.clone() {
                        if to.y >= col as usize && from.y <= col as usize {
                            sum += 1;
                        }
                    }
                } else {
                    part2 = to.y - from.y;

                    for col in empty_cols.clone() {
                        if to.y >= col as usize && from.y <= col as usize {
                            sum += 1;
                        }
                    }
                }

                sum += part1 + part2;
            }

            Some(sum)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Cord {
    x: usize,
    y: usize,
    x_shifts: usize,
    y_shifts: usize,
}