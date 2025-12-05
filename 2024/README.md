# Advent of Code 2024 - Rust 🦀

I love Rust I swear

## Dev workflow

1. Place inputs in `inputs/day-XX.txt`
2. Create a standalone binary by creating `src/bin/day-XX.rs` (`src/bin` is a cargo feature)
3. Run sample days with (using [cargo watch](https://github.com/watchexec/cargo-watch)):
    ```bash
    cargo watch -cqx 'run --bin day-XX'
    ```