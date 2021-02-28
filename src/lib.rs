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

#![feature(str_split_once)]
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Graph library which provides an adjacency list based directed graph.
//!
//! # Example
//! ```
//! use graph_algos::{Graph, Edge};
//! 
//! let mut graph: Graph<u32> = Graph::empty();
//! 
//! // add an edge from node 0 to node one with a weight of 5
//! graph.add_edge(0, Edge::new_with_weight(1, 5));
//! 
//! // add two more edges
//! graph.add_edge(0, Edge::new_with_weight(2, 2));
//! graph.add_edge(2, Edge::new_with_weight(1, 1));
//! 
//! if graph.is_edge(&0, &1) {
//!     println!("There is an edge from node 0 to node 1");
//! } else {
//!     println!("There is no edge from node 0 to node 1");
//! }
//! ```

// use macros from the failure crate
#[macro_use]
extern crate failure;

mod graph;
pub use graph::{Graph, NodeBounds};

mod edge;
pub use edge::Edge;
pub(crate) use edge::ParseEdgeError;

mod edge_weight;
pub use edge_weight::EdgeWeight;

mod path;
pub use path::{Path, PredMap};

//#[macro_export]
