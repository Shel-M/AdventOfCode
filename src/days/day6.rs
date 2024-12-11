use crate::{
    util::graph::{Direction, Graph},
    Day,
};

pub struct Day6;

impl Day for Day6 {
    const DAY_NUMBER: u8 = 6;

    fn part1(input: Vec<String>) -> String {
        let mut graph = Graph::new(input);
        let starting_position = graph.find('^');
        let mut direction = Direction::N;

        let mut next = starting_position;

        format!("")
    }

    fn part2(input: Vec<String>) -> String {
        todo!()
    }
}
