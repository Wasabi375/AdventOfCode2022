use advent_of_code::helpers::lines_as_numbers;

pub fn part_one(input: &str) -> Option<i32> {
    let lines = lines_as_numbers(input);

    let mut max: i32 = 0;
    let mut current: i32 = 0;

    for line in lines {
        match line {
            Some(calories) => {
                current += calories;
            }
            None => {
                if current > max {
                    max = current;
                }
                current = 0;
            }
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut lines: Vec<_> = lines_as_numbers(input).collect();
    lines.push(None);

    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    let mut current = 0;

    for line in lines {
        match line {
            Some(calories) => {
                current += calories;
            }
            None => {
                if current > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = current;
                } else if current > max2 {
                    max3 = max2;
                    max2 = current;
                } else if current > max3 {
                    max3 = current;
                }
                current = 0;
            }
        }
    }

    Some(max1 + max2 + max3)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
