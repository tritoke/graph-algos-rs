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

use graph_algos::{Edge, EdgeWeight, Graph, graph, NodeBounds, Path, PredMap};
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry;
use std::cmp::Ordering;

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

    // get references into the graph for the start and end node.
    let start = &"a";
    let end = &"e";

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
    let mut predecessors: PredMap<'a, N>             = [(s, Edge::new(s))].iter().cloned().collect();
    let mut distances: HashMap<&'a N, EdgeWeight>    = [(s, 0.into())].iter().cloned().collect();
    let mut node_queue: BinaryHeap<QueueItem<'a, N>> = vec![QueueItem::new(s, 0)].into();

    // while we haven't explored the all the nodes
    while let Some(next) = node_queue.pop() {
        // see if we popped a valid node
        // if they match we have a valid node, otherwise we popped an old one,
        // so just continue through the loop
        if distances[next.node] == next.weight {
            for edge in graph.succs(next.node).unwrap() {
                let edge_weight = edge.weight().expect("Dijkstra requires edges to have weights - panicking.");
                let new_distance = next.weight + edge_weight;

                let closer;
                // update the distance map
                match distances.entry(edge.destination()) {
                    Entry::Vacant(v) => {
                        v.insert(new_distance);
                        closer = true;
                    },
                    Entry::Occupied(mut o) => {
                        if *o.get() > new_distance {
                            *o.get_mut() = new_distance;
                            closer = true;
                        } else {
                            closer = false;
                        }
                    },
                }

                // if we are now closer to this node then we should update the queue
                if closer {
                    // push the new item, don't worry about sift up etc
                    node_queue.push(QueueItem::new(edge.destination(), new_distance));
                    predecessors.insert(edge.destination(), Edge::new_with_weight(next.node, edge_weight));
                }
            }
        }
    }

    (predecessors, distances)
}

#[derive(Debug, Clone)]
struct QueueItem<'a, N: NodeBounds> {
    node: &'a N,
    weight: EdgeWeight,
}

impl<'a, N: NodeBounds> QueueItem<'a, N> {
    fn new(node: &'a N, weight: impl Into<EdgeWeight>) -> Self {
        Self {
            node,
            weight: weight.into(),
        }
    }
}

impl<N: NodeBounds> PartialEq for QueueItem<'_, N> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<N: NodeBounds> Eq for QueueItem<'_, N> {}

impl<N: NodeBounds> PartialOrd for QueueItem<'_, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: NodeBounds> Ord for QueueItem<'_, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering to make heap into min-heap instead of max-heap
        self.weight.cmp(&other.weight).reverse()
    }
}
