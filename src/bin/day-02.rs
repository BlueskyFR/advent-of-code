const INPUT: &str = include_str!("../../inputs/day-02.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Action {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Action {
    // Returns what the current action wins against
    const fn beats(self) -> Action {
        match self {
            Action::ROCK => Action::SCISSORS,
            Action::PAPER => Action::ROCK,
            Action::SCISSORS => Action::PAPER,
        }
    }
    
    // Like beats() but reverse, so what it looses against
    const fn beaten_by(self) -> Action {
        match self {
            Action::SCISSORS => Action::ROCK,
            Action::ROCK => Action::PAPER,
            Action::PAPER => Action::SCISSORS,
        }
    }
}

// The score associated with each game outcome
enum Outcome {
    LOSS = 0,
    DRAW = 3,
    WIN = 6,
}

fn score_p1(elf_action: Action, my_action: Action) -> u32 {
    let outcome = if elf_action == my_action {
        Outcome::DRAW
    }
    // If the elf's action beats ours
    else if elf_action.beats() == my_action {
        Outcome::LOSS
    }
    // Otherwise we win since there is no other possibility
    else {
        Outcome::WIN
    };
    
    outcome as u32 + my_action as u32
}

// The outcome has to be the one specified in `strategy`
// and we just compute how much points it gives us
fn score_p2(elf_action: Action, strategy: Outcome) -> u32 {
    // What we are going to play with respect to the current strategy
    // We deduce our move from the elf's action
    let our_move = match strategy {
        Outcome::LOSS => elf_action.beats(),
        Outcome::DRAW => elf_action,
        Outcome::WIN => elf_action.beaten_by(),
    };
    
    // Return how much points is worth our strategy (outcome) + our move
    strategy as u32 + our_move as u32
}

// Returns the points for (part 1, part 2)
fn solution() -> (u32, u32) {
    let lines = INPUT
        // Split each line
        .lines();
    
    let p1 = lines.clone()
        // For each line (["X B", ...]), map X/Y/Z and A/B/C to rock/paper/scissors
        .map(|line| 
            line
                .split(" ")
                .map(|c| match c {
                    "X" | "A" => Action::ROCK,
                    "Y" | "B" => Action::PAPER,
                    "Z" | "C" => Action::SCISSORS,
                    _ => panic!(),
                })
                .collect::<Vec<Action>>()
        )
        // The first action is the elf's and the second one is ours
        .map(|l| {
            let [elf_action, my_action] = l[..] else { panic!() };
            
            // Compute our score based on the outcome
            // and sum it with the value of our action
            score_p1(elf_action, my_action)
        })
        // Get the total rewards score
        .sum();
    
    let p2 = lines.clone()
        .map(|line| {
            let [action, strategy] = line.split(" ").collect::<Vec<_>>()[..] else { panic!() };
            
            let action = match action {
                "A" => Action::ROCK,
                "B" => Action::PAPER,
                "C" => Action::SCISSORS,
                _ => panic!(),
            };
            
            let strategy = match strategy {
                "X" => Outcome::LOSS,
                "Y" => Outcome::DRAW,
                "Z" => Outcome::WIN,
                _ => panic!(),
            };
            
            score_p2(action, strategy)
        })
        .sum();
    
    (p1, p2)
}

fn main() {
    let (part1, part2) = solution();
    println!("Part 1: {part1} / Part 2: {part2}");
}
