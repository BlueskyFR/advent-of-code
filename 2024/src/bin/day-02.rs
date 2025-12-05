#![feature(iter_map_windows)]

use itertools::{Itertools, all};

const INPUT: &str = include_str!("../../inputs/day-02.txt");

fn solution() -> (i32, i32) {
    fn is_safe(line: &Vec<i32>) -> bool {
        // Compute the diffs between 2 consecutive values
        let diffs = line.iter().map_windows(|[x, y]| *x - *y).collect_vec();

        // Now run our checks on the line
        // 1. Check if strictly increasing or decreasing (i.e. all signs are the same)
        let signs_sum = diffs
            .clone()
            .into_iter()
            .map(i32::signum)
            .sum::<i32>()
            .abs();

        // 2. abs diffs must be <= 3
        let abs_diffs = diffs.clone().into_iter().map(i32::abs);

        // Put both into a condition, returning true if the line is "Safe"
        signs_sum == diffs.len() as i32 && all(abs_diffs, |x| x <= 3)
    }

    let parsed_input = INPUT.lines().map(|line| {
        line.split_whitespace()
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect_vec()
    });

    let part_1 = parsed_input.clone().filter(is_safe).count() as i32;

    // Part 2

    let part_2: i32 = parsed_input
        .filter(|line| {
            // Check if removing any element in the vec makes it safe
            (0..(line.len())).any(|i| {
                let mut l = line.clone();
                l.remove(i);
                is_safe(&l)
            }) || is_safe(line)
        })
        .count() as i32;

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
