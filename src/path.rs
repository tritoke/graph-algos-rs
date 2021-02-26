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

use crate::{EdgeWeight, NodeBounds};
use std::collections::HashMap;
use std::fmt;

/// Reepresents the predecessor map generated by various
pub type PredMap<'a, N> = HashMap<&'a N, (&'a N, Option<EdgeWeight>)>;

/// Represents a path through a graph as a start node
/// then pairs of destination node and optionally, the edge weight
#[derive(Debug)]
pub struct Path<'a, N: NodeBounds> {
    head: &'a N,
    edges: Vec<(&'a N, Option<EdgeWeight>)>,
}

impl<'a, N: NodeBounds> Path<'a, N> {
    /// extracts a path from the predecessor map and an end node
    pub fn new_path_to(pred_map: &PredMap<'a, N>, end_node: &'a N) -> Result<Self, &'static str> {
        let mut rev_path: Vec<(&'a N, Option<EdgeWeight>)> = Vec::new();
        let mut next_node: &N = end_node;

        while let Some(u) = pred_map.get(next_node) {
            if u.0 != next_node {
                rev_path.push((next_node, u.1));
                next_node = u.0;
            } else {
                rev_path.push(*u);
                break;
            }
        }

        if pred_map.get(next_node).is_some() {
            let (head, _) = rev_path
                .pop()
                .ok_or("no path exists to destination node.")?;
            rev_path.reverse();

            Ok(Self {
                head,
                edges: rev_path,
            })
        } else {
            Err("no path exists to destination node.")
        }
    }
}

impl<'a, N: NodeBounds> fmt::Display for Path<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.head)?;

        for (v, weight) in self.edges.iter() {
            if let Some(w) = weight {
                write!(f, " --({})-> {:?}", w, v)?;
            } else {
                write!(f, " --> {:?}", v)?;
            }
        }

        Ok(())
    }
}
