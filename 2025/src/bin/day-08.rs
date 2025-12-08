use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day-08.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    // All coords are positive
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    // Returns the euclidian distance between 2 points (always positive)
    fn distance_to(&self, other: &Point) -> f32 {
        (
            (self.x as f32 - other.x as f32).powi(2) +
            (self.y as f32 - other.y as f32).powi(2) +
            (self.z as f32 - other.z as f32).powi(2)
        ).sqrt()
    }
}

fn solution() -> (usize, usize) {
    // Maps a point to its cluster id
    let mut point_to_cluster_id: HashMap<Point, usize> = INPUT.lines().enumerate()
        .map(|(id, line)| {
            let (x, y, z) = line.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            
            // Assign each point a unique cluster id (each point is its own cluster for now)
            (Point { x, y, z }, id)
        })
        .collect();

    // For each combination of points (n² complexity), compute the distance between the 2 points;
    // then sort by distance (asc)
    let closest_pairs = point_to_cluster_id.keys().tuple_combinations()
        // Copy each point by dereferencing it (Point has the copy trait)
        .map(|(a, b)| (*a, *b, a.distance_to(b)))
        // Sort by distance
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2))
        .collect_vec();
    
    let mut part_1 = 0;
    let mut part_2 = 0;
    for (i, (a, b, _distance)) in closest_pairs.iter().enumerate() {
        // Iterate over the first 1000 pairs only
        if i == 1000 {
            part_1 = point_to_cluster_id.values().counts().values().sorted().rev().take(3).product::<usize>();
        }
        
        // Set b's cluster id to a's
        let a_cluster_id = *point_to_cluster_id.get(a).unwrap();
        let b_cluster_id = *point_to_cluster_id.get(b).unwrap();
        
        if a_cluster_id != b_cluster_id && point_to_cluster_id.values().unique().count() == 2 {
            part_2 = a.x as usize * b.x as usize;
        }
        
        // Change the cluster id for b to a's, but also to every other point that had the same as b
        point_to_cluster_id.iter_mut()
            .filter(|(_, cluster_id)| **cluster_id == b_cluster_id)
            .for_each(|(_, cluster_id)| *cluster_id = a_cluster_id);
    }
    
    (part_1 , part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
