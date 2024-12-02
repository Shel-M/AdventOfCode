use std::{
    fs::{read_to_string, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Request,
};

mod day1;

fn main() {
    day1::solution1();
    day1::solution2();
}

pub trait Day {
    const DAY: u8;

    fn get_input() -> Vec<String> {
        let path_string = format!("./input/{}", Self::DAY);
        let path = Path::new(&path_string);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
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
                        Self::DAY
                    ))
                    .send()
                    .unwrap();

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
        input
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    fn part1(input: Vec<String>) -> String;
    fn part2(input: Vec<String>) -> String;
}
