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
use graph_algos::{Graph, NodeTraits};

fn main() {
    let mut graph: Graph<u32> = Graph::empty();

    graph.fill_from_str(include_str!("../inputs/graph_1.in"));

    // sort from node 1
    let sorted = topological_sort(&graph, 1);

    println!("{:?}", graph);

    println!("{:?}", sorted);
}

fn topological_sort<N: NodeTraits>(graph: &Graph<N>, u: N) -> Vec<N> {
    let mut processed: HashMap<N, bool> = HashMap::new();

    let mut rev_order: Vec<N> = Vec::new();

    topo_rec(graph, u, &mut processed, &mut rev_order);

    rev_order.reverse();

    rev_order
}

fn topo_rec<N: NodeTraits>(
    graph: &Graph<N>,
    u: N,
    mut processed: &mut HashMap<N, bool>,
    mut rev_order: &mut Vec<N>,
) {
    let processed_node = *processed
        .entry(u.clone())
        .and_modify(|p| {
            if !*p {
                panic!("Loop detected.")
            }
        })
        .or_insert(false);

    if !processed_node {
        if let Some(succs) = graph.succs(&u) {
            for v in succs {
                topo_rec(graph, v.clone(), &mut processed, &mut rev_order);
            }
        }

        // finished processing
        processed.insert(u.clone(), true);

        rev_order.push(u);
    }
}
