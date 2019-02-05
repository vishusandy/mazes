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
