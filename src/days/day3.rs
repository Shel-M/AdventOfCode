use core::str;

use log::{debug, error};
use regex::Regex;

use crate::Day;

pub struct Day3;

impl Day3 {
    fn parse_input(input: Vec<String>) -> String {
        input.join("")
    }
}

impl Day for Day3 {
    const DAY_NUMBER: u8 = 3;

    fn part1(input: Vec<String>) -> String {
        let parsed = Self::parse_input(input);

        let s = Regex::new(r#"mul\((\d*),(\d*)\)"#).unwrap();
        let result: i32 = s
            .captures_iter(&parsed)
            .map(|cap| {
                let (_matched, arr): (&str, [&str; 2]) = cap.extract();
                let left: i32 = arr[0].parse::<i32>().unwrap();
                let right: i32 = arr[1].parse::<i32>().unwrap();

                return left * right;
            })
            .sum();
        format!("{result}")
    }

    fn part2(input: Vec<String>) -> String {
        let parsed = Self::parse_input(input);

        let mut enabled = true;
        let s = Regex::new(r#"(do\(\)()()|don't\(\)()()|mul\((\d*),(\d*)\))"#).unwrap();
        let mut result: i32 = 0;
        for cap in s.captures_iter(&parsed) {
            let (matched, arr): (&str, [&str; 3]) = cap.extract();
            debug!("{matched} : {}", arr[0]);

            match &matched[0..3] {
                "do(" => {
                    error!("Found do()");
                    enabled = true
                }
                "don" => {
                    error!("Found don't()");
                    enabled = false
                }
                "mul" => {
                    if enabled {
                        let left: i32 = arr[1].parse::<i32>().unwrap();
                        let right: i32 = arr[2].parse::<i32>().unwrap();

                        error!(
                            "Adding {left} * {right} = {} + {result} = {}",
                            left * right,
                            left * right + result
                        );
                        result += left * right;
                    } else {
                        error!("Not enabled, not adding {} * {}", arr[1], arr[2]);
                    }
                }
                _ => {
                    error!("Shouldn't be possible.");
                    panic!("Match group returned nonsense.")
                }
            }
        }
        format!("{result}")
    }
}
