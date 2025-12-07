const INPUT: &str = include_str!("../../inputs/day-07.txt");

fn solution() -> (usize, usize) {
    // Create a vec with the same with as our input file
    let line_width = INPUT.lines().next().unwrap().len();
    // Stores the timelines count + hence if there is a beam at that position (> 0) or not (= 0)
    let mut beam_positions: Vec<usize> = vec![0; line_width];

    // Part 1: count the split count
    let mut part_1 = 0;

    for line in INPUT.lines() {
        for (position, c) in line.bytes().enumerate() {
            match c {
                b'S' => beam_positions[position] = 1,
                b'^' => {
                    // Beam is splitted
                    if beam_positions[position] > 0 {
                        // += makes merge add their beam value together
                        beam_positions[position - 1] += beam_positions[position];
                        beam_positions[position + 1] += beam_positions[position];
                        beam_positions[position] = 0;

                        part_1 += 1;
                    }
                }
                _ => continue,
            }
        }
    }

    let part_2 = beam_positions.iter().sum();

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
