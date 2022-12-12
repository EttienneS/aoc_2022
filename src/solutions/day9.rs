// https://adventofcode.com/2022/day/X
use crate::helpers::input_parser::InputParser;
use min_max::*;
use std::collections::HashSet;

const INPUTFILE: &str = "src/input/day9e.txt";

pub fn part1() {
    let parser = InputParser::new(INPUTFILE);

    let mut path: HashSet<(i32, i32)> = HashSet::new();
    path.insert((0, 0));
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in parser.get_string_vec() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut move_x = 0;
        let mut move_y = 0;
        match args[..] {
            ["R", value] => move_x = value.parse::<i32>().unwrap(),
            ["L", value] => move_x = value.parse::<i32>().unwrap() * -1,
            ["U", value] => move_y = value.parse::<i32>().unwrap(),
            ["D", value] => move_y = value.parse::<i32>().unwrap() * -1,
            _ => (),
        }

        if move_x != 0 {
            let min_max_x = min_max!(move_x, 0);
            for x in min_max_x.0..min_max_x.1 {
                head.0 += x;
                if (tail.0 - head.0).abs() > 1 {
                    tail.0 += x;
                    path.insert((tail.0, tail.1));
                }
            }
        }
        if move_y != 0 {
            let min_max_y = min_max!(move_y, 0);
            for y in min_max_y.0..min_max_y.1 {
                head.1 += y;
                if (tail.1 - head.1).abs() > 1 {
                    tail.1 += y;
                    path.insert((tail.0, tail.1));
                }
            }
        }
        println!("Move {}:{}", move_x, move_y);
    }

    println!("Day 9 Part 1 Solution: {:?}", path.len());
}

pub fn part2() {
    let parser = InputParser::new(INPUTFILE);

    println!("Day 9 Part 2 Solution: {:?}", 0);
}

// // https://adventofcode.com/2022/day/9
// use crate::helpers::input_parser::InputParser;

// const INPUTFILE: &str = "src/input/day9e.txt";

// #[derive(Eq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }
// struct Board {
//     cells: Vec<Vec<u32>>,
//     size: i32,
//     head: Point,
//     tail: Point,
// }

// impl Board {
//     fn new(size: i32) -> Self {
//         let mut cells: Vec<Vec<u32>> = vec![];
//         for _ in 0..size {
//             let mut row: Vec<u32> = vec![];
//             for _ in 0..size {
//                 row.push(0);
//             }
//             cells.push(row);
//         }

//         let head = Point { x: 0, y: 0 };

//         let tail = Point { x: 0, y: 0 };

//         cells[0][0] += 1;

//         Self {
//             size,
//             cells,
//             head,
//             tail,
//         }
//     }

//     fn print_board(&self) {
//         for y in 0..self.size {
//             for x in 0..self.size {
//                 let p = Point { x, y };

//                 if self.head == p && self.tail == p {
//                     print!("X");
//                 } else if self.head == p {
//                     print!("H");
//                 } else if self.tail == p {
//                     print!("T");
//                 } else {
//                     let count = self.cells[x as usize][y as usize];

//                     if count > 0 {
//                         print!("#");
//                     } else {
//                         print!(".");
//                     }
//                 }
//             }
//             println!();
//         }
//         println!();
//     }

//     fn move_head(&mut self, move_x: i32, move_y: i32) {
//         for _ in 0..move_x.abs() {
//             if move_x > 0 {
//                 self.head.x += 1;
//             } else {
//                 self.head.x -= 1;
//             }
//             self.follow_head();
//             self.print_board();
//         }

//         for _ in 0..move_y.abs() {
//             if move_y > 0 {
//                 self.head.y += 1;
//             } else {
//                 self.head.y -= 1;
//             }
//             self.follow_head();
//             self.print_board();
//         }
//     }

//     fn follow_head(&mut self) {
//         let diff_x = self.head.x - self.tail.x;
//         let diff_y = self.head.y - self.tail.y;
//         let mut moved: bool;
//         if diff_x.abs() > 0 && diff_y.abs() > 1 {
//             self.tail.x = self.head.x;
//             self.tail.y += diff_y / 2;
//             moved = true;
//         } else if diff_x.abs() > 1 {
//             self.tail.x += diff_x / 2;
//             moved = true;
//         } else {
//             self.tail.y += diff_y / 2;
//             moved = true;
//         }

//         if moved {
//             self.cells[self.tail.x as usize][self.tail.y as usize] += 1;
//         }
//     }
// }

// pub fn part1() {
//     let parser = InputParser::new(INPUTFILE);

//     let mut board = Board::new(10);

//     for line in parser.get_string_vec() {
//         let args: Vec<&str> = line.split_ascii_whitespace().collect();
//         let mut move_x = 0;
//         let mut move_y = 0;
//         match args[..] {
//             ["R", value] => move_x = value.parse::<i32>().unwrap(),
//             ["L", value] => move_x = value.parse::<i32>().unwrap() * -1,
//             ["U", value] => move_y = value.parse::<i32>().unwrap(),
//             ["D", value] => move_y = value.parse::<i32>().unwrap() * -1,
//             _ => (),
//         }

//         println!("Move {}:{}", move_x, move_y);
//         board.move_head(move_x, move_y);
//     }

//     let mut total = 0;
//     for y in board.cells {
//         for x in y {
//             if x > 0 {
//                 total += 1;
//             }
//         }
//     }

//     println!("Day 9 Part 1 Solution: {:?}", total);
// }

// pub fn part2() {
//     let parser = InputParser::new(INPUTFILE);

//     println!("Day 9 Part 2 Solution: {:?}", 0);
// }
