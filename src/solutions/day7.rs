use std::f64::MAX_EXP;

use indextree::{Arena, NodeId};

// https://adventofcode.com/2022/day/7
use crate::helpers::input_parser::InputParser;

enum Entry {
    File { name: String, size: usize },
    Directory { name: String },
}

fn load_directory(fs: &mut Arena<Entry>) -> (NodeId, Vec<NodeId>) {
    let input = InputParser::new("src/input/day7.txt");
    let root = fs.new_node(Entry::Directory {
        name: String::from("/"),
    });
    let mut directories: Vec<NodeId> = vec![];
    let mut current_node = root;

    for line in input.get_string_vec() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        match args[..] {
            ["$", "ls"] => (), // do nothing

            ["$", "cd", "/"] => current_node = root,
            ["$", "cd", ".."] => current_node = fs[current_node].parent().unwrap(),

            // swap to child node
            ["$", "cd", directory] => {
                current_node = current_node
                            .children(fs)
                            .find(|&child| matches!(fs[child].get(), Entry::Directory { name } if name == directory)
                            )
                            .unwrap();
            }

            // make directory
            ["dir", name] => {
                let dir = fs.new_node(Entry::Directory {
                    name: String::from(name),
                });
                directories.push(dir);
                current_node.append(dir, fs);
            }

            // make file
            [size, name] => {
                let size: usize = size.parse().unwrap();
                let file = fs.new_node(Entry::File {
                    name: String::from(name),
                    size,
                });
                current_node.append(file, fs);
            }
            _ => panic!("Unknown command!"),
        }
    }
    (root, directories)
}

pub fn part1() {
    let fs = &mut Arena::new();
    let (_, directories) = load_directory(fs);

    let sizes: usize = directories
        .iter()
        .map(|d| get_size(&d, &fs))
        .filter(|s| *s < 100_000)
        .sum();

    println!("Day 7 Part 1 Solution: {:?}", sizes);
}

pub fn part2() {
    let fs = &mut Arena::new();
    let (root, directories) = load_directory(fs);

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let used_space = get_size(&root, fs);
    let free_space = total_space - used_space;

    let space_to_free = needed_space - free_space;
    println!(
        "We have {used_space}/{total_space} ({} free), we need to clear {} minimum",
        free_space, space_to_free
    );

    let mut min = total_space;
    for dir in directories {
        let size = get_size(&dir, fs);
        if size >= space_to_free {
            if size < min {
                min = size;
                print!("*");
            }
            println!(
                "Deleting entry with size {} would free up enough space",
                size
            );
        }
    }

    println!("Day 7 Part 2 Solution: {:?}", min);
}

fn get_size(entry: &NodeId, arena: &Arena<Entry>) -> usize {
    match arena[*entry].get() {
        Entry::File { name: _, size } => *size,
        Entry::Directory { name: _ } => entry
            .children(arena)
            .map(|child| match arena[child].get() {
                Entry::File { name: _, size } => *size,
                Entry::Directory { name: _ } => get_size(&child, arena),
            })
            .sum(),
    }
}
