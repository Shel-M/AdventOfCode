use std::u32;

use log::debug;

pub struct Day2;

impl Day2 {
    fn parse_input(input: Vec<String>) -> Vec<Vec<i32>> {
        let mut out = Vec::new();
        for line in input {
            let d = line
                .split(" ")
                .map(|s| s.parse::<i32>().expect("Could not parse to integer"))
                .collect::<Vec<i32>>();
            out.push(d);
        }

        out
    }
}

#[derive(Debug)]
enum Direction {
    New,
    Increasing,
    Decreasing,
}

impl crate::Day for Day2 {
    const DAY_NUMBER: u8 = 2;

    fn part1(input: Vec<String>) -> String {
        let data = Self::parse_input(input);
        debug!("data: {data:?}");

        let mut out = 0;
        for report in data {
            let mut last = i32::MAX;
            let mut direction = Direction::New;
            let mut safe = true;

            let report_clone = report.clone();
            for level in report {
                if last == i32::MAX {
                    last = level;
                    continue;
                }
                debug!("{last} {level}");
                let r = match direction {
                    Direction::Increasing => last..=last + 3,
                    Direction::Decreasing => last - 3..=last,
                    Direction::New => last - 3..=last + 3,
                };

                if last == level || !r.contains(&level) {
                    safe = false;
                    debug!("{report_clone:?} {last} {level} {direction:?}");
                    break;
                }

                if last < level {
                    direction = Direction::Increasing
                } else {
                    direction = Direction::Decreasing
                }

                last = level
            }

            if safe {
                out += 1;
            }
        }

        format!("{out}")
    }

    fn part2(input: Vec<String>) -> String {
        let data = Self::parse_input(input);

        fn check_safe(report: &Vec<i32>) -> bool {
            let mut last = i32::MAX;
            let mut direction = Direction::New;
            let mut safe = true;
            let report_clone = report.clone();

            for level in report {
                let level = *level;
                if last == i32::MAX {
                    last = level;
                    continue;
                }
                let r = match direction {
                    Direction::Increasing => last..=last + 3,
                    Direction::Decreasing => last - 3..=last,
                    Direction::New => last - 3..=last + 3,
                };

                if last == level || !r.contains(&level) {
                    safe = false;
                    debug!("{report_clone:?} {last} {level} {direction:?}");
                    break;
                }

                match direction {
                    Direction::New => {
                        if last < level {
                            direction = Direction::Increasing
                        } else {
                            direction = Direction::Decreasing
                        }
                    }
                    _ => (),
                }

                last = level
            }

            return safe;
        }

        let mut out = 0;
        for report in data {
            if check_safe(&report) {
                out += 1;
            } else {
                debug!("testing for single removal in {report:?}...");
                for i in 0..report.len() {
                    let mut copy = report.clone();

                    copy.remove(i);

                    debug!("testing {report:?} as {copy:?}");
                    if check_safe(&copy) {
                        out += 1;
                        break;
                    }
                }
            }
        }

        format!("{out}")
    }
}
