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
use std::{
    fmt::{self, Debug},
    hash::Hash,
    str::FromStr,
};

use crate::{Edge, EdgeWeight, ParseEdgeError};

/// A trait to represent all of the bounds that a node in the graph must provide
pub trait NodeBounds: Hash + Debug + Eq + Clone {}
impl<T: Hash + Debug + Eq + Clone> NodeBounds for T {}

/// A node-generic graph type implemented using an adjacency list
/// Where the successors of a node are stored in a hashmap.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Graph<N: NodeBounds> {
    /// the graph is backed by a hashmap from a node to a vector of nodes
    backing_map: HashMap<N, Vec<Edge<N>>>,
}

/// A macro to construct graphs in a more visual way
///
/// This can be used for unweighted graphs:
/// ```
/// use graph_algos::{graph, Graph};
///
/// // use the macro to define a graph
/// let graph1: Graph<u32> = graph! {
///     1 => [2, 3],
///     2 => [4, 6],
///     3 => [5, 6],
///     5 => [6],
/// };
///
/// // parse from a string (much nicer with include_str!)
/// let graph2: Graph<u32> = "\
///     1:2 3\n\
///     2:4 6\n\
///     3:5 6\n\
///     5:6"
/// .parse()
/// .unwrap();
///
/// // make sure they all have the same number of nodes
/// assert_eq!(graph1.len(), graph2.len());
///
/// let mut g1_nodes = graph1.nodes().collect::<Vec<_>>();
/// let mut g2_nodes = graph2.nodes().collect::<Vec<_>>();
///
/// g1_nodes.sort();
/// g2_nodes.sort();
///
/// // make sure they all have the same nodes
/// assert_eq!(g1_nodes, g2_nodes);
///
/// let mut g1_edges = graph1.edges().collect::<Vec<_>>();
/// let mut g2_edges = graph2.edges().collect::<Vec<_>>();
///
/// g1_edges.sort();
/// g2_edges.sort();
///
/// // make sure they all have the same edges
/// assert_eq!(g1_edges, g2_edges);
/// ```
///
/// ... and for weighted ones:
/// ```
/// use graph_algos::{graph, Graph};
///
/// // use the macro to define a weighted graph
/// let graph1: Graph<&str> = graph! {
///     "a" => [ "c" => 2, "b" => 3 ],
///     "b" => [ "e" => 6, "d" => 5 ],
///     "c" => [ "g" => 2, "f" => 1 ],
///     "d" => [ "i" => 2, "h" => 3 ],
///     "e" => [ "h" => 7 ],
///     "f" => [ "e" => 6 ],
///     "i" => [ "b" => 4 ],
/// };
///
/// // parse from a string
/// let graph2: Graph<String> = "\
///     a:c,2 b,3\n\
///     b:e,6 d,5\n\
///     c:g,2 f,1\n\
///     d:i,2 h,3\n\
///     e:h,7\n\
///     f:e,6\n\
///     i:b,4\n\
/// "
/// .parse()
/// .unwrap();
///
/// // make sure they all have the same number of nodes
/// assert_eq!(graph1.len(), graph2.len());
///
/// let mut g1_nodes = graph1.nodes().map(|node| *node).collect::<Vec<_>>();
/// let mut g2_nodes = graph2.nodes().map(|node| node.as_str()).collect::<Vec<_>>();
///
/// g1_nodes.sort();
/// g2_nodes.sort();
///
/// // make sure they all have the same nodes
/// assert_eq!(g1_nodes, g2_nodes);
///
/// let mut g1_edges = graph1
///     .edges()
///     .map(|(node, edge)| (*node, *edge.destination(), edge.weight()))
///     .collect::<Vec<_>>();
/// let mut g2_edges = graph2
///     .edges()
///     .map(|(node, edge)| (node.as_str(), edge.destination().as_str(), edge.weight()))
///     .collect::<Vec<_>>();
///
/// g1_edges.sort();
/// g2_edges.sort();
///
/// assert_eq!(g1_edges, g2_edges);
/// ```
#[macro_export]
macro_rules! graph {
    ($($node:expr => [$($edge:expr),* $(,)*]),* $(,)*) => {{
        let mut graph = ::graph_algos::Graph::empty();
        $($(graph.add_edge($node, ::graph_algos::Edge::new($edge));)*)*
        graph
    }};
    ($($node:expr => [$($edge:expr => $weight:expr),* $(,)*]),* $(,)*) => {{
        let mut graph = ::graph_algos::Graph::empty();
        $($(graph.add_edge($node, ::graph_algos::Edge::new_with_weight($edge, $weight));)*)*
        graph
    }};
}

