# Advent of Code Helper Library

A library that lets you focus on problem-solving rather than boilerplate when doing Advent of Code.

This library utilises Rust's testing framework to run each part of each day with either real live inputs from [adventofcode.com](https://adventofcode.com) or your own test inputs.

It also contains a few utility functions to speed up common input transformations.

## Installation

Place the following in your `Cargo.toml` dependencies section:

```toml
[dependencies]
aoc-lib = "1.0.0"
```

or you can do a `cargo add aoc-lib`.

## Usage

### Cookie

The auto-download functionality needs a cookie to authenticate with AOC.

Get the `adventofcode.com` cookie **value** from your browser and place it in a `~/.aoc.cookie` file (name: `.aoc.cookie`, in your home folder).

There's many browser extensions that let you view your cookies.

### Your Repository

Follow the below example as closely as possible:

```
src
├── lib.rs
└── year2022
    ├── day01.rs
    └── mod.rs
```

The year folders must follow the regex `^year\d\d\d\d$` and the days must follow the regex `^day\d\d\.rs$`.

Hint: To avoid `unused` warnings, your `mod.rs` file should use pub: `pub mod dayXX` and similarly for your `lib.rs`: `pub mod year2022`.

Here's a template `dayXX.rs` file:

```rust
pub fn part1(_input: String) -> usize {
    0
}

pub fn part2(_input: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::*;

    #[test]
    fn test_part1() {
        run_test!(part1);
    }

    #[test]
    fn run_part1() {
        run_real!(part1);
    }

    #[test]
    fn test_part2() {
        run_test!(part2);
    }

    #[test]
    fn run_part2() {
        run_real!(part2);
    }
}
```

After all the above is set up, just run the tests!

Before you commit anything, don't forget to `.gitignore` the `/input` folder.
