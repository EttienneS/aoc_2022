use std::collections::VecDeque;

// https://adventofcode.com/2022/day/5
use crate::helpers::input_parser::InputParser;

struct Crane {
    parser: InputParser,
    stack_count: usize,
    debug_mode: bool,
}

impl Crane {
    fn new() -> Self {
        Self {
            parser: InputParser::new("src/input/day5.txt"),
            stack_count: 9,
            debug_mode: false,
        }
    }

    fn run_as_model_9000(&self) {
        // model 9000 can only move one crate at at time
        let data = &self.get_data(self.stack_count);
        let mut stacks = data.0.clone();
        let mut instructions = data.1.clone();

        loop {
            if let Some(i) = instructions.pop() {
                let amount = i.0;
                let from = i.1;
                let to = i.2;

                for _i in 0..amount {
                    if let Some(item) = stacks[from].pop_back() {
                        stacks[to].push_back(item)
                    } else {
                        println!("empty stack")
                    }
                }
            } else {
                break;
            }

            if self.debug_mode {
                print_stacks(&stacks);
            }
        }

        print!("Model 9000: ");
        print_top_of_stacks(stacks);
    }

    fn run_as_model_9001(&self) {
        // model 9001 can move multiple crates at a time
        let data = &self.get_data(self.stack_count);
        let mut stacks = data.0.clone();
        let mut instructions = data.1.clone();

        loop {
            if let Some(i) = instructions.pop() {
                let amount = i.0;
                let from = i.1;
                let to = i.2;

                let mut buffer: Vec<&str> = Vec::new();
                for _i in 0..amount {
                    if let Some(item) = stacks[from].pop_back() {
                        buffer.push(item)
                    } else {
                        println!("empty stack")
                    }
                }

                if buffer.len() == 0 {
                    continue;
                }

                buffer.reverse();
                for item in buffer {
                    stacks[to].push_back(item);
                }
            } else {
                break;
            }

            if self.debug_mode {
                print_stacks(&stacks);
            }
        }

        print!("Model 9001: ");
        print_top_of_stacks(stacks);
    }

    fn get_data(&self, stack_count: usize) -> (Vec<VecDeque<&str>>, Vec<(u32, usize, usize)>) {
        let mut stacks: Vec<VecDeque<&str>> = Vec::new();
        let mut instructions: Vec<(u32, usize, usize)> = Vec::new();

        for _i in 0..stack_count {
            stacks.push(VecDeque::new());
        }

        let mut stack_mode = true;
        for line in self.parser.get_string_vec() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with(" 1") {
                stack_mode = false;
                continue;
            }

            if stack_mode {
                for i in 0..stack_count {
                    let index = (i * 4) + 1;
                    let c = &line[index..index + 1];

                    if c != " " && !c.is_empty() {
                        stacks[i].push_front(c);
                    }
                }
            } else if line.starts_with("move") {
                //move 1 from 2 to 1
                let from_index = line.find(" from ").unwrap();
                let to_index = line.find(" to ").unwrap();

                let amount = line[5..from_index].trim().parse::<u32>().unwrap();
                let from = line[from_index + 6..to_index]
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                let to = line[to_index + 4..].trim().parse::<usize>().unwrap();

                instructions.push((amount, from - 1, to - 1));
            }
        }

        instructions.reverse();

        (stacks, instructions)
    }
}

pub fn part1() {
    let crane = Crane::new();
    crane.run_as_model_9000();
}

pub fn part2() {
    let crane = Crane::new();
    crane.run_as_model_9001();
}

fn print_top_of_stacks(stacks: Vec<VecDeque<&str>>) {
    print!("Top of stacks: ");
    for mut stack in stacks {
        if stack.len() > 0 {
            print!("{}", stack.pop_back().unwrap())
        }
    }
    println!();
}

fn print_stacks(stacks: &Vec<VecDeque<&str>>) {
    let mut c = 0;
    for stack in stacks {
        c += 1;
        print!("Stack {}:", c);
        for i in stack.iter() {
            print!("[{}]", i);
        }
        println!();
    }
}
