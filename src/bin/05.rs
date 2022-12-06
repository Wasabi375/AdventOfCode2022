use advent_of_code::helpers::lines;
use itertools::{all, Itertools};
use std::cmp::max;

pub fn parse_crate_stacks<'a, I>(lines: &mut I) -> Vec<Vec<char>>
where
    I: Iterator<Item = &'a str>,
{
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut count_line_found = false;

    let mut last_stack_index: i32 = -1;

    while !count_line_found {
        let line = lines.next().unwrap();

        line.chars()
            .chunks(4)
            .into_iter()
            .map(|it| it.into_iter().collect::<String>())
            .enumerate()
            .for_each(|(stack_index, crate_description)| {
                let trimmed = crate_description.trim();
                last_stack_index = max(last_stack_index, stack_index as i32);

                if trimmed.is_empty() {
                    return;
                }

                if all(trimmed.chars(), |it| it.is_numeric()) {
                    count_line_found = true;
                    stacks[stack_index].reverse();
                } else {
                    let crate_char = trimmed.chars().nth(1).unwrap();

                    while stacks.len() <= stack_index {
                        stacks.push(Vec::new());
                    }

                    stacks[stack_index].push(crate_char);
                }
            });
    }

    assert_eq!(stacks.len() as i32, last_stack_index + 1);

    stacks
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_instruction(text: &str) -> Instruction {
    let mut input = text.split(' ');

    assert_eq!(input.next().unwrap(), "move");
    let count = input.next().unwrap().parse().unwrap();

    assert_eq!(input.next().unwrap(), "from");
    let from = input.next().unwrap().parse::<usize>().unwrap() - 1;

    assert_eq!(input.next().unwrap(), "to");
    let to = input.next().unwrap().parse::<usize>().unwrap() - 1;

    assert!(input.next().is_none());

    assert_ne!(from, to);

    Instruction { from, to, count }
}

fn move_crates_one_by_one(stacks: &mut [Vec<char>], instruction: Instruction) {
    for _ in 0..instruction.count {
        let value = stacks[instruction.from].pop().unwrap();
        stacks[instruction.to].push(value);
    }
}

fn move_crates_all_at_once(stacks: &mut [Vec<char>], instruction: Instruction) {
    let from = stacks.get_mut(instruction.from).unwrap();

    let at = from.len() - instruction.count;
    let mut to_move = from.split_off(at);

    stacks[instruction.to].append(&mut to_move);
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = lines(input);

    let mut stacks = parse_crate_stacks(&mut lines);

    assert!(lines.next().unwrap().is_empty());

    lines
        .take_while(|it| !it.is_empty())
        .map(parse_instruction)
        .for_each(|it| move_crates_one_by_one(&mut stacks, it));

    Some(stacks.iter().map(|it| it.last().unwrap_or(&' ')).join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines = lines(input);

    let mut stacks = parse_crate_stacks(&mut lines);

    assert!(lines.next().unwrap().is_empty());

    lines
        .take_while(|it| !it.is_empty())
        .map(parse_instruction)
        .for_each(|it| move_crates_all_at_once(&mut stacks, it));

    Some(stacks.iter().map(|it| it.last().unwrap_or(&' ')).join(""))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
