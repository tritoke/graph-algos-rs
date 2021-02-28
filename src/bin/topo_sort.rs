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

use graph_algos::{graph, Graph, NodeBounds};
use std::collections::HashMap;

fn main() {
    // inputs/graph_1.in
    let graph: Graph<u32> = graph! {
        1 => [2, 3],
        2 => [4, 6],
        3 => [5, 6],
        5 => [6],
    };

    // sort from node 1
    let sorted = topological_sort(&graph, &1);

    println!("{}", graph);

    println!("{:?}", sorted);
}

fn topological_sort<'a, N: NodeBounds>(graph: &'a Graph<N>, u: &'a N) -> Vec<&'a N> {
    let mut processed: HashMap<&N, bool> = HashMap::new();

    let mut rev_order: Vec<&N> = Vec::new();

    topo_rec(graph, u, &mut processed, &mut rev_order);

    rev_order.reverse();

    rev_order
}

fn topo_rec<'a, N: NodeBounds>(
    graph: &'a Graph<N>,
    u: &'a N,
    mut processed: &mut HashMap<&'a N, bool>,
    mut rev_order: &mut Vec<&'a N>,
) {
    let processed_node = *processed
        .entry(u)
        .and_modify(|p| {
            if !*p {
                panic!("Loop detected.")
            }
        })
        .or_insert(false);

    if !processed_node {
        if let Some(succs) = graph.succs(u) {
            for edge in succs {
                topo_rec(graph, edge.destination(), &mut processed, &mut rev_order);
            }
        }

        // finished processing
        processed.insert(u, true);

        rev_order.push(u);
    }
}
