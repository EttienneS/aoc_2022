// https://adventofcode.com/2022/day/4
use crate::helpers::input_parser::InputParser;

pub fn part1() {
    let total: u32 = get_pairs_from_input()
        .iter()
        .map(|pair| pair_contains_other(pair))
        .sum();

    println!("Answer P1: {}", total);
}

fn pair_contains_other(pair: &Vec<(u32, u32)>) -> u32 {
    let min1 = pair[0].0;
    let max1 = pair[0].1;
    let min2 = pair[1].0;
    let max2 = pair[1].1;

    print!("Contains: {:?}-{:?}", pair[0], pair[1]);
    if (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1) {
        println!(" - YES");
        1
    } else {
        println!(" - NO");
        0
    }
}

pub fn part2() {
    let total: u32 = get_pairs_from_input()
        .iter()
        .map(|pair| pair_overlaps_other(pair))
        .sum();

    println!("Answer P2: {}", total);
}

fn get_pairs_from_input() -> Vec<Vec<(u32, u32)>> {
    let parser = InputParser::new("src/input/day4.txt");
    let pairs: Vec<Vec<(u32, u32)>> = parser
        .get_string_vec()
        .iter()
        .map(|l| {
            l.split(",")
                .map(|p| {
                    let d1 = p.find("-").unwrap();
                    let start = p[..d1].parse::<u32>().unwrap();
                    let end = p[d1 + 1..].parse::<u32>().unwrap();
                    (start, end)
                })
                .collect()
        })
        .collect();
    pairs
}

fn pair_overlaps_other(pair: &Vec<(u32, u32)>) -> u32 {
    let min1 = pair[0].0;
    let max1 = pair[0].1;
    let min2 = pair[1].0;
    let max2 = pair[1].1;

    print!("Overlaps {:?}-{:?}", pair[0], pair[1]);
    if (min1 <= max2 && max1 >= min2) || (min2 <= max1 && max2 >= min1) {
        println!(" - YES");
        1
    } else {
        println!(" - NO");
        0
    }
}
