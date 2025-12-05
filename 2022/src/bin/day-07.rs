const INPUT: &str = include_str!("../../inputs/day-07.txt");

struct Node {
    value: u32,
    children: Vec<Node>,
}

fn solution() -> (i32, i32) {
    let mut root = Node {
        value: 0,
        children: Vec::new(),
    };
    let loc = &mut root;

    for elements in INPUT.split("$ ls") {
        for line in elements.lines() {
            let t: [&str; 2] = line
                .split_whitespace()
                .take(2)
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();

            if t[0] == "$ cd .." {
                
            } else if t[0].starts_with("$ cd") {
                loc
            } else if t[0] != "dir" {
                loc.children.push(Node {
                    value: str::parse::<u32>(t[0]).unwrap(),
                    children: Vec::new(),
                });
            }
        }
    }

    (3, 4)
}

fn main() {
    let (part1, part2) = solution();
    println!("Part 1: {part1} / Part 2: {part2}");
}
