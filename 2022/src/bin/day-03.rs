use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day-03.txt");

fn split_pockets(rucksack: &str) -> &[&str] {
    [
        // Each half of the string is a pocket
        &rucksack[0..(rucksack.len() / 2)],
        &rucksack[(rucksack.len() / 2)..],
    ]
}

fn find_common_chars(strings: &[&str]) -> Vec<char> {
    if strings.is_empty() {
        return Vec::new();
    }
    
    let mut res: HashSet<_> = strings[0].chars().collect();
    for str in &strings[1..] {
        let chars: HashSet<_> = str.chars().collect();
        res = res.intersection(&chars).cloned().collect();
    }
    
    res.into_iter().collect()
}

fn to_priorities(types: Vec<char>) -> Vec<u32> {
    types.into_iter()
        .map(|c: char|
            c
                .to_digit(36)
                .expect("Can't be None") - 9 + (if c.is_uppercase() { 26 } else { 0 }))
        .collect()
}

fn solution() -> (u32, u32) {
    let p1_priorities: u32 =
        INPUT
            .lines()
            .map(split_pockets)
            .map(find_common_chars)
            .map(to_priorities)
            .map(|priorities| priorities.into_iter().sum::<u32>())
            .sum::<u32>();
    
    let p2_priorities: u32 =
        INPUT
            .lines().collect::<Vec<_>>()
            .chunks(3) // check error here too
            .map(find_common_chars) // find common chars between a, b and c
            .map(to_priorities)
            .map(|priorities| priorities.iter().sum::<u32>())
            .sum::<u32>();
    
    (p1_priorities, p2_priorities)
}

fn main() {
    let (part1, part2) = solution();
    println!("Part 1: {part1} / Part 2: {part2}");
}
