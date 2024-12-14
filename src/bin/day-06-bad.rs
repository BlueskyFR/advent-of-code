const INPUT: &[u8; 4096] = include_bytes!("../../inputs/day-06.txt");

fn solution() -> usize {
    const WINDOW_SIZE: usize = 4;

    for i in (WINDOW_SIZE - 1)..INPUT.len() {
        let slice: &[u8] = &INPUT[i + 1 - WINDOW_SIZE..=i];
        if !(1..4).any(|j| slice[j..].contains(&slice[j - 1])) {
            return i + 1
        }
    }

    return 0;
}

fn solution2() -> usize {
    const WINDOW_SIZE: usize = 14;

    for i in (WINDOW_SIZE - 1)..INPUT.len() {
        let slice: &[u8] = &INPUT[i + 1 - WINDOW_SIZE..=i];
        if !(1..14).any(|j| slice[j..].contains(&slice[j - 1])) {
            return i + 1
        }
    }

    return 0;
}

fn main() {
    println!("{}, {}", solution(), solution2());
}
