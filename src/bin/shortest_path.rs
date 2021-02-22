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

    let pred_map = shortest_paths(&graph, 1);
    let path = get_path(&pred_map, 8);

    println!("{:?}", pred_map);
    println!("{:?}", path);
}

/// returns the predecessor map, from the graph and a start node
fn shortest_paths<N: NodeTraits>(graph: &Graph<N>, s: N) -> HashMap<N, N> {
    let mut discovered: VecDeque<&N> = vec![&s].into();
    let mut finished: HashSet<&N> = HashSet::new();

    let mut pred_map: HashMap<N, N> = HashMap::new();
    pred_map.insert(s.clone(), s.clone());

    while let Some(u) = discovered.pop_front() {
        finished.insert(u);

        if let Some(succs) = graph.succs(&u) {
            for v in succs.iter().filter(|v| !finished.contains(v)) {
                pred_map.insert(v.clone(), u.clone());

                discovered.push_back(v);
            }
        }
    }

    pred_map
}

/// extracts a path from the predecessor map and an end node
fn get_path<N: NodeTraits>(pred_map: &HashMap<N, N>, end_node: N) -> Vec<N> {
    let mut rev_path = vec![end_node.clone()];
    let mut next_node = end_node;

    println!("{:?}", pred_map.get(&next_node));

    while let Some(u) = pred_map.get(&next_node) {
        if u.clone() != next_node {
            next_node = u.clone();
            rev_path.push(u.clone());
        } else {
            break;
        }
    }

    rev_path.reverse();

    rev_path
}
