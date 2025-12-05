use itertools::Itertools;
use ndarray::Array;
use ndarray::Zip;
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

const INPUT: &[u8; 18905] = include_bytes!("../../inputs/day-04.txt");

fn solution() -> (usize, usize) {
    // Split the byte array into lines to be able to extract its shape
    let lines = INPUT
        .split(|c| *c == b'\n')
        .map(|row| row.iter().collect_vec())
        .collect_vec();

    let shape = (lines.first().unwrap().len(), lines.len());

    // Then parse the byte array into a single dimension vec
    let parsed_paper_rolls = INPUT
        .iter()
        .filter_map(|c| match c {
            b'.' => Some(0),
            b'@' => Some(1),
            _ => None,
        })
        .collect_vec();

    // dbg!(&parsed_paper_rolls);
    let mut paper_rolls = Array::from_shape_vec(shape, parsed_paper_rolls).unwrap();

    println!("{}", paper_rolls);

    // Given a 2D matrix (1 = paper roll, 0 = empty),
    // returns 1 (0-3 neighbors) or 0 (empty or 4+ neighbors)
    fn convolution(paper_rolls: &Array2<i32>) -> Array2<i32> {
        let kernel = array![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        let mut conved = paper_rolls
            .conv(&kernel, ConvMode::Same, PaddingMode::Zeros)
            .unwrap();

        // Differenciate paper stacks (0 to 8) from empty cells (-1)
        Zip::from(&mut conved)
            .and(paper_rolls)
            // Set cell to -1 if it is not a paper stack
            .for_each(|res, &rolls| *res = if rolls == 1 { *res } else { -1 });

        // Filter the paper rolls stacks having 4 or less neighbors (but more than 0!)
        conved.map_inplace(|c| *c = if *c >= 0 && *c < 4 { 1 } else { 0 });

        conved
    }

    let part_1 = convolution(&paper_rolls).sum() as usize;

    // Part 2
    let mut part_2: usize = 0;
    // Store a copy of the previous state so we can know whenever the algorithm converges
    let mut last_iteration = paper_rolls.clone();
    loop {
        // Get the list of collectable paper stacks
        let collectable = convolution(&paper_rolls);
        part_2 += collectable.sum() as usize;

        // Now remove those stacks from the original paper_rolls list and go again
        Zip::from(&mut paper_rolls)
            .and(&collectable)
            .for_each(|roll, &collectable| {
                if collectable == 1 {
                    *roll = 0
                }
            });

        // If the state did not change, then the algorithm converged!
        if paper_rolls == last_iteration {
            break;
        }

        last_iteration = paper_rolls.clone();
    }

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
