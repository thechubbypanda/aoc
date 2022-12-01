use std::fs::File;
use std::io::Write;
use std::path::Path;

use reqwest::header::COOKIE;

use crate::input_path::{get_day_path, INPUT_DIR};
use crate::input_type::InputType;

fn read_session_cookie() -> String {
    let filename = home::home_dir()
        .expect("Failed to find home directory")
        .join(".aoc.cookie");
    println!("Reading session cookie from file: {:?}", filename);
    std::fs::read_to_string(filename)
        .expect("Failed to read from session file")
        .trim()
        .to_string()
}

pub(crate) struct AocClient {
    year: u16,
    session_token: String,
    client: reqwest::blocking::Client,
}

impl AocClient {
    pub fn new(year: u16) -> Self {
        let _ = std::fs::create_dir(Path::new(INPUT_DIR));
        Self {
            year,
            session_token: read_session_cookie(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_input(&self, day: u8) -> String {
        let input = std::fs::read_to_string(get_day_path(self.year, day, InputType::Real))
            .unwrap_or(self.download_input(day));
        self.cache_input(day, &input);
        input
    }

    fn cache_input(&self, day: u8, input: &str) {
        File::create(get_day_path(self.year, day, InputType::Real))
            .expect("Failed to create input file")
            .write_all(input.as_bytes())
            .expect("Failed to write input file");
    }

    fn download_input(&self, day: u8) -> String {
        self.client
            .get(format!(
                "https://adventofcode.com/{}/day/{}/input",
                self.year, day
            ))
            .header(COOKIE, format!("session={}", self.session_token))
            .send()
            .expect("Failed to sent input request to server")
            .error_for_status()
            .expect("Got error from server")
            .text()
            .expect("Failed to parse response from server")
    }
}
