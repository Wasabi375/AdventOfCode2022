#[derive(Eq, PartialEq, Debug, Ordinalize, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn new(char: char) -> Move {
        match char {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("unknown move '{}'", char),
        }
    }

    fn get_score(self: &Move) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum RoundResult {
    Win,
    Loose,
    Draw,
}

impl RoundResult {
    fn new(char: char) -> RoundResult {
        match char {
            'X' => RoundResult::Loose,
            'Y' => RoundResult::Draw,
            'Z' => RoundResult::Win,
            _ => panic!("unknown round result '{}'", char),
        }
    }

    fn get_score(self: &RoundResult) -> u32 {
        match self {
            RoundResult::Win => 6,
            RoundResult::Loose => 0,
            RoundResult::Draw => 3,
        }
    }
}

fn get_result(elf: &Move, player: &Move) -> RoundResult {
    if elf == player {
        return RoundResult::Draw;
    }

    let elf_score = elf.get_score() - 1;
    let player_score = player.get_score() - 1;

    if elf_score == (player_score + 1) % 3 {
        RoundResult::Win
    } else {
        RoundResult::Loose
    }
}

fn get_player_move(elf: &Move, result: &RoundResult) -> Move {
    let offset = match result {
        RoundResult::Draw => return elf.clone(),
        RoundResult::Loose => -1,
        RoundResult::Win => 1,
    };

    let result_ordinal = (elf.ordinal() + offset + 3) % 3;

    Move::from_ordinal(result_ordinal).unwrap()
}

use advent_of_code::helpers::lines;
use enum_ordinalize::Ordinalize;

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;

    for round in lines(input) {
        if round.is_empty() {
            continue;
        }
        assert_eq!(round.len(), 3);
        assert_eq!(round.chars().nth(1), Some(' '));

        let elf = Move::new(round.chars().next().unwrap());
        let player = Move::new(round.chars().nth(2).unwrap());

        score += player.get_score() + get_result(&player, &elf).get_score();
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;

    for round in lines(input) {
        if round.is_empty() {
            continue;
        }
        assert_eq!(round.len(), 3);
        assert_eq!(round.chars().nth(1), Some(' '));

        let elf_move = Move::new(round.chars().next().unwrap());
        let expected_result = RoundResult::new(round.chars().nth(2).unwrap());

        let player_move = get_player_move(&elf_move, &expected_result);

        score += player_move.get_score() + expected_result.get_score();
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors() {
        assert_eq!(get_result(&Move::Rock, &Move::Rock), RoundResult::Draw);
        assert_eq!(get_result(&Move::Rock, &Move::Paper), RoundResult::Loose);
        assert_eq!(get_result(&Move::Rock, &Move::Scissors), RoundResult::Win);

        assert_eq!(get_result(&Move::Paper, &Move::Rock), RoundResult::Win);
        assert_eq!(get_result(&Move::Paper, &Move::Paper), RoundResult::Draw);
        assert_eq!(
            get_result(&Move::Paper, &Move::Scissors),
            RoundResult::Loose
        );

        assert_eq!(get_result(&Move::Scissors, &Move::Rock), RoundResult::Loose);
        assert_eq!(get_result(&Move::Scissors, &Move::Paper), RoundResult::Win);
        assert_eq!(
            get_result(&Move::Scissors, &Move::Scissors),
            RoundResult::Draw
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
