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

use std::collections::HashMap;
use graph_algos::{NodeTraits, WeightedGraph};

fn main() {
    let mut graph: WeightedGraph<String> = WeightedGraph::empty();

    graph.fill_from_str(include_str!("../inputs/graph_3.in"));

    let start = graph.nodes().find(|node| *node == "a").unwrap();
    let end = graph.nodes().find(|node| *node == "e").unwrap();

    let (p, d) = bellman_ford(&graph, start);
    let path = get_path(&p, end);

    println!(
        "Node {:?} is distance {} from Node {:?}",
        start, d[end], end
    );
    let mut pathstr = format!("{:?}", start);

    // iterate over pairs in the path
    for (u, v) in path.iter().zip(path.iter().skip(1)) {
        pathstr.push_str(&format!(" --({})-> {:?}", graph.weight((*u, *v)), *v));
    }

    println!("Path taken: {}", pathstr);
}

fn bellman_ford<'a, N: NodeTraits>(
    graph: &'a WeightedGraph<N>,
    s: &'a N,
) -> (HashMap<&'a N, &'a N>, HashMap<&'a N, f64>) {
    let mut pred_map: HashMap<&'a N, &'a N> = HashMap::new();
    let mut dist_map: HashMap<&'a N, f64> =
        graph.nodes().map(|node| (node, f64::INFINITY)).collect();

    dist_map.insert(s, 0.0);

    // loop until a round changes nothing
    let nodes = graph.len();
    for _ in 0..nodes - 1 {
        let mut changed = false;
        // perform relaxation
        for ((u, v), w) in graph.weights() {
            if dist_map[u] + w < dist_map[v] {
                dist_map.insert(v, dist_map[u] + w);
                pred_map.insert(v, u);
                changed = true;
            }
        }

        // break out if nothing happened this round
        if !changed {
            break;
        };
    }

    (pred_map, dist_map)
}

/// extracts a path from the predecessor map and an end node
fn get_path<'a, N: NodeTraits>(pred_map: &HashMap<&'a N, &'a N>, end_node: &'a N) -> Vec<&'a N> {
    let mut rev_path = vec![end_node];
    let mut next_node: &N = end_node;

    while let Some(u) = pred_map.get(next_node) {
        if *u != next_node {
            next_node = u;
            rev_path.push(u);
        } else {
            break;
        }
    }

    rev_path.reverse();

    rev_path
}
