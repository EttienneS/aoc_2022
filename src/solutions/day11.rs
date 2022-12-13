// https://adventofcode.com/2022/day/11
use std::collections::{HashMap, VecDeque};

struct Monkey {
    operation: fn(u64) -> u64,
    divisor: u64,
    when_true: usize,
    when_false: usize,
}

pub fn part1(bench: bool) {
    let values = monkey_business(20, true, bench);
    let answer = values[0] * values[1];
    if !bench {
        println!("Day 11 Part 1 Solution: {:?}", answer);
    }
}

pub fn part2(bench: bool) {
    let values = monkey_business(10000, false, bench);
    let answer = values[0] * values[1];
    if !bench {
        println!("Day 11 Part 2 Solution: {:?}", answer);
    }
}

fn monkey_business(rounds: i32, decrease: bool, bench: bool) -> Vec<usize> {
    let monkeys = get_real_monkeys();
    let mut items = get_real_values();
    let mut inspection_map: HashMap<usize, usize> = HashMap::new();

    let common_denominator: u64 = monkeys.iter().map(|m| m.divisor).product();

    for round in 0..rounds {
        for (pos, monkey) in monkeys.iter().enumerate() {
            // make a copy of the current items for the monkey and clear
            let monkey_items = items[pos].clone();
            items[pos].clear();

            for item in monkey_items {
                inspection_map
                    .entry(pos)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                let mut worry_level: u64 = (monkey.operation)(item);
                if decrease {
                    worry_level = worry_level / 3;
                } else {
                    worry_level = worry_level % common_denominator;
                }

                if worry_level % monkey.divisor == 0 {
                    items[monkey.when_true].push_back(worry_level);
                } else {
                    items[monkey.when_false].push_back(worry_level);
                }
            }
        }

        if bench {
            continue;
        }

        if rounds <= 20 || rounds % 1000 == 0 {
            println!("After round {}:", round + 1);
            for (pos, items) in items.iter().enumerate() {
                println!("Monkey {pos} holds {:?}", items);
            }
            println!();
        }
    }
    let mut values: Vec<usize> = inspection_map.values().cloned().collect();
    values.sort();
    values.reverse();
    values
}

fn get_real_values() -> Vec<VecDeque<u64>> {
    let items: Vec<VecDeque<u64>> = vec![
        VecDeque::from([65, 58, 93, 57, 66]),
        VecDeque::from([76, 97, 58, 72, 57, 92, 82]),
        VecDeque::from([90, 89, 96]),
        VecDeque::from([72, 63, 72, 99]),
        VecDeque::from([65]),
        VecDeque::from([97, 71]),
        VecDeque::from([83, 68, 88, 55, 87, 67]),
        VecDeque::from([64, 81, 50, 96, 82, 53, 62, 92]),
    ];
    items
}
fn get_real_monkeys() -> Vec<Monkey> {
    let monkeys: Vec<Monkey> = vec![
        Monkey {
            operation: |old| old * 7,
            divisor: 19,
            when_true: 6,
            when_false: 4,
        },
        Monkey {
            operation: |old| old + 4,
            divisor: 3,
            when_true: 7,
            when_false: 5,
        },
        Monkey {
            operation: |old| old * 5,
            divisor: 13,
            when_true: 5,
            when_false: 1,
        },
        Monkey {
            operation: |old| old * old,
            divisor: 17,
            when_true: 0,
            when_false: 4,
        },
        Monkey {
            operation: |old| old + 1,
            divisor: 2,
            when_true: 6,
            when_false: 2,
        },
        Monkey {
            operation: |old| old + 8,
            divisor: 11,
            when_true: 7,
            when_false: 3,
        },
        Monkey {
            operation: |old| old + 2,
            divisor: 5,
            when_true: 2,
            when_false: 1,
        },
        Monkey {
            operation: |old| old + 5,
            divisor: 7,
            when_true: 3,
            when_false: 0,
        },
    ];

    monkeys
}

fn get_example_values() -> Vec<VecDeque<u64>> {
    let items = vec![
        VecDeque::from([79, 98]),
        VecDeque::from([54, 65, 75, 74]),
        VecDeque::from([79, 60, 97]),
        VecDeque::from([74]),
    ];
    items
}
fn get_example_monkeys() -> Vec<Monkey> {
    let monkeys: Vec<Monkey> = vec![
        Monkey {
            operation: |old| old * 19,
            divisor: 23,
            when_true: 2,
            when_false: 3,
        },
        Monkey {
            operation: |old| old + 6,
            divisor: 19,
            when_true: 2,
            when_false: 0,
        },
        Monkey {
            operation: |old| old * old,
            divisor: 13,
            when_true: 1,
            when_false: 3,
        },
        Monkey {
            operation: |old| old + 3,
            divisor: 17,
            when_true: 0,
            when_false: 1,
        },
    ];

    monkeys
}
