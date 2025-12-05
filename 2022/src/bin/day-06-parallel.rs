use std::thread;

const INPUT: &[u8; 4096] = include_bytes!("../../inputs/day-06.txt");
const WINDOW_SIZE: usize = 4;

fn process_slice(i: usize) -> usize {
  let slice: &[u8] = &INPUT[i + 1 - WINDOW_SIZE..=i];
  if !(1..4).any(|j| slice[j..].contains(&slice[j - 1])) {
    return i + 1
  }
  
  0
}

fn solution() -> usize {
    
    ((WINDOW_SIZE - 1)..INPUT.len()).map(|i| {
      let handle: thread::JoinHandle<usize> = thread::spawn(move || {
        return process_slice(i);
      });
      
      handle
    }).find_map(|handle| {
      let res: usize = handle.join().unwrap();
      if res == 0 { None } else { Some(res) }
    }).unwrap()
}

fn main() {
    println!("{}", solution());
}
