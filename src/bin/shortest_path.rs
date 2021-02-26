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

use graph_algos::{Graph, NodeBounds, Path, PredMap};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let graph: Graph<u32> = include_str!("../inputs/graph_2.in").parse().unwrap();

    println!("{:?}", graph);

    let source = graph.nodes().find(|&edge| edge == &1).unwrap();
    let dest = graph.nodes().find(|&edge| edge == &8).unwrap();

    let pred_map = shortest_paths(&graph, source);
    let path = Path::new_path_to(&pred_map, dest).unwrap();

    println!("{:?}", pred_map);
    println!("{}", path);
}

/// returns the predecessor map, from the graph and a start node
fn shortest_paths<'a, N: NodeBounds>(graph: &'a Graph<N>, s: &'a N) -> PredMap<'a, N> {
    let mut discovered: VecDeque<&N> = vec![s].into();
    let mut finished: HashSet<&N> = HashSet::new();

    let mut pred_map: PredMap<N> = HashMap::new();
    pred_map.insert(s, (s, None));

    while let Some(u) = discovered.pop_front() {
        finished.insert(u);

        if let Some(succs) = graph.succs(&u) {
            for edge in succs
                .iter()
                .filter(|edge| !finished.contains(edge.destination()))
            {
                pred_map.insert(edge.destination(), (u, None));

                discovered.push_back(edge.destination());
            }
        }
    }

    pred_map
}
