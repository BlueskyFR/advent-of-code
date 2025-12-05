use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day-03.txt");

fn solution() -> (i32, i32) {
    // Create capturing groups using parentheses
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let part_1: i32 = re
        .captures_iter(INPUT)
        .map(|cap| {
            let [a, b] = cap
                // Extract the matched occurrences
                .extract::<2>()
                // Ignore the first arg (full captured string), and keep
                // the second one (list of capturing groups)
                .1
                // The matches are strings
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            a * b
        })
        // Finally sum all the multiplications together
        .sum();

    // Also scan for do() and don't()
    // "?:" are non-capturing groups
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\)|do(?:n't)?\(\))").unwrap();

    let mut enabled = true;
    let mut part_2 = 0;
    for c in re.captures_iter(INPUT) {
        let full_match = c.get(0).unwrap().as_str();

        if full_match == "do()" {
            enabled = true;
        } else if full_match == "don't()" {
            enabled = false;
        } else if enabled {
            let [a, b] = [c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()]
                .map(str::parse::<i32>)
                .map(Result::unwrap);

            part_2 += a * b;
        }
    }

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
