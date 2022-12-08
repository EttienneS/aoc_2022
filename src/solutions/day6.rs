use std::collections::{HashSet, VecDeque};

// https://adventofcode.com/2022/day/6
use crate::helpers::input_parser::InputParser;

pub fn part1() {
    print!("Day6 Part 1 Solution: ");
    find_marker(4);
}

pub fn part2() {
    print!("Day6 Part 2 Solution: ");
    find_marker(14);
}

fn find_marker(packet_size: usize) {
    let parser = InputParser::new("src/input/day6.txt");
    for line in parser.get_string_vec() {
        let mut buffer: VecDeque<char> = VecDeque::new();

        let mut i = 0;
        for c in line.chars() {
            i += 1;
            buffer.push_back(c);
            if buffer.len() == packet_size {
                let mut unique_set: HashSet<char> = HashSet::new();
                let mut unique = true;
                for b in buffer.clone().iter() {
                    if !unique_set.insert(*b) {
                        unique = false;
                        break;
                    }
                }

                if unique {
                    print!("Found unique sequence at: {} Sequence: ", i);
                    for c in buffer.iter() {
                        print!("{}", c);
                    }
                    break;
                } else {
                    buffer.pop_front();
                }
            }
        }
        buffer.clear();
        println!();
    }
}
