// https://adventofcode.com/2022/day/3
use crate::helpers::input_parser::InputParser;

const SEQ: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part1() {
    let parser = InputParser::new("src/input/day3.txt");
    let input = parser.get_string_vec();

    let mut total = 0;

    for bag in input {
        let half = bag.len() / 2;

        let p1 = &bag[..half];
        let p2 = &bag[half..];

        for p1 in p1.chars() {
            if p2.contains(p1) {
                total += get_score(p1);
                break;
            }
        }
    }

    println!("Answer P1: {}", total);
}

pub fn get_score(c: char) -> usize {
    SEQ.find(c).unwrap() + 1
}

pub fn part2() {
    let parser = InputParser::new("src/input/day3.txt");
    let input = parser.get_string_vec();

    let mut total = 0;

    let mut counter = 0;
    let mut set: Vec<&str> = Vec::new();
    for bag in input {
        set.push(bag);
        counter += 1;

        if counter == 3 {
            counter = 0;
            set.sort_by(|a, b| a.len().cmp(&b.len()));
            set.reverse();

            let main = set[0];
            let other1 = set[1];
            let other2 = set[2];

            for c in main.chars() {
                if other1.contains(c) && other2.contains(c) {
                    total += get_score(c);
                    break;
                }
            }

            set.clear();
        }
    }

    println!("Answer P2: {}", total);
}
