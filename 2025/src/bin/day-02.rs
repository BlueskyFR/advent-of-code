use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-02.txt");

fn solution() -> (usize, usize) {
    let ranges = INPUT.split(',').map(|range| {
        let (start, end) = range.split_once('-').unwrap();
        start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
    });

    let part_1 = ranges.clone().fold(0, |total, range| {
        let mut subtotal = 0;

        for n in range {
            // Convert to string
            let s = n.to_string();
            let len = s.len();

            // Skip non-even str lengths
            if len % 2 == 1 {
                continue;
            }
            // Increase the total if the string is a mirror of itself
            if s[..(len / 2)] == s[(len / 2)..] {
                subtotal += n;
            }
        }

        total + subtotal
    });

    fn chars_chunks(n: usize, str: &str) -> Vec<String> {
        // Extract chars from the string
        let chars: Vec<char> = str.chars().collect();

        // Create groups of n chars
        let split = chars
            .chunks(n)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>();

        split
    }

    // Better: https://www.reddit.com/r/adventofcode/comments/1pcbzbd/2025_day_2_part_2_rust_is_too_elegant/
    let part_2 = ranges.fold(0, |total, range| {
        let mut subtotal = 0;

        for n in range {
            // Convert to string
            let s = n.to_string();
            let len = s.len();

            for i in 1..=(len / 2) {
                // Skip if the str len cannot be divided by this substr len
                if len % i != 0 {
                    continue;
                }

                if chars_chunks(i, &s).iter().all_equal() {
                    subtotal += n;
                    break;
                }
            }
        }

        total + subtotal
    });

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
