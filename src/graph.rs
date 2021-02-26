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

// Trait imports
use std::{fmt::Debug, hash::Hash, str::FromStr};

use crate::{Edge, ParseEdgeError};

/// A trait to represent all of the bounds that a node in the graph must provide
pub trait NodeBounds: Hash + FromStr<Err: Debug> + Debug + Eq + Clone {}
impl<T: Hash + FromStr<Err: Debug> + Debug + Eq + Clone> NodeBounds for T {}

/// A node-generic graph type implemented using an adjacency list
/// Where the successors of a node are stored in a hashmap.
#[derive(Debug)]
pub struct Graph<N: NodeBounds> {
    /// the graph is backed by a hashmap from a node to a vector of nodes
    backing_map: HashMap<N, Vec<Edge<N>>>,
}

impl<N: NodeBounds> Graph<N> {
    /// creates a new empty graph
    pub fn empty() -> Self {
        Default::default()
    }

    /// adds and edge to the graph
    pub fn add_edge(&mut self, u: N, e: Edge<N>) {
        self.backing_map
            .entry(e.destination().clone())
            .or_insert_with(Vec::new);

        self.backing_map.entry(u).or_insert_with(Vec::new).push(e);
    }

    /// removes and edge from the graph
    pub fn remove_edge(&mut self, u: &N, v: &N) {
        if let Some(edges) = self.backing_map.get_mut(u) {
            if let Some(pos) = edges.iter().position(|e| e.destination() == v) {
                edges.remove(pos);
            }
        }
    }

    /// Returns whether an edge exists in the graph
    pub fn is_edge(&self, u: &N, v: &N) -> bool {
        if let Some(succs) = self.backing_map.get(u) {
            succs.iter().find(|edge| edge.destination() == v).is_some()
        } else {
            false
        }
    }

    /// Returns the successors of a node in the graph
    pub fn succs(&self, u: &N) -> Option<&[Edge<N>]> {
        self.backing_map.get(u).map(|vec| vec.as_slice())
    }

    /// Returns the number of nodes in a graph
    pub fn len(&self) -> usize {
        self.backing_map.len()
    }

    /// Returns whether the graph is empty
    pub fn is_empty(&self) -> bool {
        self.backing_map.is_empty()
    }

    /// Returns an iterator over the nodes in a graph
    pub fn nodes(&self) -> Nodes<'_, N> {
        Nodes {
            inner: self.backing_map.keys(),
        }
    }

    /// Returns an iterator over the edges in the graph
    pub fn edges(&self) -> Edges<'_, N> {
        Edges {
            inner: self.backing_map.iter(),
            curr_node: None,
            curr_dest_no: 0,
        }
    }
}

impl<N: NodeBounds> Default for Graph<N> {
    fn default() -> Self {
        Self {
            backing_map: HashMap::new(),
        }
    }
}

/// represents the failure to parse a node
#[derive(Fail, Debug)]
pub enum GraphParseError {
    /// Represents the failure to parse an outbound edge
    #[fail(display = "Failed to parse outbound edge: {}", _0)]
    EdgeParseError(#[fail(cause)] ParseEdgeError),

    /// Represents the failure to parse the destination node in an edge
    #[fail(display = "Failed to parse source node: {}", _0)]
    NodeParseError(String),

    #[fail(display = "Error in graph format.")]
    FormatError,
}

impl<N: NodeBounds> FromStr for Graph<N> {
    type Err = GraphParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fill an unweighted directed graph from a string
        // each line is node:edges seperated by spaces

        // new empty graph
        let mut graph: Graph<N> = Default::default();

        for line in s.lines() {
            let (u, edges) = line.split_once(':').ok_or(GraphParseError::FormatError)?;

            let u_fs: N = u
                .parse()
                .map_err(|err| GraphParseError::NodeParseError(format!("{:?}", err)))?;

            let edges = edges
                .split(' ')
                .map(|edge| edge.parse().map_err(GraphParseError::EdgeParseError));

            for edge in edges {
                let e = edge?;
                graph.add_edge(u_fs.clone(), e);
            }
        }

        Ok(graph)
    }
}

/// An iterator over the nodes of the graph
#[derive(Debug)]
pub struct Nodes<'a, N: NodeBounds> {
    inner: hash_map::Keys<'a, N, Vec<Edge<N>>>,
}

impl<'a, N: NodeBounds> Iterator for Nodes<'a, N> {
    type Item = &'a N;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// An iterator over the Edges of the graph
#[derive(Debug)]
pub struct Edges<'a, N: NodeBounds> {
    inner: hash_map::Iter<'a, N, Vec<Edge<N>>>,
    curr_node: Option<(&'a N, &'a Vec<Edge<N>>)>,
    curr_dest_no: usize,
}

impl<'a, N: NodeBounds> Iterator for Edges<'a, N> {
    type Item = (&'a N, &'a Edge<N>);

    fn next(&mut self) -> Option<Self::Item> {
        // loop until we get an edge or until there are none left
        loop {
            match self.curr_node {
                // we have a node and enough remaining dests
                Some((node, dests)) if self.curr_dest_no < dests.len() => {
                    self.curr_dest_no += 1;
                    break Some((node, &dests[self.curr_dest_no - 1]));
                }

                // get next thing from inner iterator
                _ => {
                    if let Some(node) = self.inner.next() {
                        self.curr_node = Some(node);

                        self.curr_dest_no = 0;
                    } else {
                        // no more edges to make so return None
                        break None;
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // we can only know how many more edges there are attached to the current node
        if let Some((_, dests)) = self.curr_node {
            (dests.len() - self.curr_dest_no, None)
        } else {
            (0, None)
        }
    }
}
