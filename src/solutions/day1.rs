// https://adventofcode.com/2022/day/1
use std::fs;

pub fn part1() {
    let totals = get_ordered_totals();

    println!("Most: {}", totals[0]);
}

pub fn part2() {
    let totals = get_ordered_totals();

    let top3 = totals[0] + totals[1] + totals[2];
    println!("Top 3: {}", top3);
}

fn get_ordered_totals() -> Vec<u32> {
    let file_path = String::from("src/input/day1.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.split("\r\n");

    let mut totals = Vec::new();
    let mut current: u32 = 0;

    for line in lines {
        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }
    totals.sort();
    totals.reverse();

    totals
}