impl<N: NodeBounds> Graph<N> {
    /// creates a new empty graph
    /// ```
    /// use graph_algos::Graph;
    ///
    /// let graph: Graph<u32> = Graph::empty();
    ///
    /// assert!(graph.is_empty());
    /// ```
    pub fn empty() -> Self {
        Default::default()
    }

    /// adds and edge to the graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let mut graph: Graph<u32> = Graph::empty();
    /// assert!(graph.is_empty());
    ///
    /// graph.add_edge(5, Edge::new(6));
    /// assert!(!graph.is_empty());
    /// // we now have 2 nodes in the graph
    /// assert_eq!(graph.len(), 2);
    /// assert!(graph.is_edge(&5, &6));
    /// ```
    pub fn add_edge(&mut self, u: N, e: Edge<N>) {
        self.backing_map
            .entry(e.destination().clone())
            .or_insert_with(Vec::new);

        self.backing_map.entry(u).or_insert_with(Vec::new).push(e);
    }

    /// removes and edge from the graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let mut graph: Graph<u32> = [
    ///     (5_u32, Edge::new(6_u32)),
    ///     (6_u32, Edge::new(7_u32)),
    ///     (7_u32, Edge::new(8_u32)),
    ///     (8_u32, Edge::new(5_u32)),
    /// ].iter().cloned().collect();
    ///
    /// assert_eq!(graph.len(), 4);
    /// assert!(graph.is_edge(&5, &6));
    ///
    /// graph.remove_edge(&5, &6);
    /// assert!(!graph.is_edge(&5, &6));
    ///
    /// assert_eq!(graph.len(), 4);
    ///
    /// ```
    pub fn remove_edge(&mut self, u: &N, v: &N) {
        if let Some(edges) = self.backing_map.get_mut(u) {
            if let Some(pos) = edges.iter().position(|e| e.destination() == v) {
                edges.remove(pos);
            }
        }
    }

    /// Returns whether an edge exists in the graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let mut graph: Graph<u32> = Graph::empty();
    ///
    /// graph.add_edge(5, Edge::new(6));
    /// assert!(graph.is_edge(&5, &6));
    /// ```
    pub fn is_edge(&self, u: &N, v: &N) -> bool {
        if let Some(succs) = self.backing_map.get(u) {
            succs.iter().find(|edge| edge.destination() == v).is_some()
        } else {
            false
        }
    }

    /// Returns the successors of a node in the graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let graph: Graph<u32> = [
    ///     (5_u32, Edge::new(6_u32)),
    ///     (5_u32, Edge::new(7_u32)),
    ///     (5_u32, Edge::new(8_u32)),
    ///     (5_u32, Edge::new(9_u32)),
    /// ].iter().cloned().collect();
    ///
    /// let correct = [6, 7, 8, 9].iter().map(|edge| Edge::new(*edge)).collect::<Vec<_>>();
    ///
    /// assert!(correct.eq(graph.succs(&5).unwrap()));
    /// ```
    pub fn succs(&self, u: &N) -> Option<&[Edge<N>]> {
        self.backing_map.get(u).map(|vec| vec.as_slice())
    }

