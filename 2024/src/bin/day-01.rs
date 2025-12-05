use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-01.txt");

fn solution() -> (i32, i32) {
    let nums: Vec<(i32, i32)> = INPUT
        .lines()
        .map(|line| {
            let r: (i32, i32) = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap();

            r
        })
        .collect();

    let (mut first_col, mut second_col): (Vec<i32>, Vec<i32>) = nums.into_iter().unzip();

    first_col.sort();
    second_col.sort();

    let part_1 = first_col
        .iter()
        .zip(second_col.clone())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    // Part 2

    let second_col_counts = second_col.into_iter().counts();

    let part_2 = first_col
        .into_iter()
        .map(|x| x * (*second_col_counts.get(&x).unwrap_or(&0) as i32))
        .sum();

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
