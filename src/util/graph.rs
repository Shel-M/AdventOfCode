use std::{cmp::Ordering, fmt::Display};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone)]
pub struct Graph {
    pub width: usize,
    pub height: usize,
    pub nodes: Vec<Vec<Node>>,
}

impl Graph {
    pub fn new(data: Vec<String>) -> Self {
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
            if !graph_line.is_empty() {
                graph.nodes.push(graph_line)
            };
        }

        graph
    }

    pub fn get_next(&self, node: &Node, direction: &Direction) -> Option<Node> {
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

        Some(self.nodes.get(y)?.get(x)?.clone())
    }

    pub fn get_node(&self, location: (usize, usize)) -> Option<Node> {
        Some(*self.nodes.get(location.1)?.get(location.0)?)
    }

    pub fn save_node(&mut self, node: Node) -> Option<()> {
        let (x, y) = node.location;
        if let Some(n) = self.nodes.get_mut(y)?.get_mut(x) {
            n.location = node.location;
            n.data = node.data;
        }
        Some(())
    }

    pub fn find_char(&self, c: char) -> Option<Node> {
        for row in &self.nodes {
            match row.iter().find(|n| n.data == c) {
                Some(n) => {
                    return Some(n.clone());
                }
                None => {}
            };
        }
        None
    }

    pub fn count(&self, c: char) -> usize {
        let mut res = 0;
        for row in &self.nodes {
            res += row.iter().filter(|v| v.data == c).count()
        }
        res
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.nodes {
            write!(f, "[")?;
            for (i, node) in line.iter().enumerate() {
                // write!(
                //     f,
                //     "\"{}\"{}",
                //     node.data,
                //     if i < line.len() { ", " } else { "" }
                // )?;

                write!(f, "{}", node.data,)?;
            }

            write!(f, "]\n")?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Node {
    pub location: (usize, usize),
    pub data: char,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.location.0 < other.location.0 {
            Ordering::Less
        } else if self.location.0 > other.location.0 {
            Ordering::Greater
        } else {
            if self.location.1 < other.location.1 {
                Ordering::Less
            } else if self.location.1 < other.location.1 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl Node {
    pub fn new(location: (usize, usize), data: char) -> Self {
        Self { location, data }
    }
}

#[allow(dead_code)]
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
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

    pub fn turn_right(self) -> Self {
        Direction::from_u8((self as u8 + 2) % 8).unwrap()
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

// impl<T: CheckedRem> From<T> for Direction {
//     fn from(value: T) -> Self {
//         match value.rem(8) {
//             0 => Self::NW,
//             1 => Self::N,
//             2 => Self::NE,
//             3 => Self::E,
//             4 => Self::SE,
//             5 => Self::S,
//             6 => Self::SW,
//             7 => Self::W,
//             _ => panic!("Direction % 8 returned value > 7"),
//         }
//     }
// }
