use std::ops::RangeInclusive;

use advent_of_code::helpers::lines_skip_empty;

fn from_description(range_description: &str) -> RangeInclusive<u32> {
    let mut part = range_description.split('-');

    let start = part.next().unwrap().parse::<u32>().unwrap();
    let end = part.next().unwrap().parse::<u32>().unwrap();

    RangeInclusive::new(start, end)
}

fn range_includes(base: &RangeInclusive<u32>, other: &RangeInclusive<u32>) -> bool {
    base.start() <= other.start() && base.end() >= other.end()
}

fn ranges_overlap(base: &RangeInclusive<u32>, other: &RangeInclusive<u32>) -> bool {
    base.contains(other.start())
        || base.contains(other.end())
        || other.contains(base.start())
        || other.contains(base.end())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in lines_skip_empty(input) {
        let mut ranges = line.split(',');
        let range_description1 = ranges.next().expect("Expected 2 rangese in each line");
        let range_description2 = ranges.next().expect("Expected 2 rangese in each line");

        assert_eq!(ranges.next(), None);

        let range1 = from_description(range_description1);
        let range2 = from_description(range_description2);

        if range_includes(&range1, &range2) || range_includes(&range2, &range1) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;

    for line in lines_skip_empty(input) {
        let mut ranges = line.split(',');
        let range_description1 = ranges.next().expect("Expected 2 rangese in each line");
        let range_description2 = ranges.next().expect("Expected 2 rangese in each line");

        assert_eq!(ranges.next(), None);

        let range1 = from_description(range_description1);
        let range2 = from_description(range_description2);

        if ranges_overlap(&range1, &range2) {
            count += 1;
        }
    }

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
