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

use graph_algos::{Edge, EdgeWeight, Graph, graph, NodeBounds, Path, PredMap};

type DistMap<'a, N> = HashMap<&'a N, EdgeWeight>;

fn main() {
    let graph: Graph<&str> = graph! {
        "a" => ["c" => 2, "b" => 3],
        "b" => ["e" => 6, "d" => 5],
        "c" => ["g" => 2, "f" => 1],
        "d" => ["i" => 2, "h" => 3],
        "e" => ["h" => 7],
        "f" => ["e" => 6],
        "i" => ["b" => 4],
    };

    println!("{}", graph);

    let start = &"a";
    let end = &"e";

    let (pred_map, dist_map) = bellman_ford(&graph, start);

    println!(
        "Node {:?} is distance {} from Node {:?}",
        start, dist_map[end], end
    );

    let path: Path<&str> = Path::new_path_to(&pred_map, end).unwrap();
    println!("{}", path);
}

fn bellman_ford<'a, N: NodeBounds>(
    graph: &'a Graph<N>,
    s: &'a N,
) -> (PredMap<'a, N>, HashMap<&'a N, EdgeWeight>) {
    let mut pred_map: PredMap<N> = Default::default();
    let mut dist_map: DistMap<N> = graph
        .nodes()
        .map(|node| (node, EdgeWeight::infinity()))
        .collect();

    // insert pred map self link
    pred_map.insert(s, Edge::new_with_weight(s, 0));

    // assert origin is distance 0 from itself
    dist_map.insert(s, 0.into());

    // loop until a round changes nothing
    let nodes = graph.len();
    for _ in 0..nodes - 1 {
        let mut changed = false;
        // perform relaxation
        for (u, edge) in graph.edges() {
            // bellman ford requires weights so a graph without weights is malformed
            let w = edge.weight().expect("No weight for this edge, panicking");
            let v = edge.destination();

            if dist_map[u] + w < dist_map[v] {
                dist_map.insert(v, dist_map[u] + w);
                pred_map.insert(v, Edge::new_with_weight(v, w));
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
