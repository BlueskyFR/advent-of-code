const INPUT: &str = include_str!("../../inputs/day-04.txt");

fn solution() -> (i32, i32) {
    let nums: Vec<[i32; 4]> = INPUT
        .lines()
        .map(|line| {
            let r: [i32; 4] = line
                .split(['-', ','])
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();
            
            r
        })
        .collect();
    
    let part1: i32 = nums.iter()
        .map(|[a, b, c, d]| {
            ((a - c) * (b - d) <= 0) as i32
        })
        .sum();
    
    let part2: i32 = nums.iter()
        .map(|[a, b, c, d]| {
            !(c > b || d < a) as i32
        })
        .sum();
    
    (part1, part2)
}

fn main() {
    let (part1, part2) = solution();
    println!("Part 1: {part1} / Part 2: {part2}");
}
