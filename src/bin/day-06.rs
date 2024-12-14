const INPUT: &[u8; 4096] = include_bytes!("../../inputs/day-06.txt");

fn solution(window_size: usize) -> usize {
    let mut correct: [u8; 13] = [0; 13];
    // Cursor that indicates the next free element's id
    let mut curr_id: usize = 0;
    
    // For every char of the input
    'char_loop: for i in 0..INPUT.len() {
        // Check current char (byte) against every unique
        // previous char found so far
        for j in 0..curr_id {
            // If identical to a previous unique value
            if INPUT[i] == correct[j] {
                // We delete the previous j elements
                for k in (j + 1)..curr_id {
                    correct[k - (j + 1)] = correct[k];
                }
                
                curr_id -= j;
                
                // Also insert the current element at the end
                correct[curr_id - 1] = INPUT[i];
                
                continue 'char_loop;
            }
        }
        
        // Char is unique so count it as part of the unique chars
        if curr_id + 1 == window_size {
            return i + 1;
        }
        correct[curr_id] = INPUT[i];
        curr_id += 1;
    }

    return 0;
}

fn main() {
    println!("Part 1: {} / Part 2: {}", solution(4), solution(14));
}
