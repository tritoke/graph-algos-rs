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

use std::collections::{hash_map, HashMap};

use crate::graph::{Graph, NodeTraits, Nodes};

/// Weighted graph type is just a wrapper around the standard graph
/// adding another map in which to store the weights of edges
#[derive(Debug)]
pub struct WeightedGraph<N>
where
    N: NodeTraits,
{
    /// the underlying graph
    graph: Graph<N>,
    /// A mapping from edge to weight
    weights: HashMap<(N, N), f64>,
}

impl<N: NodeTraits> WeightedGraph<N> {
    /// creates a new empty graph
    pub fn empty() -> Self {
        Self {
            graph: Graph::empty(),
            weights: HashMap::new(),
        }
    }

    /// adds and edge to the graph
    pub fn add_edge(&mut self, u: N, v: N, w: f64) {
        self.graph.add_edge(u.clone(), v.clone());

        self.weights.insert((u, v), w);
    }

    /// removes and edge from the graph
    pub fn remove_edge(&mut self, u: &N, v: &N) {
        self.graph.remove_edge(u, v);

        self.weights.remove(&(u.clone(), v.clone()));
    }

    /// returns whether an edge exists in the graph
    #[inline]
    pub fn is_edge(&self, u: &N, v: &N) -> bool {
        self.graph.is_edge(u, v)
    }

    /// Returns the successors of a node in the graph
    #[inline]
    pub fn succs(&self, u: &N) -> Option<&[N]> {
        self.graph.succs(u)
    }

    /// Gets the weight of an edge
    /// if it doesn't exist then it is said to have an infinite weight
    pub fn weight(&self, edge: (&N, &N)) -> f64 {
        *self
            .weights
            .get(&(edge.0.clone(), edge.1.clone()))
            .unwrap_or(&f64::INFINITY)
    }

    /// Returns the number of nodes in the graph
    #[inline]
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    /// returns whether the graph is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    /// Returns a reference to a node in the graph
    #[inline]
    pub fn node(&self, needle: N) -> Option<&N> {
        self.graph.node(needle)
    }

    /// Returns an iterator over the nodes in the graph
    #[inline]
    pub fn nodes(&self) -> Nodes<N> {
        self.graph.nodes()
    }

    /// Returns an iterator over the edges in the graph
    #[inline]
    pub fn edges(&self) -> Edges<N> {
        Edges {
            inner: self.weights.keys(),
        }
    }

    /// Returns an iterator over the edges in the graph
    #[inline]
    pub fn weights(&self) -> Weights<N> {
        Weights {
            inner: self.weights.iter(),
        }
    }

    /// fill an weighted directed graph from a string
    /// each line is a node followed by a space seperated list of node,weight pairs
    pub fn fill_from_str(&mut self, s: &str) {
        // the first node added to the graph
        for line in s.lines() {
            let (u, vs) = line.split_once(':').expect("Invalid graph format.");

            let u_fs: N = u.parse().expect("Failed to parse origin node.");

            let edges = vs.split(' ').map(|pair| {
                let (v, w) = pair
                    .split_once(',')
                    .expect("Failed to parse pair - no comma.");

                (
                    v.parse().expect("Failed to parse destination node."),
                    w.parse().expect("Failed to parse weight for edge."),
                )
            });

            for (v, w) in edges {
                self.add_edge(u_fs.clone(), v, w);
            }
        }
    }
}

/// Edge struct can be implemented more efficiently for weighted graph
/// due to the edge-weight map
pub struct Edges<'a, N>
where
    N: NodeTraits,
{
    inner: hash_map::Keys<'a, (N, N), f64>,
}

impl<'a, N> Iterator for Edges<'a, N>
where
    N: NodeTraits,
{
    type Item = (&'a N, &'a N);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|edge| (&edge.0, &edge.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Edge struct can be implemented more efficiently for weighted graph
/// due to the edge-weight map
pub struct Weights<'a, N>
where
    N: NodeTraits,
{
    inner: hash_map::Iter<'a, (N, N), f64>,
}

impl<'a, N> Iterator for Weights<'a, N>
where
    N: NodeTraits,
{
    type Item = <hash_map::Iter<'a, (N, N), f64> as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
