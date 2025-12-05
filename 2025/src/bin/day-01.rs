const INPUT: &str = include_str!("../../inputs/day-01.txt");

fn solution() -> (isize, isize) {
    // Part 1
    let code_1 = INPUT
        .lines()
        // Accumulator
        .fold((50isize, 0), |(previous_pos, total), direction| {
            // Parse 'L40' or 'R40' to 40
            let mut amount: isize = str::parse(&direction[1..]).unwrap();
            // If direction starts with an 'L', the amount should be negative
            if &direction[..1] == "L" {
                amount *= -1;
            }

            // Rust's modulo can be negative but it doesn't matter for this one
            let new_pos = (previous_pos + amount) % 100;

            (new_pos, total + (new_pos == 0) as isize)
        })
        .1;

    // Part 2
    let code_2 = INPUT
        .lines()
        .fold((50isize, 0), |(previous_pos, total), direction| {
            // Parse 'L40' or 'R40' to 40
            let mut amount: isize = str::parse(&direction[1..]).unwrap();
            // If direction starts with an 'L', the amount should be negative
            if &direction[..1] == "L" {
                amount *= -1;
            }

            // Add the amount to our previous position
            let new_pos = previous_pos + amount;

            // Compute the absolute number of "turns" we performed with that amount combined
            // to the previous offset we had
            let mut turns = (new_pos / 100).abs();
            // Edge case: going from positive to negative (e.g. from 20 to -20) doesn't count as a turn
            // by dividing by 100 so we'll add 1 manually;
            // Second edge case: stopping on 0 should count as a turn even, so count it the first time
            // but not the second time if we are actually leaving it.
            if new_pos <= 0 && previous_pos != 0 {
                turns += 1;
            }

            (
                // Just keep the nonnegative remainder over our clock,
                // to keep track of our position offset (so 0 <= pos < 100)
                new_pos.rem_euclid(100),
                total + turns,
            )
        })
        .1;

    (code_1, code_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
