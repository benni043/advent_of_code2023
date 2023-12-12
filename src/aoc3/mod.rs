use std::fmt::{Debug};
use std::fs;

pub fn aoc3() -> Option<usize> {
    let result = fs::read_to_string("assets/aoc3/aoc3.txt");
    let mut sum = 0;

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
                            form: char.0.to_string()
                        });
                    }
                }
            }

            for x in &mut numbers {
                if has_symbol_neighbor(x, &symbols) {
                    sum += x.number;
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

fn has_symbol_neighbor(number: &mut Number, symbols: &Vec<Symbol>) -> bool {
    for x in &number.possible_symbol_cords {
        for y in symbols {
            if x.x == y.x_index && x.y == y.y_index {
                return true;
            }
        }
    }

    false
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
    form: String
}
