/*
 *  Copyright (C) 2021  Sam Leonard
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::{HashMap, HashSet, VecDeque};
use graph_algos::{Graph, NodeTraits};

fn main() {
    let mut graph: Graph<u32> = Graph::empty();
    graph.fill_from_str(include_str!("../inputs/graph_2.in"));

    println!("{:?}", graph);

    let source = graph.node(1).unwrap();
    let dest = graph.node(8).unwrap();

    let pred_map = shortest_paths(&graph, source);
    let path = get_path(&pred_map, dest);

    println!("{:?}", pred_map);
    println!("{:?}", path);
}

/// returns the predecessor map, from the graph and a start node
fn shortest_paths<'a, N: NodeTraits>(graph: &'a Graph<N>, s: &'a N) -> HashMap<&'a N, &'a N> {
    let mut discovered: VecDeque<&N> = vec![s].into();
    let mut finished: HashSet<&N> = HashSet::new();

    let mut pred_map: HashMap<&N, &N> = HashMap::new();
    pred_map.insert(s, s);

    while let Some(u) = discovered.pop_front() {
        finished.insert(u);

        if let Some(succs) = graph.succs(&u) {
            for v in succs.iter().filter(|v| !finished.contains(v)) {
                pred_map.insert(v, u);

                discovered.push_back(v);
            }
        }
    }

    pred_map
}

/// extracts a path from the predecessor map and an end node
fn get_path<'a, N: NodeTraits>(pred_map: &HashMap<&'a N, &'a N>, end_node: &'a N) -> Vec<&'a N> {
    let mut rev_path = vec![end_node];
    let mut next_node = end_node;

    println!("{:?}", pred_map.get(next_node));

    while let Some(u) = pred_map.get(next_node) {
        if u != &next_node {
            next_node = u;
            rev_path.push(u);
        } else {
            break;
        }
    }

    rev_path.reverse();

    rev_path
}
