use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-03.txt");

// Returns (argmax, max_value)
fn first_max_with_position(v: &[usize]) -> Option<(usize, usize)> {
    v.iter().enumerate().fold(None, |max, (index, &value)| {
        match max {
            // Init with the first element
            None => Some((index, value)),
            // Replace the max if it is strictly superior to it (keep the first occurrence)
            Some((_, max_value)) if value > max_value => Some((index, value)),
            // Otherwise do nothing
            _ => max,
        }
    })
}

fn largest_number_for_bank(n_digits: usize, bank: &[usize]) -> usize {
    let mut res = Vec::<usize>::with_capacity(n_digits);
    let mut next_start_position = 0;

    for i in (0..n_digits).rev() {
        let slice = &bank[next_start_position..(bank.len() - i)];
        let (argmax, max) = first_max_with_position(slice).unwrap();

        next_start_position += argmax + 1;
        res.push(max);
    }

    // Concat all digits together into a single one
    res.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn solution() -> (usize, usize) {
    let each_line_as_digits = INPUT.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec()
    });

    let part_1 = each_line_as_digits
        .clone()
        .map(|bank: Vec<usize>| largest_number_for_bank(2, &bank))
        .sum();

    let part_2 = each_line_as_digits
        .map(|bank: Vec<usize>| largest_number_for_bank(12, &bank))
        .sum();

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
