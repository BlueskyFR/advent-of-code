fn solution(windows_size: usize, bytes: &[u8]) -> usize {
    let mut size = 0;
    let mut buffer: [u8; 13] = [0; 13];
    let mut i = 0;

    loop {
        // check if the bytes is in the table
        for j in 0..size {
            // if a match is found, every bytes after the match are shifted
            if buffer[j] == bytes[i] {
                for k in (j + 1)..size {
                    buffer[k - (j + 1)] = buffer[k];
                }
                size -= j + 1;
            }
        }
        if size + 1 == windows_size {
            return i + 1;
        }
        buffer[size] = bytes[i];
        size += 1;
        i += 1;
    }
}

fn main() {
    // read file
    let bytes = include_bytes!("../../inputs/day-06.txt");

    println!("{}", solution(4, bytes).to_string());
    println!("{}", solution(14, bytes).to_string());
}