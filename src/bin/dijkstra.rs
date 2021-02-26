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

use graph_algos::{EdgeWeight, Graph, NodeBounds, Path, PredMap};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut graph: Graph<String> = include_str!("../inputs/graph_3.in").parse().unwrap();

    // get references into the graph for the start and end node.
    let start = graph.nodes().find(|&node| node == "a").unwrap();
    let end = graph.nodes().find(|&node| node == "e").unwrap();

    let (pred_map, dist_map) = dijkstra(&graph, start);
    let path = Path::new_path_to(&pred_map, end).unwrap();

    println!(
        "Node {:?} is distance {} from Node {:?}",
        start, dist_map[end], end
    );

    println!("Path taken: {}", path);
}

fn dijkstra<'a, N: NodeBounds>(
    graph: &'a Graph<N>,
    s: &'a N,
) -> (PredMap<'a, N>, HashMap<&'a N, EdgeWeight>) {
    unimplemented!()
}