    /// Returns the number of nodes in a graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let graph: Graph<u32> = [
    ///     (5_u32, Edge::new(6_u32)),
    ///     (5_u32, Edge::new(7_u32)),
    ///     (5_u32, Edge::new(8_u32)),
    ///     (5_u32, Edge::new(9_u32)),
    /// ].iter().cloned().collect();
    ///
    /// assert_eq!(graph.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.backing_map.len()
    }

    /// Returns whether the graph is empty
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let mut graph: Graph<u32> = Graph::empty();
    /// assert!(graph.is_empty());
    ///
    /// graph.add_edge(5, Edge::new(6));
    /// assert!(!graph.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.backing_map.is_empty()
    }

    /// Returns an iterator over the nodes in a graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let graph: Graph<u32> = [
    ///     (5_u32, Edge::new(6_u32)),
    ///     (5_u32, Edge::new(7_u32)),
    ///     (5_u32, Edge::new(8_u32)),
    ///     (5_u32, Edge::new(9_u32)),
    /// ].iter().cloned().collect();
    ///
    /// let correct: Vec<&u32> = [5, 6, 7, 8, 9].iter().collect();
    /// let mut nodes: Vec<&u32> = graph.nodes().collect();
    /// nodes.sort();
    ///
    /// assert_eq!(nodes, correct);
    /// ```
    pub fn nodes(&self) -> Nodes<'_, N> {
        Nodes {
            inner: self.backing_map.keys(),
        }
    }

    /// Returns an iterator over the edges in the graph
    /// ```
    /// use graph_algos::{Graph, Edge};
    ///
    /// let graph: Graph<u32> = [
    ///     (5_u32, Edge::new(6_u32)),
    ///     (5_u32, Edge::new(7_u32)),
    ///     (5_u32, Edge::new(8_u32)),
    ///     (5_u32, Edge::new(9_u32)),
    /// ].iter().cloned().collect();
    ///
    /// let correct: Vec<(u32, Edge<u32>)> = vec![
    ///     (5, Edge::new(6)),
    ///     (5, Edge::new(7)),
    ///     (5, Edge::new(8)),
    ///     (5, Edge::new(9)),
    /// ];
    ///
    /// let mut edges: Vec<(u32, Edge<u32>)> = graph.edges().map(|(a, b)| (a.clone(), b.clone())).collect();
    /// edges.sort();
    ///
    /// assert!(correct.iter().eq(edges.iter()));
    /// ```
    pub fn edges(&self) -> Edges<'_, N> {
        Edges {
            inner: self.backing_map.iter(),
            curr_node: None,
            curr_dest_no: 0,
        }
    }
}

impl<N: NodeBounds + Ord + fmt::Display> Graph<N> {
    /// to_string is intended to be a direct inverse of the parse method
    /// it relies on the fmt::Display implementation for the node type
    /// being able to produce a string which can be parsed with .parse()
    ///
    /// Note: this is very expensive and requires additional trait bounds.
    /// This is because we copy the data into a BTreeMap which has an implicit
    /// ordering and thus we will always get a consistent output.
    /// ```
    /// use graph_algos::{Graph, graph};
    ///
    /// let graph: Graph<u32> = graph! {
    ///     1 => [2, 3],
    ///     2 => [4, 5],
    ///     3 => [6, 7],
    ///     4 => [8, 9],
    ///     6 => [5],
    ///     9 => [2],
    /// };
    ///
    /// let correct: String = "\
    ///     1:2 3\n\
    ///     2:4 5\n\
    ///     3:6 7\n\
    ///     4:8 9\n\
    ///     6:5\n\
    ///     9:2\n\
    /// ".into();
    ///
    /// let repr = graph.to_string();
    ///
    /// assert_eq!(repr, correct);
    ///
    /// // parse back to a graph
    /// let parsed_graph: Graph<u32> = repr.as_str().parse().unwrap();
    ///
    /// assert_eq!(graph, parsed_graph);
    /// ```
    pub fn to_string(&self) -> String {
        let mut buf = String::new();

        // collect the backing hashmap into a BTreeMap so we can read it out sorted
        let btree_graph: std::collections::BTreeMap<_, _> = self.backing_map.iter().collect();

        for (node, succs) in btree_graph.iter() {
            if !succs.is_empty() {
                // start the line
                buf.push_str(format!("{}:", node).as_str());

                // get an iterator over the successors
                let mut out_it = succs.iter();

                // handle the first now now so we don't get trailing spaces later
                if let Some(first_edge) = out_it.next() {
                    buf.push_str(format!("{}", first_edge.destination()).as_str());
                    if let Some(EdgeWeight::Weight(w)) = first_edge.weight() {
                        buf.push_str(format!(",{}", w).as_str());
                    }
                }

                // handle the remaining edges
                for edge in out_it {
                    buf.push_str(" ");
                    buf.push_str(format!("{}", edge.destination()).as_str());
                    if let Some(EdgeWeight::Weight(w)) = edge.weight() {
                        buf.push_str(format!(",{}", w).as_str());
                    }
                }

                // end the line
                buf.push('\n');
            }
        }

        buf
    }
}

