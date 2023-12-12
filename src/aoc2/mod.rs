use std::fs;

pub fn aoc2() -> Option<usize> {
    let common_game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = fs::read_to_string("assets/aoc2/aoc2.txt");
    let mut sum_id = 0;

    match result {
        Ok(file) => {
            for x in file.lines() {
                let mut splitted = x.split(':');

                let id: usize = splitted
                    .next()
                    .unwrap()
                    .split(' ')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();

                let act_line = splitted.next().unwrap().trim();

                let splitted = act_line.split(';');

                let mut current_game = Game {
                    red: 0,
                    blue: 0,
                    green: 0,
                };

                for x in splitted {
                    let splitted = x.split(',');

                    for mut y in splitted {
                        y = y.trim();

                        let mut splitted = y.split(' ');

                        let count: usize = splitted.next().unwrap().parse().unwrap();
                        let color = splitted.next().unwrap();

                        match color {
                            "red" => {
                                current_game.red = current_game.red.max(count);
                            }
                            "blue" => {
                                current_game.blue = current_game.blue.max(count);
                            }
                            "green" => {
                                current_game.green = current_game.green.max(count);
                            }
                            _ => {}
                        }
                    }
                }

                if current_game.red <= common_game.red
                    && current_game.blue <= common_game.blue
                    && current_game.green <= common_game.green
                {
                    sum_id += id;
                }

                current_game.red = 0;
                current_game.blue = 0;
                current_game.green = 0;
            }

            Some(sum_id)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

struct Game {
    red: usize,
    green: usize,
    blue: usize,
}
