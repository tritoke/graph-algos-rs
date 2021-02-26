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

/// An Edge in the graph
#[derive(Debug)]
pub struct Edge<N: NodeBounds> {
    /// The destination node of the edge
    destination: N,

    /// The "weight" of traversing this edge
    ///
    /// A value of None represents an edge with no weight
    weight: Option<EdgeWeight>,
}

impl<N: NodeBounds> Edge<N> {
    /// Returns a reference to the weight of an edge if it exists
    pub fn weight(&self) -> Option<&EdgeWeight> {
        self.weight.as_ref()
    }

    /// Returns a reference to the destination node of the edge
    pub fn destination(&self) -> &N {
        &self.destination
    }
}

impl<N: NodeBounds> PartialEq for Edge<N> {
    fn eq(&self, other: &Self) -> bool {
        self.destination() == other.destination()
    }
}

impl<N: NodeBounds> Eq for Edge<N> {}

/// represents the failure to parse an edge
#[derive(Fail, Debug)]
pub enum ParseEdgeError {
    /// Represents the failure to parse the weight of an edge
    #[fail(display = "Failed to parse edge weight: {}", _0)]
    WeightParseError(#[fail(cause)] std::num::ParseIntError),

    /// Represents the failure to parse the destination node in an edge
    #[fail(display = "Failed to parse destination node: {}", _0)]
    NodeParseError(String),
}

impl<N: NodeBounds> std::str::FromStr for Edge<N> {
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if we have a tuple then parse destination and weight
        // otherwise treat as just a destination
        if let Some((v, w)) = s.split_once(',') {
            let destination = v
                .parse()
                .map_err(|err| ParseEdgeError::NodeParseError(format!("{:?}", err)))?;
            let weight = w.parse().map_err(ParseEdgeError::WeightParseError)?;

            Ok(Edge {
                destination,
                weight: Some(weight),
            })
        } else {
            let destination = s
                .parse()
                .map_err(|err| ParseEdgeError::NodeParseError(format!("{:?}", err)))?;
            Ok(Edge {
                destination,
                weight: None,
            })
        }
    }
}
