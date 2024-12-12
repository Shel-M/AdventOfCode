use log::debug;

use crate::{
    util::graph::{Direction, Graph, Node},
    Day,
};

pub struct Day6;

impl Day for Day6 {
    const DAY_NUMBER: u8 = 6;

    fn part1(input: Vec<String>) -> String {
        let mut graph = Graph::new(input);
        let _ = traverse(&mut graph);

        format!("{}", graph.count('X'))
    }

    fn part2(input: Vec<String>) -> String {
        let mut graph = Graph::new(input.clone());
        let mut res = 0;
        let mut traversed = traverse(&mut graph).unwrap();
        traversed.sort();
        traversed.dedup();

        debug!("{:?}", traversed);
        for node in traversed {
            let mut mod_graph = Graph::new(input.clone());
            let mut node = node;
            node.data = '#';
            mod_graph.save_node(node).unwrap();
            match traverse(&mut mod_graph) {
                None => res += 1,
                _ => continue,
            }
        }

        format!("{res}")
    }
}

fn traverse(graph: &mut Graph) -> Option<Vec<Node>> {
    //debug!("Traversing \n{graph}");
    let starting_position = graph.find_char('^');
    let mut direction = Direction::N;
    let mut traversed = vec![];

    let mut current = starting_position;
    while current.is_some() {
        let mut unwrapped = current.unwrap();
        traversed.push((unwrapped.location, direction));
        unwrapped.data = 'X';
        graph.save_node(unwrapped);

        let mut next = graph.get_next(&unwrapped, &direction);
        if next.is_some() && next.clone().unwrap().data == '#' {
            direction = direction.turn_right();
            next = graph.get_next(&unwrapped, &direction);

            if next.is_some() && traversed.contains(&(next.unwrap().location, direction)) {
                debug!(
                    "Infinite loop detected {:?}, {direction:?}",
                    next.unwrap().location
                );

                debug!("In graph\n{graph}");
                return None;
            }
        }

        current = next;
    }

    Some(
        traversed
            .iter()
            .map(|t| graph.get_node(t.0).unwrap())
            .collect(),
    )
}
