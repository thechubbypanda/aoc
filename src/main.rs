extern crate core;

pub mod year2021;
pub mod year2022;

fn main() {
    // Generates binary for benchmarking
    println!(
        "{}",
        year2022::day14::part2(include_str!("../input/year2022_day14.real").to_string())
    );
}
