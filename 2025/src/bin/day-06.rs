use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-06.txt");

// Apply operands in `ops` to `numbers`
fn apply_ops(numbers: &[Vec<isize>], ops: &[u8]) -> isize {
    numbers
        .iter()
        .zip(ops)
        .map(|(numbers, op)| match op {
            b'*' => numbers.iter().product::<isize>(),
            b'+' => numbers.iter().sum::<isize>(),
            _ => unreachable!(),
        })
        .sum()
}

fn solution() -> (isize, isize) {
    let mut lines = INPUT.lines().collect_vec();
    // Get a list of * or + ops on the last line
    let ops = lines
        .pop()
        .unwrap()
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect_vec();

    // Part 1
    let mut numbers = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, number) in line.split_ascii_whitespace().enumerate() {
            if row == 0 {
                numbers.push(Vec::new());
            }

            // Store each number into a vector of its column
            numbers[col].push(number.parse().unwrap());
        }
    }

    let part_1 = apply_ops(&numbers, &ops);

    // Part 2
    let mut numbers = vec![vec![]];
    let mut col = 0;
    // len is in bytes
    for i in 0..lines[0].len() {
        let parsed_number = lines
            .iter()
            .map(|l| l.as_bytes()[i] as char)
            // Trim the string by keeping digits only
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        // The next column starts whenever a whitespace is found on all number lines at once
        if parsed_number.is_empty() {
            // Create a new entry if we are at the start of a new number
            numbers.push(Vec::new());
            // Move to the next column
            col += 1;
            continue;
        }

        // Which we can then parse
        numbers[col].push(parsed_number.parse().unwrap());
    }

    let part_2 = apply_ops(&numbers, &ops);

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
