use std::{
    fs::{self, read_to_string, File},
    io::{Read, Write},
    path::Path,
};

use clap::Parser;
use log::Level;
use log::{debug, error};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Error, StatusCode,
};

mod days;
use days::*;

fn main() {
    let cli = CLI::parse();

    simple_logger::init_with_level(cli.log).unwrap();

    println!(
        "{}",
        match cli.day {
            1 => Day1::solution(cli.part),
            2 => Day2::solution(cli.part),
            3 => Day3::solution(cli.part),
            _ => "Not complete, or not implemented.".to_string(),
        }
    );
}

#[derive(Parser, Debug)]
struct CLI {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    #[arg(short, long, default_value_t=Level::Error)]
    log: Level,
}

pub trait Day {
    const DAY_NUMBER: u8;

    fn get_input() -> Result<Vec<String>, Error> {
        let path_string = format!("./input/{}", Self::DAY_NUMBER);
        let path = Path::new(&path_string);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                if fs::read_dir("./input").is_err() {
                    fs::create_dir("./input").unwrap()
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::COOKIE,
                    HeaderValue::from_str(&format!(
                        "session={}",
                        read_to_string("./session_token")
                            .unwrap()
                            .lines()
                            .last()
                            .unwrap()
                    ))
                    .unwrap(),
                );
                let client = reqwest::blocking::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .unwrap();

                let mut request = client
                    .get(format!(
                        "https://adventofcode.com/2024/day/{}/input",
                        Self::DAY_NUMBER
                    ))
                    .send()
                    .unwrap();

                if request.status() != StatusCode::OK {
                    error!("Unable to get input {}", request.status().as_str());
                    return Err(request.error_for_status().expect_err("What???"));
                }

                let mut response = String::new();
                request.read_to_string(&mut response).unwrap();

                let mut f = File::create(path).unwrap();
                f.write(response.as_bytes()).unwrap();
                f.flush().unwrap();

                File::open(path).unwrap()
            }
        };

        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        Ok(input
            .split('\n')
            .map(|s| s.trim().to_string())
            .filter(|s| s.len() > 0)
            .collect::<Vec<String>>())
    }

    fn solution(part: u8) -> String {
        let input = Self::get_input().unwrap();
        debug!("{input:#?}");

        debug!("Calling part {part}...");
        if part == 1 {
            return Self::part1(input);
        }
        if part == 2 {
            return Self::part2(input);
        }
        panic!("Invalid part '{part}', shouldn't be possible.")
    }

    fn part1(input: Vec<String>) -> String;
    fn part2(input: Vec<String>) -> String;
}
