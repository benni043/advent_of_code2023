use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn aoc11() -> Option<i32> {
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

            for row in empty_rows.iter().rev() {
                grid.insert(*row as usize, ".".repeat(grid[0].len()).chars().collect());
            }

            for col in empty_cols.iter().rev() {
                for i in (0..grid.len()).rev() {
                    let mut chars: Vec<char> = grid.get(i).unwrap().clone();
                    chars.insert(*col as usize, '.');

                    grid.remove(i);
                    grid.push(chars);
                }
            }

            grid.reverse();

            let mut cords: Vec<Cord> = vec![];
            for (index, row) in grid.iter().enumerate() {
                for (index2, cell) in row.iter().enumerate() {
                    if cell == &'#' {
                        cords.push((Cord { x: index, y: index2 }));
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

            println!("{:?}", connections.len());

            for connection in connections {
                // println!("{:?}", shortest_path_length(connection.0, connection.1, &grid));
            }

            Some(1)
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

// fn get_neighbors(cord: Cord, grid: &Vec<Vec<char>>) -> Vec<Cord> {
//     let mut neighbors = Vec::new();
//     let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // Right, Left, Down, Up
//     for &(dx, dy) in directions.iter() {
//         let nx = cord.x + dx;
//         let ny = cord.y + dy;
//         if nx >= 0 && ny >= 0 && nx < grid.len() && ny < grid[0].len()
//             && grid[nx as usize][ny as usize] != '.'
//         {
//             neighbors.push(Cord {
//                 x: nx,
//                 y: ny,
//             });
//         }
//     }
//     neighbors
// }
//
// fn shortest_path_length(start: Cord, end: Cord, grid: &Vec<Vec<char>>) -> Option<usize> {
//     let mut queue: VecDeque<(Cord, usize)> = VecDeque::new();
//     let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
//
//     queue.push_back((start, 0));
//     visited[&start.x][&start.y] = true;
//
//     while let Some((Cord, dist)) = queue.pop_front() {
//         if Cord.x == end.x && Cord.y == end.y {
//             return Some(dist); // Return the length of the shortest path
//         }
//         for neighbor in get_neighbors(Cord, grid) {
//             if !visited[neighbor.x][neighbor.y] {
//                 visited[neighbor.x][neighbor.y] = true;
//                 queue.push_back((neighbor, dist + 1));
//             }
//         }
//     }
//     None // No path found
// }

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Cord {
    x: usize,
    y: usize,
}