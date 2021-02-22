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

pub trait NodeTraits: Hash + FromStr<Err: Debug> + Debug + Eq + Clone {}
impl<T: Hash + FromStr<Err: Debug> + Debug + Eq + Clone> NodeTraits for T {}

#[derive(Debug)]
pub struct Graph<N>
where
    N: NodeTraits,
{
    /// the graph is backed by a hashmap from a node to a vector of nodes
    pub(super) backing_map: HashMap<N, Vec<N>>,
}

impl<N: NodeTraits> Graph<N> {
    /// creates a new empty graph
    pub fn empty() -> Self {
        Self {
            backing_map: HashMap::new(),
        }
    }

    /// adds and edge to the graph
    pub fn add_edge(&mut self, u: N, v: N) {
        self.backing_map
            .entry(u)
            .or_insert_with(Vec::new)
            .push(v.clone());
        self.backing_map.entry(v).or_insert_with(Vec::new);
    }

    /// removes and edge from the graph
    pub fn remove_edge(&mut self, u: &N, v: &N) {
        if let Some(edges) = self.backing_map.get_mut(u) {
            if let Some(pos) = edges.iter().position(|e| *e == *v) {
                edges.remove(pos);
            }
        }
    }

    /// Returns whether an edge exists in the graph
    pub fn is_edge(&self, u: &N, v: &N) -> bool {
        if let Some(succs) = self.backing_map.get(u) {
            succs.contains(v)
        } else {
            false
        }
    }

    /// Returns the successors of a node in the graph
    pub fn succs(&self, u: &N) -> Option<&[N]> {
        self.backing_map.get(u).map(|succs| succs.as_slice())
    }

    /// Returns the number of nodes in a graph
    #[inline]
    pub fn len(&self) -> usize {
        self.backing_map.len()
    }

    /// Returns whether the graph is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.backing_map.is_empty()
    }

    /// Returns a reference to a node in the graph
    pub fn node(&self, needle: N) -> Option<&N> {
        self.backing_map.keys().find(|&key| *key == needle)
    }

    /// Returns an iterator over the nodes in a graph
    pub fn nodes(&self) -> Nodes<N> {
        Nodes {
            inner: self.backing_map.keys(),
        }
    }

    /// Returns an iterator over the edges in the graph
    pub fn edges(&self) -> Edges<N> {
        Edges {
            inner: self.backing_map.iter(),
            curr_node: None,
            curr_dest_no: 0,
        }
    }

    /// fill an unweighted directed graph from a string
    /// each line is node:edges seperated by spaces
    pub fn fill_from_str(&mut self, s: &str) {
        // the first node added to the graph
        for line in s.lines() {
            let (u, vs) = line.split_once(':').expect("Invalid graph format.");

            let u_fs: N = u.parse().expect("Failed to parse origin node.");

            let vs_fs = vs
                .split(' ')
                .map(|v| v.parse().expect("Failed to parse destination node."));

            for v in vs_fs {
                self.add_edge(u_fs.clone(), v);
            }
        }
    }
}

/// An iterator over the nodes of the graph

pub struct Nodes<'a, N>
where
    N: NodeTraits,
{
    inner: hash_map::Keys<'a, N, Vec<N>>,
}

impl<'a, N> Iterator for Nodes<'a, N>
where
    N: NodeTraits,
{
    type Item = &'a N;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// An iterator over the Edges of the graph
pub struct Edges<'a, N>
where
    N: NodeTraits,
{
    inner: hash_map::Iter<'a, N, Vec<N>>,
    curr_node: Option<(&'a N, &'a Vec<N>)>,
    curr_dest_no: usize,
}

impl<'a, N> Iterator for Edges<'a, N>
where
    N: NodeTraits,
{
    type Item = (&'a N, &'a N);

    fn next(&mut self) -> Option<Self::Item> {
        // loop until we get an edge or until there are none left
        loop {
            match self.curr_node {
                // we have a node and enough remaining dests
                Some((node, dests)) if dests.len() < self.curr_dest_no => {
                    break Some((node, &dests[self.curr_dest_no]));
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
