use std::fmt::Display;
use std::fs::File;
use std::time::Instant;

use scanf::sscanf;

use client::AocClient;

use crate::input_path::get_day_path;
use crate::input_type::InputType;

pub mod input_type;
pub mod util;

mod client;
mod input_path;

#[macro_export]
macro_rules! run_test {
    ($e:expr) => {
        use aoc_lib::input_type::InputType;
        use aoc_lib::run;
        run(file!(), $e, InputType::Test);
    };
}

#[macro_export]
macro_rules! run_real {
    ($e:expr) => {
        use aoc_lib::input_type::InputType;
        use aoc_lib::run;
        run(file!(), $e, InputType::Real);
    };
}

pub fn run<F, R>(src: &str, f: F, input_type: InputType)
where
    F: Fn(String) -> R,
    R: Display,
{
    let mut year: u16 = 0;
    let mut day: u8 = 0;
    sscanf!(src, "src/year{}/day{}.rs", year, day).unwrap();
    let input = match input_type {
        InputType::Real => AocClient::new(year).get_input(day),
        InputType::Test => {
            let file = &get_day_path(year, day, input_type)
                .to_str()
                .unwrap()
                .to_string();
            std::fs::read_to_string(file).unwrap_or_else(|e| {
                println!("Failed to find {file}");
                match File::create(file) {
                    Ok(_) => {
                        println!("Created empty file: {file}");
                        println!("Place your test data in there and re-run");
                    }
                    Err(_) => println!("Error: {e}"),
                }
                String::new()
            })
        }
    };
    if input.is_empty() {
        return;
    }
    let timer = Instant::now();
    let r = f(input);
    let elapsed = timer.elapsed();
    println!("Time: {:?}", elapsed);
    println!("Output: {}", r);
}
