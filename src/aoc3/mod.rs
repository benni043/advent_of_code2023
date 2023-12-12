use std::fmt::{Debug};
use std::fs;

pub fn aoc3() -> Option<usize> {
    let result = fs::read_to_string("assets/aoc3/aoc3.txt");

    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    match result {
        Ok(file) => {
            for line in file.lines().enumerate() {
                let line_arr: Vec<char> = line.1.chars().collect();
                let mut build_number = "".to_string();

                for char in line_arr.iter().enumerate() {
                    if char.1.is_digit(10) {
                        build_number += &char.1.to_string();
                    }

                    if char.1 == &'.' || (char.1 != &'.' && !char.1.is_digit(10)) {
                        if !build_number.is_empty() {
                            let mut number = Number {
                                x_from: (char.0 - build_number.len()) as i32,
                                x_to: char.0 as i32 - 1,
                                y_index: line.0 as i32,
                                number: build_number.parse().unwrap(),
                                possible_symbol_cords: vec![],
                            };

                            for x in number.x_from..=number.x_to {
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index - 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x,
                                    y: number.y_index - 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index - 1,
                                });
                            }

                            numbers.push(number);

                            build_number = "".to_string();
                        }
                    }

                    if char.0 == line_arr.len() - 1 {
                        if !build_number.is_empty() {
                            let mut number = Number {
                                x_from: (char.0 - build_number.len() - 1) as i32,
                                x_to: char.0 as i32,
                                y_index: line.0 as i32,
                                number: build_number.parse().unwrap(),
                                possible_symbol_cords: vec![],
                            };

                            for x in number.x_from..=number.x_to {
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x - 1,
                                    y: number.y_index - 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x,
                                    y: number.y_index - 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index + 1,
                                });
                                number.possible_symbol_cords.push(PossibleSymbolCord {
                                    x: x + 1,
                                    y: number.y_index - 1,
                                });
                            }

                            numbers.push(number);

                            build_number = "".to_string();
                        }
                    }

                    if char.1 != &'.' && !char.1.is_digit(10) {
                        symbols.push(Symbol {
                            x_index: char.0 as i32,
                            y_index: line.0 as i32,
                            form: char.1.to_string(),
                            numbers: vec![],
                        });
                    }
                }
            }

            for x in &mut symbols {
                if x.form == '*'.to_string() {
                    'outer: for number in &numbers {
                        for cord in &number.possible_symbol_cords {
                            if cord.x == x.x_index && cord.y == x.y_index {
                                x.numbers.push(number.number as i32);
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            let mut temp = 0;
            for x in &symbols {
                if x.numbers.len() == 2 {
                    temp += x.numbers[0] * x.numbers[1];
                }
            }

            Some(temp as usize)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[derive(Debug)]
struct PossibleSymbolCord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Number {
    x_from: i32,
    x_to: i32,
    y_index: i32,
    number: usize,
    possible_symbol_cords: Vec<PossibleSymbolCord>,
}

#[derive(Debug)]
struct Symbol {
    x_index: i32,
    y_index: i32,
    form: String,
    numbers: Vec<i32>,
}