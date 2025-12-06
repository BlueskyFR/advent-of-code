# Advent of Code 2025 - Rust 🦀

## Dev workflow

1. Place inputs in `inputs/day-XX.txt`
2. Create a standalone binary by creating `src/bin/day-XX.rs` (`src/bin` is a cargo feature)
3. Run sample days with (using [bacon](https://github.com/Canop/bacon)):
    ```bash
    bacon -j run -- --quiet --bin day-XX
    ```
    or
    ```
    # debug (default)
    ./run.sh day-XX
    # release
    ./run.sh day-XX --release
    ```

## Template `day-XX.rs`

```rust
const INPUT: &str = include_str!("../../inputs/day-XX.txt");

fn solution() -> (usize, usize) {
    INPUT.lines()
    
    let part_1 = 1;
    let part_2 = 2;

    (part_1, part_2)
}

fn main() {
    let (part_1, part_2) = solution();
    println!("Part 1: {part_1} / Part 2: {part_2}");
}
```

## Retrieve all inputs

```bash
mkdir -p inputs && cd inputs
for i in $(seq 1 12); do
  curl "https://adventofcode.com/2025/day/$i/input" \
  -b 'session=<your session cookie value here>' \
  -o day-$(printf "%02d" $i).txt
done;
```