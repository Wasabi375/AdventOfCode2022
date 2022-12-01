/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

pub fn lines_as_numbers(input: &str) -> Vec<Option<i32>> {
    return input.split('\n')
        .map(|it| -> Option<i32> {
            it.parse::<i32>().ok()
            
    }).collect::<Vec<Option<i32>>>();
}