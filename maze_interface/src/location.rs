use crate::maze_grid::MazeGrid;
use crate::points::Point;
use crate::Edge;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Location {
    pub(crate) side: Edge,
    pub(crate) point: Point,
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
    pub fn has_edge(&self, grid: &MazeGrid) -> bool {
        let v_loc = grid.locate(self);
        let FlatLocation { side, idx } = v_loc;
        let cell = &grid.cells()[idx as usize];
        cell.has_edge(&side)
    }
    pub fn on_border(&self, grid: &MazeGrid) -> bool {
        let v_loc = grid.locate(self);
        let FlatLocation { side, idx } = v_loc;
        let cell = &grid.cells()[idx as usize];
        cell.on_border(&side)
    }
    pub fn locate(&self, length: u8) -> FlatLocation {
        // destructure the Point in cell into col and row variables
        let Location { side, point: cell } = self;
        let row = cell.y();
        let col = cell.x();
        // multiply length by the row number and add the column number to
        // get the actual index in an inefficient grid implementation.
        // this is divided by 2 to get the efficient implementation index,
        // which may be a whole integer number or may have a .5 remainder
        // (which indicates it does not refer to )
        let full_index = (row as u16) * (length as u16) + (col as u16);

        let v_div = (full_index as f32) / 2f32;
        let v_side: Edge;
        let v_idx: u16;
        if v_div != v_div.trunc() {
            let v_len = length as u16 / 2u16;
            match side {
                Edge::N => {
                    if cell.on_top_border(length) {
                        // floor() v_idx here because the northern border of the grid
                        // starts at an even v_idx, meaning if it were ceil()ed the cell
                        // could potentially reference the next row.
                        v_idx = v_div.floor() as u16;
                        v_side = Edge::N;
                    } else {
                        // floor() to get previous cell in vector and remove a row
                        v_idx = (v_div.floor() as u16) - v_len;
                        v_side = Edge::S;
                    }
                }
                Edge::E => {
                    if cell.on_right_border(length) {
                        // here v_idx will never go over the length of the cells vector
                        // as it is floored and then one row is added (since the item is
                        // on the east border the vector must have at least one more row).
                        v_idx = v_div.floor() as u16 + v_len;
                        v_side = Edge::E;
                    } else {
                        // ceil() to get next cell in vector
                        v_idx = v_div.ceil() as u16;
                        v_side = Edge::W;
                    }
                }
                Edge::S => {
                    if cell.on_bottom_border(length) {
                        // ceil() v_idx here because the southern border of the grid
                        // has a v_idx with a remainder of .5, meaning if it were floor()ed
                        // the cell could potentially reference a previous row.
                        v_idx = v_div.ceil() as u16;
                        v_side = Edge::S;
                    } else {
                        // ceil() to get next cell in the vector and add a row
                        v_idx = (v_div.ceil() as u16) + v_len;
                        v_side = Edge::N;
                    }
                }
                Edge::W => {
                    if cell.on_left_border(length) {
                        // here v_idx will never go over the length of the cells vector
                        // as it is ceiled and then one row subtracted (since the item is
                        // on the west border the vector must have at least one previous row).
                        v_idx = v_div.ceil() as u16 - v_len;
                        v_side = Edge::W;
                    } else {
                        // floor() to get previous cell in the vector
                        v_idx = v_div.floor() as u16;
                        v_side = Edge::E;
                    }
                }
            }
            FlatLocation::new(v_idx, v_side)
        } else {
            // index is even, so no change
            FlatLocation::new(v_div as u16, side.clone())
        }
    }
}

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
    pub(crate) side: Edge,
    pub(crate) idx: u16,
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
    pub fn idx(&self) -> u16 {
        self.idx
    }
    pub fn side(&self) -> Edge {
        self.side.clone()
    }
}
