// https://adventofcode.com/2022/day/2
use crate::helpers::input_parser::InputParser;

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum DesiredOutcome {
    Win,
    Lose,
    Draw,
}

pub fn part1() {
    run_game(&get_player_move);
}

pub fn part2() {
    run_game(&get_move_to_reach_outcome);
}

fn run_game(get_player_move: &dyn Fn(&str, &Move) -> Move) {
    let parser = InputParser::new("src/input/day2.txt");
    let lines = parser.get_string_vec();
    let mut total: u32 = 0;
    for line in lines {
        let opponent = get_opponent_move(line);
        let player = get_player_move(line, &opponent);

        let score = calculate_score(opponent, player);
        total += score;
    }
    println!("Part 1 Total {}", total);
}

fn get_move_to_reach_outcome(line: &str, opponent: &Move) -> Move {
    let outcome = get_desired_outcome(line);
    let player: Move = match outcome {
        DesiredOutcome::Win => match *opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        DesiredOutcome::Lose => match *opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        DesiredOutcome::Draw => match *opponent {
            Move::Rock => Move::Rock,
            Move::Paper => Move::Paper,
            Move::Scissors => Move::Scissors,
        },
    };
    player
}

fn get_player_move(line: &str, _opponent: &Move) -> Move {
    match line.chars().nth(2).unwrap() {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        _ => Move::Scissors,
    }
}

fn get_desired_outcome(line: &str) -> DesiredOutcome {
    match line.chars().nth(2).unwrap() {
        'X' => DesiredOutcome::Lose,
        'Y' => DesiredOutcome::Draw,
        _ => DesiredOutcome::Win,
    }
}

fn get_opponent_move(line: &str) -> Move {
    match line.chars().nth(0).unwrap() {
        'A' => Move::Rock,
        'B' => Move::Paper,
        _ => Move::Scissors,
    }
}

fn calculate_score(opponent: Move, player: Move) -> u32 {
    let mut score: u32 = 0;
    match player {
        Move::Rock => {
            score = score + 1;
            score += match opponent {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            }
        }
        Move::Paper => {
            score = score + 2;
            score += match opponent {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            }
        }
        Move::Scissors => {
            score = score + 3;
            score += match opponent {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            }
        }
    };

    score
}