impl<N: NodeBounds + Ord + fmt::Display> Graph<N> {
    /// to_string is intended to be a direct inverse of the parse method
    /// it relies on the fmt::Display implementation for the node type
    /// being able to produce a string which can be parsed with `.parse()`
    ///
    /// Note: this is far cheaper than `to_string_sorted`, but the output is unstable.
    /// ```
    /// use graph_algos::{Graph, graph};
    ///
    /// let graph: Graph<u32> = graph! {
    ///     1 => [2, 3],
    ///     2 => [4, 5],
    ///     3 => [6, 7],
    ///     4 => [8, 9],
    ///     6 => [5],
    ///     9 => [2],
    /// };
    ///
    /// let repr = graph.to_string_unstable();
    ///
    /// let correct: String = "\
    ///     1:2 3\n\
    ///     2:4 5\n\
    ///     3:6 7\n\
    ///     4:8 9\n\
    ///     6:5\n\
    ///     9:2\n\
    /// ".into();
    ///
    /// // parse back to a graph
    /// let parsed_graph: Graph<u32> = repr.as_str().parse().unwrap();
    ///
    /// assert_eq!(graph, parsed_graph);
    /// ```
    pub fn to_string_unstable(&self) -> String {
        let mut buf = String::new();

        for (node, succs) in self.backing_map.iter() {
            if !succs.is_empty() {
                // start the line
                buf.push_str(format!("{}:", node).as_str());

                // get an iterator over the successors
                let mut out_it = succs.iter();

                // handle the first now now so we don't get trailing spaces later
                if let Some(first_edge) = out_it.next() {
                    buf.push_str(format!("{}", first_edge.destination()).as_str());
                    if let Some(EdgeWeight::Weight(w)) = first_edge.weight() {
                        buf.push_str(format!(",{}", w).as_str());
                    }
                }

                // handle the remaining edges
                for edge in out_it {
                    buf.push_str(" ");
                    buf.push_str(format!("{}", edge.destination()).as_str());
                    if let Some(EdgeWeight::Weight(w)) = edge.weight() {
                        buf.push_str(format!(",{}", w).as_str());
                    }
                }

                // end the line
                buf.push('\n');
            }
        }

        buf
    }
}

impl<N: NodeBounds + fmt::Display> fmt::Display for Graph<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // the intention of this is to mimic the macro syntax
        if self.is_empty() {
            write!(f, "{{}}")?;
        } else {
            writeln!(f, "{{")?;
            for (node, edges) in self.backing_map.iter() {
                if edges.is_empty() {
                    writeln!(f, "    {} => [],", node)?;
                } else {
                    write!(f, "    {} => [", node)?;
                    let mut edge_it = edges.iter();

                    // handle the first edge so we don't get trailing ", " later
                    if let Some(first_edge) = edge_it.next() {
                        write!(f, "{}", first_edge.destination())?;
                        if let Some(EdgeWeight::Weight(w)) = first_edge.weight() {
                            write!(f, " => {}", w)?;
                        }
                    }

                    // write out the remaining edges
                    for edge in edge_it.next() {
                        write!(f, ", {}", edge.destination())?;
                        if let Some(EdgeWeight::Weight(w)) = edge.weight() {
                            write!(f, " => {}", w)?;
                        }
                    }

                    writeln!(f, "],")?;
                }
            }
            write!(f, "}}")?;
        }

        Ok(())
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

impl<N: NodeBounds> FromStr for Graph<N>
where
    N: FromStr,
    <N as FromStr>::Err: Debug,
{
    type Err = GraphParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fill an unweighted directed graph from a string
        // each line is node:edges seperated by spaces

        // new empty graph
        let mut graph: Graph<N> = Graph::empty();

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

impl<N: NodeBounds> std::iter::FromIterator<(N, Edge<N>)> for Graph<N> {
    fn from_iter<I: IntoIterator<Item = (N, Edge<N>)>>(iter: I) -> Self {
        let mut graph: Graph<N> = Graph::empty();

        for (src, edge) in iter {
            graph.add_edge(src, edge);
        }

        graph
    }
}
