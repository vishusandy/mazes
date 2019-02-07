#![allow(dead_code)]
#![allow(unused_variables)]

/* Todo:
    Add a render() method to MazeGrid
    Add a display implementation that will draw a maze using ASCII chars (`_`, and `|`)
    In save_png() return the actual file size of the newly created image file instead of 0 always
*/

// mod maze;
pub mod maze_cell;
pub mod maze_grid;
pub mod points;

// use image::{Rgb, RgbImage};
// use imageproc::drawing::{
//     draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
//     draw_hollow_rect_mut, draw_line_segment_mut,
// };
// use imageproc::rect::Rect;

use self::maze_grid::MazeGrid;
use crate::points::Point;
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
    fn from(_err: EdgeFromStrError) -> Self {
        LocationFromStrError
    }
}

impl std::convert::From<std::num::ParseIntError> for LocationFromStrError {
    fn from(_err: std::num::ParseIntError) -> Self {
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
// use lazy_static::*;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use crate::maze_cell::MazeCell;
    use crate::maze_grid::MazeGrid;
    use crate::points::Point;
    use lazy_static::*;
    lazy_static! {
        static ref MAZEGRID: MazeGrid = MazeGrid::new(8u8);
    }
    #[test]
    fn maze_grid_length() {
        // let grid = MazeGrid::new(8u8);
        if MAZEGRID.cells().len() != 32 {
            panic!();
        }
    }
    fn maze_grid(val: bool) {
        let mut i = 0u8;
        println!("");
        if !val {
            print!(" ");
        }
        for cell in MAZEGRID.cells() {
            // if !(i % 16 == 0) {
            //     print!("  ");
            // }

            // top border
            if val {
                if cell.has_left_edge() {
                    print!("  |");
                } else {
                    print!("    ")
                }
            } else {
                if cell.has_left_edge() {
                    print!(" |");
                } else {
                    print!("   ")
                }
            }
            if val {
                print!("{:2}", i);
            } else {
                match (cell.has_top_edge(), cell.has_bottom_edge()) {
                    (true, false) => print!("-"),
                    (true, true) => print!("X"),
                    (false, false) => print!(" "),
                    (false, true) => print!("_"),
                }
            }
            if cell.has_right_edge() {
                print!("|");
            } else {
                print!(" ");
            }

            match i {
                6 | 22 | 38 | 54 => {
                    if val {
                        print!("\n   ")
                    } else {
                        print!("\n   ")
                    }
                }
                14 | 30 | 46 => {
                    if val {
                        print!("\n")
                    } else {
                        print!("\n ")
                    }
                }
                _ => {}
            }

            // if i % 8 == 0 && !cell.on_left() {
            //     panic!("Cell {} was supposed to be on the left border but its state does not indicate this.  State: {}", i, cell.get());
            // }
            // // right border - end of row
            // if i % 16 ==  && !cell.on_right() {
            //     panic!("Cell {} was supposed to be on the right border but its state does not indicate this.  State: {}", i, cell.get());
            // } else if cell.on_right() {
            //     print!("\n  ");
            // }
            // if i % 8 == 6 {
            //     print!("\n");
            // }
            // if i < 8 && !cell.on_top() {
            //     panic!("Cell {} was supposed to be on the top border but its state does not indicate this.  State: {}", i, cell.get());
            // }
            // // bottom border
            // if i >= 56 && !cell.on_bottom() {
            //     panic!("Cell {} was supposed to be on the bottom border but its state does not indicate this.  State: {}", i, cell.get());
            // }

            i += 2;
        }
        println!("\nEnded with last i value being: {}\n\n", i - 2);
    }
    #[test]
    fn test_grid() {
        maze_grid(true);
        maze_grid(false);
    }

    #[test]
    fn test_locate() {
        let mut c = 0u8;
        let mut r = 0u8;
        let mut fail = false;
        for i in 0..64 {
            let row: u8 = i / 8;
            let col: u8 = i % 8;

            if r != row {
                println!("r != row: {} != {}", &r, &row);
                fail = true;
            }
            if c != col {
                println!("c != col: {} != {}", &c, &col);
                fail = true;
            }

            c += 1;
            if c == 8 {
                c = 0;
                r += 1;
            }
        }
        if fail {
            panic!();
        }
    }
}
