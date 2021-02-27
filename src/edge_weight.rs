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

use std::{cmp::Ordering, fmt, ops};

/// A wrapper around i64 to handle the different options
/// for the weight of an edge.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum EdgeWeight {
    /// Weight(value) represents an edge of weight value
    Weight(i64),

    /// PosInfinity represents a weight of +infinity
    PosInfinity,

    /// NegInfinity represents a weight of -infinity
    NegInfinity,
}

impl EdgeWeight {
    /// Create a new EdgeWeight with a given weight
    /// ```
    /// use graph_algos::EdgeWeight;
    ///
    /// let w = EdgeWeight::new(5);
    /// assert_eq!(w, EdgeWeight::Weight(5_i64));
    /// ```
    pub fn new(weight: i64) -> Self {
        Self::Weight(weight)
    }

    /// reuturns positive infinity
    /// ```
    /// use graph_algos::EdgeWeight;
    ///
    /// let inf = EdgeWeight::infinity();
    /// assert_eq!(inf, EdgeWeight::PosInfinity);
    /// ```
    pub fn infinity() -> Self {
        Self::PosInfinity
    }

    /// reuturns negative infinity
    /// ```
    /// use graph_algos::EdgeWeight;
    ///
    /// let neg_inf = EdgeWeight::neg_infinity();
    /// assert_eq!(neg_inf, EdgeWeight::NegInfinity);
    /// ```
    pub fn neg_infinity() -> Self {
        Self::NegInfinity
    }

    /// Inverts the sign of the contained weight and returns the new weight
    /// ```
    /// use graph_algos::EdgeWeight;
    ///
    /// let neg_inf = EdgeWeight::neg_infinity();
    /// let inf = EdgeWeight::infinity();
    /// let w = EdgeWeight::Weight(5);
    ///
    /// assert_eq!(neg_inf.flip_sign(), inf);
    /// assert_eq!(inf.flip_sign(), neg_inf);
    /// assert_eq!(w.flip_sign(), EdgeWeight::Weight(-5));
    /// ```
    pub fn flip_sign(&self) -> Self {
        match self {
            Self::Weight(w) => Self::Weight(-1 * w),
            Self::PosInfinity => Self::NegInfinity,
            Self::NegInfinity => Self::PosInfinity,
        }
    }
}

impl Default for EdgeWeight {
    fn default() -> Self {
        Self::Weight(0)
    }
}

impl std::convert::From<i64> for EdgeWeight {
    fn from(weight: i64) -> EdgeWeight {
        EdgeWeight::new(weight)
    }
}

impl std::str::FromStr for EdgeWeight {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We just attempt to parse a weight of either infinity makes no sense
        Ok(EdgeWeight::Weight(s.parse()?))
    }
}

impl fmt::Display for EdgeWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weight(w) => write!(f, "{}", w),
            Self::PosInfinity => write!(f, "+inf"),
            Self::NegInfinity => write!(f, "-inf"),
        }
    }
}

impl ops::Add for EdgeWeight {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        use EdgeWeight::*;
        match (self, other) {
            (Weight(a), Weight(b)) => Weight(a + b),
            // Anything + either infinity is that infinity
            (inf, Weight(_)) | (Weight(_), inf) => inf,
            // if we have the same kind of infinity then forward that on
            (inf1, inf2) if inf1 == inf2 => inf1,
            (inf1, inf2) => panic!("Cannot add {:?} and {:?} - undefined.", inf1, inf2),
        }
    }
}

impl ops::Sub for EdgeWeight {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        use EdgeWeight::*;
        match (self, other) {
            (Weight(a), Weight(b)) => Weight(a - b),
            // an infinity - x = that infinity
            (inf, Weight(_)) => inf,
            // an x - infinity = the infinity with its sign inverted
            (Weight(_), inf) => inf.flip_sign(),
            // if the signs on the infinities differ then its
            (inf1, inf2) if inf1 != inf2 => inf1,
            (inf1, inf2) => panic!("Cannot subtract {:?} and {:?} - undefined.", inf1, inf2),
        }
    }
}

impl Ord for EdgeWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        use EdgeWeight::*;

        match (self, other) {
            (Weight(a), Weight(b)) => a.cmp(b),
            (PosInfinity, PosInfinity) => Ordering::Equal,
            (NegInfinity, NegInfinity) => Ordering::Equal,
            (PosInfinity, _) => Ordering::Greater,
            (NegInfinity, _) => Ordering::Less,
            (Weight(_), PosInfinity) => Ordering::Less,
            (Weight(_), NegInfinity) => Ordering::Greater,
        }
    }
}

impl PartialOrd for EdgeWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
