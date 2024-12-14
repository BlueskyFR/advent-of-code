const INPUT: &str = include_str!("../../inputs/day-01.txt");

fn solution() -> (i32, i32) {
    let mut calories = INPUT
        .split("\n\n") // Split by elf
        .map(|elf| {
            elf
                .lines()
                .map(|line| line.trim().parse::<i32>().expect("Expected a number"))
                .sum()
        })
        .collect::<Vec<i32>>();
    
    calories.sort();
    calories.reverse();
    
    (calories[0], calories[0..3].into_iter().sum())
}

fn main() {
    let (part1, part2) = solution();
    println!("Part 1: {part1} / Part 2: {part2}");
}
