use std::{cmp::max, ops::RangeInclusive};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-05.txt");

fn solution() -> (usize, usize) {
    let (fresh_ingredients, ingredients) = INPUT.split_once("\n\n").unwrap();

    let fresh_ingredients_ranges = fresh_ingredients
        .lines()
        // Map to a Rust range
        .map(|str| {
            let (start, end) = str.split_once('-').unwrap();
            start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
        })
        .collect_vec();

    let part_1 = ingredients
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        // Keep only the fresh ingredients
        .filter(|i| {
            fresh_ingredients_ranges
                .iter()
                // Find if any range contains that ID
                .any(|range| range.contains(i))
        })
        // And return the number of fresh ingredients we have!
        .count();

    // Merge all intervals and count the number of values
    let part_2 = merge_intervals(fresh_ingredients_ranges)
        .iter()
        .map(|range| range.clone().count())
        .sum();

    (part_1, part_2)
}

fn merge_intervals(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|r| *r.start());

    // Since the ranges are now sorted by start id, we can merge them together!
    ranges.iter().fold(Vec::new(), |mut merged_ranges, range| {
        if let Some(last_range) = merged_ranges.last().cloned()
            && range.start() <= last_range.end()
        {
            merged_ranges.pop();
            merged_ranges.push(*last_range.start()..=*max(last_range.end(), range.end()));

            return merged_ranges;
        }

        merged_ranges.push(range.clone());
        merged_ranges
    })
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
