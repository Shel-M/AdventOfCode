use log::debug;

pub struct Day1;

impl Day1 {
    fn parse_input(input: Vec<String>) -> (Vec<i64>, Vec<i64>) {
        let mut left: Vec<i64> = vec![];
        let mut right: Vec<i64> = vec![];

        for line in input {
            if line.is_empty() {
                continue;
            }
            debug!("parsing {line}...");
            let pair = line.split(" ");
            let pair = pair
                .map(|s| s.trim())
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            left.push(pair[0]);
            right.push(pair[1]);
        }
        return (left, right);
    }
}

impl crate::Day for Day1 {
    const DAY_NUMBER: u8 = 1;

    fn part1(input: Vec<String>) -> String {
        debug!("Parsing input...");
        let (mut left, mut right) = Self::parse_input(input);
        debug!("{left:#?}, {right:#?}");
        left.sort();
        right.sort();

        let mut vals = vec![];
        for i in 0..left.len() {
            vals.push((left[i] - right[i]).abs());
        }

        return format!("{}", vals.iter().sum::<i64>());
    }

    fn part2(input: Vec<String>) -> String {
        let (left, right) = Self::parse_input(input);

        let mut similarity = 0;
        for val in left {
            let score_mod = right.iter().filter(|i| **i == val).count() as i64;
            similarity += val * score_mod;
        }

        format!("{}", similarity)
    }
}
