/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn lines_as_numbers(input: &str) -> impl Iterator<Item = Option<i32>> + '_ {
    let result = input
        .split('\n')
        .map(|it| -> Option<i32> { it.parse::<i32>().ok() });

    result
}

pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n')
}

pub fn lines_skip_empty(input: &str) -> impl Iterator<Item = &str> {
    lines(input).filter(|it| !it.is_empty())
}
