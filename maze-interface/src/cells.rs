/* Todo:
    see if you can use a Point for the index in std::ops

    the structs are all private, change that.
      also keep their fields private, but make constructors for them

    make a new struct to return as a location

*/

mod maze_cell;
mod maze_grid;
mod points;

use points::Point;
use std::fmt;

#[derive(Clone, Debug)]
pub struct LocationFromStrError;
impl std::error::Error for LocationFromStrError {
    fn description(&self) -> &str {
        "An error occurred calling from_str() on Location.  To convert a Location object into a String it must be in the format: '0.0, 0.0 N' where N is eone of: N, E, S, or W (upper or lowercase)."
    }
}
impl std::fmt::Display for LocationFromStrError {
    // fn fmt(&self, f: &mut std::fmt:Formatter) -> std::fmt::Result {
    //     write!(f, "An error occurred calling from_str() on Location.  To convert a Location object into a String it must be in the format: '0.0, 0.0 N' where N is eone of: N, E, S, or W (upper or lowercase).")
    // }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred calling from_str() on Location.  To convert a Location object into a String it must be in the format: '0.0, 0.0 N' where N is eone of: N, E, S, or W (upper or lowercase).")
    }
}

#[derive(Clone, Debug)]
pub struct EdgeFromStrError;
impl std::error::Error for EdgeFromStrError {
    fn description(&self) -> &str {
        "An error occurred calling from_str() on Edge.  To convert an Edge object to a String it must be a single character, one of: N, E, S, or W (upper or lowercase)."
    }
}
impl std::fmt::Display for EdgeFromStrError {
    // fn fmt(&self, f: &mut std::fmt:Formatter) -> std::fmt::Result {
    //     write!(f, "An error occurred calling from_str() on Location.  To convert a Location object into a String it must be in the format: '0.0, 0.0 N' where N is eone of: N, E, S, or W (upper or lowercase).")
    // }
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred calling from_str() on Edge.  To convert an Edge object to a String it must be a single character, one of: N, E, S, or W (upper or lowercase).")
    }
}

/// Indicates a side of a cell.
#[derive(Clone, Debug)]
pub enum Edge {
    N,
    W,
    S,
    E,
}

impl Edge {
    const BOTTOM_EDGE: u8 = 16;
    const LEFT_EDGE: u8 = 32;
    const RIGHT_EDGE: u8 = 64;
    const TOP_EDGE: u8 = 128;
    pub fn value(&self) -> u8 {
        match self {
            Edge::N => Self::TOP_EDGE,
            Edge::E => Self::RIGHT_EDGE,
            Edge::S => Self::BOTTOM_EDGE,
            Edge::W => Self::LEFT_EDGE,
        }
    }
}

impl std::str::FromStr for Edge {
    type Err = EdgeFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(EdgeFromStrError)
        } else {
            let upper = s[0..1].to_uppercase();
            match &&upper[..] {
                &"N" => Ok(Edge::N),
                &"E" => Ok(Edge::E),
                &"S" => Ok(Edge::S),
                &"W" => Ok(Edge::W),
                _ => Err(EdgeFromStrError),
            }
        }
    }
}

impl std::convert::From<EdgeFromStrError> for LocationFromStrError {
    fn from(err: EdgeFromStrError) -> Self {
        LocationFromStrError
    }
}

impl std::convert::From<std::num::ParseIntError> for LocationFromStrError {
    fn from(err: std::num::ParseIntError) -> Self {
        LocationFromStrError
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    side: Edge,
    point: Point,
}
impl Location {
    pub fn blank() -> Location {
        Location {
            side: Edge::N,
            // point: Point { x: 0i16, y: 0i16 },
            point: Point::new(0, 0),
        }
    }
    pub fn new(x: i16, y: i16, side: Edge) -> Location {
        // let point: Point = Point { x, y };
        let point: Point = Point::new(x, y);
        Location { side, point }
    }
}

impl std::str::FromStr for Location {
    type Err = LocationFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(',') {
            let split: Vec<_> = s.split(',').collect();
            match &split[..] {
                &[x, y, c] => Ok(Location {
                    side: Edge::from_str(c)?,
                    point: Point::new(x.parse()?, y.parse()?),
                }),
                _ => Err(LocationFromStrError),
            }
        } else {
            Err(LocationFromStrError)
        }
    }
}

#[derive(Clone, Debug)]
pub struct FlatLocation {
    side: Edge,
    idx: u16,
}

impl FlatLocation {
    pub fn new(idx: u16, side: Edge) -> Self {
        Self { side, idx }
    }
    pub fn blank() -> Self {
        Self {
            side: Edge::N,
            idx: 0u16,
        }
    }
}
