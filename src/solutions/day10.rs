// https://adventofcode.com/2022/day/X
use crate::helpers::input_parser::InputParser;

const INPUTFILE: &str = "src/input/day10.txt";

pub fn solution() {
    let total: i32 = get_registers().iter().sum();
    println!("Day X Part 1 Solution: {:?}", total);
}

fn get_registers() -> Vec<i32> {
    let parser = InputParser::new(INPUTFILE);

    let mut commands: Vec<(i32, i32)> = parser
        .get_string_vec()
        .iter()
        .map(|line| {
            let args: Vec<&str> = line.split_ascii_whitespace().collect();
            match args[..] {
                ["noop"] => (1, 0),
                ["addx", value] => (2, value.parse::<i32>().unwrap()),
                _ => panic!("Unknown command"),
            }
        })
        .rev()
        .collect();
    let mut current = commands.pop().unwrap();
    let mut clock = 0;
    let mut x = 1;
    let mut registers: Vec<i32> = vec![];
    let mut remaining_command_time = current.0;

    let line_width = 40;
    loop {
        clock += 1;
        remaining_command_time -= 1;

        if (clock - 20) % 40 == 0 {
            registers.push(clock * x);
        }

        if remaining_command_time == 0 {
            x += current.1;
            if let Some(c) = commands.pop() {
                current = c;
            } else {
                break;
            }
            remaining_command_time = current.0;
        }

        let line_clock = clock % line_width;
        if line_clock == x - 1 || line_clock == x || line_clock == x + 1 {
            print!("▓");
        } else {
            print!(" ");
        }

        if clock % line_width == 0 {
            println!();
        }
    }

    println!();
    registers
}
