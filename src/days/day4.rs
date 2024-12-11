use log::debug;

use crate::util::graph::*;
use crate::Day;

pub struct Day4;

impl Day for Day4 {
    const DAY_NUMBER: u8 = 4;

    fn part1(input: Vec<String>) -> String {
        let graph = Graph::new(input);
        let mut result = 0;

        for line in &graph.nodes {
            debug!("{}", line.iter().map(|n| n.data).collect::<String>());

            for node in line {
                debug!("");
                if Self::part1_recursive_check(&graph, node, &Direction::E, 0, None) {
                    debug!("found");
                    result += 1;
                }
                if Self::part1_recursive_check(&graph, node, &Direction::S, 0, None) {
                    debug!("found");
                    result += 1;
                }
                if Self::part1_recursive_check(&graph, node, &Direction::SE, 0, None) {
                    debug!("found");
                    result += 1;
                }
                if Self::part1_recursive_check(&graph, node, &Direction::SW, 0, None) {
                    debug!("found");
                    result += 1;
                }
            }
        }

        format!("{result}")
    }

    fn part2(input: Vec<String>) -> String {
        let graph = Graph::new(input);
        let mut result = 0;

        for line in &graph.nodes[1..graph.nodes.len() - 1] {
            debug!("{}", line.iter().map(|n| n.data).collect::<String>());

            for node in &line[1..line.len() - 1] {
                debug!("current result: {result} node: {node:?}");

                if node.data == 'A' {
                    let mut a = vec![
                        graph.get_next(node, &Direction::NW).unwrap_or(node),
                        graph.get_next(node, &Direction::SE).unwrap_or(node),
                    ]
                    .iter()
                    .map(|n| n.data)
                    .collect::<Vec<char>>();
                    a.sort();
                    debug!("{a:?}");

                    if !(a.contains(&'M') && a.contains(&'S')) {
                        continue;
                    }

                    let mut b = vec![
                        graph.get_next(node, &Direction::SW).unwrap_or(node),
                        graph.get_next(node, &Direction::NE).unwrap_or(node),
                    ]
                    .iter()
                    .map(|n| n.data)
                    .collect::<Vec<char>>();
                    b.sort();
                    debug!("{b:?}");

                    if b.contains(&'M') && b.contains(&'S') {
                        result += 1;
                    }
                }
            }
        }

        format!("{result}")
    }
}

impl Day4 {
    const F: &str = "XMAS";
    const B: &str = "SAMX";
    fn part1_recursive_check(
        graph: &Graph,
        node: &Node,
        direction: &Direction,
        char_pos: usize,
        check_forward: Option<bool>,
    ) -> bool {
        let string = match check_forward {
            None => {
                return Self::part1_recursive_check(graph, node, &direction, char_pos, Some(true))
                    || Self::part1_recursive_check(graph, node, &direction, char_pos, Some(false));
            }
            Some(true) => Self::F,
            Some(false) => Self::B,
        };

        if char_pos >= string.len() {
            return true;
        }

        debug!(
            "loc: {:?} dir: {direction} data: {} char: {} pos: {} string: {string}",
            node.location,
            node.data,
            string.chars().nth(char_pos).unwrap(),
            char_pos
        );

        if node.data == string.chars().nth(char_pos).unwrap() {
            if char_pos == string.len() - 1 {
                return true;
            }

            let next_node = match graph.get_next(node, direction) {
                Some(n) => n,
                None => return false,
            };

            return Self::part1_recursive_check(
                graph,
                next_node,
                direction,
                char_pos + 1,
                check_forward,
            );
        }

        false
    }
}
