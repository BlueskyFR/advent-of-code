use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-05.txt");

fn main() {
    let (rules, pages) = INPUT.split_once("\n\n").unwrap();

    let mut orderings = HashMap::<&str, HashSet<&str>>::new();

    // For each value, remember which ones have to come after it
    for rule in rules.lines() {
        let (a, b) = rule.split_once("|").unwrap();
        orderings.entry(a).or_default().insert(b);
    }

    let pages = pages.lines().map(|l| l.split(",").collect_vec());

    let (mut part_1, mut part_2) = (0, 0);
    for mut p in pages {
        if p.is_sorted_by(|a, b| orderings[a].contains(b)) {
            part_1 += p[p.len() / 2].parse::<usize>().unwrap();
        } else {
            p.sort_by(|a, b| orderings[a].contains(b).cmp(&true));
            part_2 += p[p.len() / 2].parse::<usize>().unwrap();
        }
    }

    println!("Part 1: {part_1} / Part 2: {part_2}");
}
