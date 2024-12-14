const INPUT: &str = include_str!("../../inputs/day-05.txt");

fn solution(move_in_batch: bool) {
    let lines: Vec<&str> = INPUT.lines().collect();
  
    let mut stacks: [Vec<char>; 9] = Default::default();
    
    // Fill the stacks from the bottom
    for line in lines[0..8].iter().rev() {
        // Map the current line to chars
        let chars: Vec<char> = line.chars().collect();
        
        for i in 0..9 {
            let c = chars[i * 4 + 1];
            if !c.is_whitespace() {
                stacks[i].push(c);
            }
        }
    }
    
    // Parse ops
    for line in lines[10..].iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        
        let howmuch: usize = words[1].parse::<usize>().unwrap();
        let from: usize = words[3].parse::<usize>().unwrap() - 1;
        let to: usize = words[5].parse::<usize>().unwrap() - 1;
        
        if move_in_batch {
            let crates: Vec<char> =
                stacks[from]
                    .drain(stacks[from].len() - howmuch..)
                    .collect();
            stacks[to].extend(crates);
        } else {
            for _ in 0..howmuch {
                let poped = stacks[from].pop();
                
                if poped.is_none() {
                    println!("Can't move {howmuch} from {from} to {to} because there is only {} on {from}", stacks[from].len());
                    continue;
                }
                
                stacks[to].push(poped.unwrap());
            }
        }
    }
    
    // Print the create at the top of each stack
    let mut res: String = "".to_owned();
    for mut stack in stacks {
        let c = stack.pop();
        if !c.is_none() {
            res.push_str(&c.unwrap().to_string());
        }
    }
    
    println!("{}", res);
}

fn main() {
    solution(false);
    solution(true);
}
