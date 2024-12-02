use std::fs::read_to_string;

fn get_input() -> (Vec<i64>, Vec<i64>) {
    let input = read_to_string("input/1").unwrap();
    let input = input.lines();
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];

    for line in input {
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

pub fn solution1() {
    let (mut left, mut right) = get_input();
    left.sort();
    right.sort();

    let mut vals = vec![];
    for i in 0..left.len() {
        vals.push((left[i] - right[i]).abs());
    }

    println!("{}", vals.iter().sum::<i64>());
}

pub fn solution2() {
    let (left, right) = get_input();

    let mut similarity = 0;
    for val in left {
        let score_mod = right.iter().filter(|i| **i == val).count() as i64;
        similarity += val * score_mod;
    }

    println!("{}", similarity);
}
