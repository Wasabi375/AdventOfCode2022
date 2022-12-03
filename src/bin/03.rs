use std::collections::HashSet;

use advent_of_code::helpers;

fn common_item(sets: &[HashSet<u8>]) -> u8 {
    assert!(sets.len() >= 2);

    let mut iter = sets.iter();

    let mut remaining = iter.next().unwrap().clone();

    for next in iter {
        let intersection: HashSet<u8> = remaining.intersection(next).cloned().collect();
        remaining.retain(|it| intersection.contains(it));
    }

    assert_eq!(remaining.len(), 1);

    *remaining.iter().next().unwrap()
}

fn item_set_from(str: &str) -> HashSet<u8> {
    let mut items = HashSet::with_capacity(str.len());
    for byte in str.as_bytes() {
        items.insert(*byte);
    }
    items
}

fn value_of(byte: u8) -> u32 {
    let value = match byte {
        b'a'..=b'z' => byte - b'a' + 1,
        b'A'..=b'Z' => byte - b'A' + 27,
        _ => panic!("unknown item {}", byte),
    };

    value as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in helpers::lines(input) {
        if line.is_empty() {
            continue;
        }
        assert!(line.len() % 2 == 0);

        let compartment1 = &line[0..line.len() / 2];
        let compartment2 = &line[line.len() / 2..];

        assert_eq!(line, format!("{}{}", compartment1, compartment2));

        let set1 = item_set_from(compartment1);
        let set2 = item_set_from(compartment2);

        sum += value_of(common_item(&[set1, set2]));
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = helpers::lines(input);

    let mut sum = 0;

    loop {
        let elf1 = lines.next();

        let set1 = match elf1 {
            None => break,
            Some("") => break,
            Some(content) => item_set_from(content),
        };
        let set2 = item_set_from(lines.next().unwrap());
        let set3 = item_set_from(lines.next().unwrap());

        sum += value_of(common_item(&[set1, set2, set3]));
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn match_intuition() {
        let line = Some("");

        let value = match line {
            None => 1,
            Some("") => 2,
            Some(_) => 3,
        };

        assert_eq!(value, 2);
    }
}
