use std::fmt::Display;

use log::debug;

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

struct Graph {
    width: usize,
    height: usize,
    nodes: Vec<Vec<Node>>,
}

impl Graph {
    fn new(data: Vec<String>) -> Self {
        let mut graph = Self {
            width: 0,
            height: 0,
            nodes: Vec::new(),
        };

        for (line_i, line) in data.iter().enumerate() {
            graph.height += 1;
            let mut graph_line = Vec::new();

            for (char_i, char) in line.chars().enumerate() {
                graph.width = char_i;
                let node = Node::new((char_i, line_i), char);
                graph_line.push(node);
            }
            graph.nodes.push(graph_line);
        }

        graph
    }

    fn get_next(&self, node: &Node, direction: &Direction) -> Option<&Node> {
        let d = direction.tup();
        let (x, y) = (
            match node.location.0.checked_add_signed(d.0) {
                Some(v) => v,
                None => return None,
            },
            match node.location.1.checked_add_signed(d.1) {
                Some(v) => v,
                None => return None,
            },
        );

        Some(self.nodes.get(y)?.get(x)?)
    }
}

#[derive(Debug)]
struct Node {
    location: (usize, usize),
    data: char,
}

impl Node {
    fn new(location: (usize, usize), data: char) -> Self {
        Self { location, data }
    }
}

#[allow(dead_code)]
enum Direction {
    NW = 0,
    N = 1,
    NE = 2,
    E = 3,
    SE = 4,
    S = 5,
    SW = 6,
    W = 7,
}

impl Direction {
    fn tup(&self) -> (isize, isize) {
        match self {
            Self::NW => (-1, -1),
            Self::N => (0, -1),
            Self::NE => (1, -1),
            Self::W => (-1, 0),
            // Center =>  (0, 0),
            Self::E => (1, 0),
            Self::SW => (-1, 1),
            Self::S => (0, 1),
            Self::SE => (1, 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NW => "NW",
                Self::N => "N",
                Self::NE => "NE",
                Self::W => "W",
                // Self::Center =>"Center",
                Self::E => "E",
                Self::SW => "SW",
                Self::S => "S",
                Self::SE => "SE",
            }
        )
    }
}
