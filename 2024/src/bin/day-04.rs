use itertools::{Itertools, iproduct};

const INPUT: &[u8; 19739] = include_bytes!("../../inputs/day-04.txt");

#[derive(Clone)]
struct PatternSearcher<'w> {
    matrix: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,

    word_to_find: &'w str,
    word_to_find_chars: Vec<char>,
}

impl<'w> PatternSearcher<'w> {
    fn new(matrix: Vec<Vec<char>>, word_to_find: &'w str) -> Self {
        Self {
            max_x: matrix[0].len() as i32,
            max_y: matrix.len() as i32,
            matrix,
            word_to_find,
            word_to_find_chars: word_to_find.chars().collect_vec(),
        }
    }

    fn search(self, coords: (i32, i32), direction: (i32, i32), mut word: String) -> usize {
        let (x, y) = coords;

        // Check if OOB
        if x < 0 || x >= self.max_x || y < 0 || y >= self.max_y {
            return 0;
        }

        // Collect char at location
        let char: char = self.matrix[x as usize][y as usize];
        // Ensure this is the next char we expected
        if char != self.word_to_find_chars[word.len()] {
            return 0;
        }

        word.push(char);
        // Return 1 if we got a full match!
        if word == self.word_to_find {
            1
        } else {
            self.search((x + direction.0, y + direction.1), direction, word)
        }
    }

    fn search_all_directions(self, coords: (i32, i32)) -> usize {
        // List all the directions to search in ((-1, 0), (1, 1)...) but skip (0, 0)
        let all_directions = iproduct!(-1..=1, -1..=1).filter(|(x, y)| *x != 0 || *y != 0);

        all_directions
            .map(|direction| self.clone().search(coords, direction, String::new()))
            .sum()
    }

    fn search_part_2(self, coords: (usize, usize)) -> usize {
        let (x, y) = coords;
        // All coords in the squard around the current point should be valid as well
        if x == 0 || y == 0 || x >= self.max_x as usize - 1 || y >= self.max_y as usize - 1 {
            return 0;
        }

        // The center letter should be 'A'
        if self.matrix[x][y] != 'A' {
            return 0;
        }

        let (a, aa, b, bb) = (
            self.matrix[x - 1][y - 1],
            self.matrix[x + 1][y + 1],
            self.matrix[x + 1][y - 1],
            self.matrix[x - 1][y + 1],
        );

        if ((a == 'M' && aa == 'S') || (a == 'S' && aa == 'M'))
            && ((b == 'M' && bb == 'S') || (b == 'S' && bb == 'M'))
        {
            1
        } else {
            0
        }
    }
}

fn solution() -> (usize, usize) {
    // Convert the text (bytes) file to a char matrix
    let matrix: Vec<Vec<char>> = INPUT
        .map(|b| b as char)
        .split(|b| *b == '\n')
        .map(|a| a.to_vec())
        .collect_vec();

    let (max_x, max_y) = (matrix[0].len(), matrix.len());
    let searcher = PatternSearcher::new(matrix, "XMAS");
    let part_1 = iproduct!(0..max_x, 0..max_y)
        .map(|(x, y)| searcher.clone().search_all_directions((x as i32, y as i32)))
        .sum();

    let part_2 = iproduct!(0..max_x, 0..max_y)
        .map(|c| searcher.clone().search_part_2(c))
        .sum();

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
