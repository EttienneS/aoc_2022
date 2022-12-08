use std::collections::HashSet;

// https://adventofcode.com/2022/day/8
use crate::helpers::input_parser::InputParser;

const INPUTFILE: &str = "src/input/day8.txt";

#[derive(Debug, Eq, Clone, Hash)]
struct Tree {
    x: usize,
    y: usize,
    height: u32,
}
impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn part1() {
    let (trees, width, height) = get_forest();

    let mut visible: HashSet<&Tree> = HashSet::new();

    for x in 0..width {
        // top to bottom column
        visible.extend(check_collection(
            trees.iter().filter(|t| t.x == x).collect(),
        ));
        // bottom to top column
        visible.extend(check_collection(
            trees.iter().filter(|t| t.x == x).rev().collect(),
        ));
    }

    for y in 0..height {
        // left to right column
        visible.extend(check_collection(
            trees.iter().filter(|t| t.y == y).collect(),
        ));

        // right to left column
        visible.extend(check_collection(
            trees.iter().filter(|t| t.y == y).rev().collect(),
        ));
    }
    println!("Day 8 Part 1 Solution: {:?}", visible.len());
}

pub fn part2() {
    let (trees, width, height) = get_forest();

    let mut highest_score = 0;
    print!("Working.");
    for y in 0..height {
        if y == 0 {
            continue;
        }

        for x in 0..width {
            if x == 0 {
                continue;
            }

            let this_tree = trees.iter().find(|t| t.x == x && t.y == y).unwrap();

            let directions: Vec<Vec<&Tree>> = vec![
                trees.iter().filter(|t| t.x < x && t.y == y).rev().collect(), // left
                trees.iter().filter(|t| t.x > x && t.y == y).collect(),       // right
                trees.iter().filter(|t| t.y > y && t.x == x).collect(),       // down
                trees.iter().filter(|t| t.y < y && t.x == x).rev().collect(), // up
            ];

            let mut scores: Vec<u32> = vec![];
            for dir in directions {
                scores.push(get_direction_score(dir, this_tree));
            }

            let total = scores[0] * scores[1] * scores[2] * scores[3];
            if total > highest_score {
                highest_score = total;
            } else {
            }
        }
        println!("{}%", y);
    }

    println!("Day 8 Part 2 Solution: {:?}", highest_score);
}

fn get_direction_score(dir: Vec<&Tree>, this_tree: &Tree) -> u32 {
    let mut score = 0;
    for tree in dir {
        score += 1;
        if tree.height >= this_tree.height {
            break;
        }
    }
    score
}

fn check_collection(column: Vec<&Tree>) -> Vec<&Tree> {
    let mut last_visible: &Tree = column.first().unwrap();
    let mut visible: Vec<&Tree> = vec![];
    visible.push(last_visible);

    for tree in column.iter().skip(1) {
        if tree.height > last_visible.height {
            visible.push(tree);
            last_visible = tree;
        }
    }

    visible
}

fn get_forest() -> (Vec<Tree>, usize, usize) {
    let parser = InputParser::new(INPUTFILE);
    let mut trees: Vec<Tree> = vec![];
    let mut x = 0;
    let mut y = 0;
    let mut width = 0;
    for line in parser.get_string_vec() {
        for c in line.chars() {
            let tree = Tree {
                x,
                y,
                height: c.to_digit(10).unwrap(),
            };

            trees.push(tree);
            x += 1;
        }
        width = x;
        x = 0;
        y += 1;
    }
    (trees, width, y)
}
