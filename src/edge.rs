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
use std::{fmt::Debug, str::FromStr};

/// An Edge in the graph
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Edge<N: NodeBounds> {
    /// The destination node of the edge
    destination: N,

    /// The "weight" of traversing this edge
    ///
    /// A value of None represents an edge with no weight
    weight: Option<EdgeWeight>,
}

impl<N: NodeBounds> Edge<N> {
    /// Construct a new Edge with no weight
    /// ```
    /// use graph_algos::Edge;
    ///
    /// let edge = Edge::new(5);
    /// assert_eq!(edge.destination(), &5);
    /// assert!(edge.weight().is_none());
    /// ```
    pub fn new(dest: N) -> Self {
        Self {
            destination: dest,
            weight: None,
        }
    }

    /// Construct a new Edge with a given weight
    /// ```
    /// use graph_algos::Edge;
    ///
    /// let edge = Edge::new_with_weight(5, 10);
    /// assert_eq!(edge.destination(), &5);
    /// assert_eq!(edge.weight(), Some(10.into()));
    /// ```
    pub fn new_with_weight(dest: N, weight: impl std::convert::Into<EdgeWeight>) -> Self {
        Self {
            destination: dest,
            weight: Some(weight.into()),
        }
    }

    /// Returns the weight of an edge if it exists
    /// ```
    /// use graph_algos::Edge;
    ///
    /// let edge = Edge::new_with_weight(5, 10);
    /// assert_eq!(edge.weight(), Some(10.into()));
    /// ```
    pub fn weight(&self) -> Option<EdgeWeight> {
        self.weight
    }

    /// Returns a reference to the destination node of the edge
    /// ```
    /// use graph_algos::Edge;
    ///
    /// let edge = Edge::new(5);
    /// assert_eq!(edge.destination(), &5);
    /// ```
    pub fn destination(&self) -> &N {
        &self.destination
    }
}

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

impl<N: NodeBounds> FromStr for Edge<N>
where
    N: FromStr,
    <N as FromStr>::Err: Debug,
{
    type Err = ParseEdgeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if we have a tuple then parse destination and weight
        // otherwise treat as just a destination
        if let Some((v, w)) = s.split_once(',') {
            let destination = v
                .parse()
                .map_err(|err| ParseEdgeError::NodeParseError(format!("{:?}", err)))?;
            let weight: i64 = w.parse().map_err(ParseEdgeError::WeightParseError)?;

            Ok(Edge::new_with_weight(destination, weight))
        } else {
            let destination = s
                .parse()
                .map_err(|err| ParseEdgeError::NodeParseError(format!("{:?}", err)))?;

            Ok(Edge::new(destination))
        }
    }
}
