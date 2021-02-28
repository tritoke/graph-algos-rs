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

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

//! Graph library which provides an adjacency list based directed graph.
//!
//! # Example
//! ```
//! use graph_algos::{Graph, graph};
//!
//! let mut graph: Graph<u32> = graph! {
//!     1 => [2, 3],
//!     2 => [4, 6],
//!     3 => [5, 6],
//!     5 => [6],
//! };
//!
//! assert!(graph.is_edge(&1, &2));
//!
//! graph.remove_edge(&1, &2);
//!
//! assert!(!graph.is_edge(&1, &2));
//!
//! // assert that the second successor of nodes 2 and 3 are the same
//! assert_eq!(
//!     graph.succs(&2).map(|succs| &succs[1]),
//!     graph.succs(&3).map(|succs| &succs[1]),
//! );
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
