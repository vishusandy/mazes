use crate::cells::maze_cell::MazeCell;
use crate::cells::points::Point;
use crate::cells::{Edge, FlatLocation, Location};

/// MazeGrid is primarily a Vector of MazeCells.  There are two good ways to
/// organize the cells in the vector.
///
/// 1. The layout is unoptimized, and every cell has a North, East, West, South
/// border.  This means that cells sharing a border will have to ensure the
/// neighboring border is changed to be the same, so if the top right cell has
/// its eastern border removed, the second cell in the top row must also have
/// its western border removed.  An alternative would to be to just ignore all
/// Western and Northern borders and rely on the neighbors below and/or east of
/// the current cell for shared border data.  Both approaches have an
/// inefficient memory layout.
///
/// 2. The cell grid would use an optimized layout so that every other cell is
/// empty, such that the zero index would be a MazeCell, but the second index
/// would merely refer to the MazeCell next to it (the one before hand for the
/// left, or the one next to it for the right).  The cells would be split into
/// rows; if the dimensions are 8x8 then the first 4 cells would represent the
/// 0, 2, 4, & 8th indicies on the top, the next 4 cells would be the 1, 3, 5,
/// 7th indicies of the second row, the next 4 cells would be the 0, 2, 4, 8th
/// indicies on the third row, the next 4 cells would be the 1, 3, 5, 7th
/// indicies on the fourth row, and this pattern will continue until the last
/// row is filled.  This approach will still have wasted memory but much less.
/// The amount of data wasted would be equal to: length plus one bit.  This is
/// twice as efficient in regards to memory size compared to the naive method.
#[derive(Clone, Debug)]
pub struct MazeGrid {
    length: u8,
    cells: Vec<MazeCell>,
}

impl MazeGrid {
    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn cells(&self) -> &Vec<MazeCell> {
        &self.cells
    }

    pub fn modify(&mut self) -> &mut Vec<MazeCell> {
        &mut self.cells
    }

    pub fn new(length: u8) -> Self {
        let cells: Vec<MazeCell> = Vec::with_capacity((length * length + 1) as usize);
        Self { length, cells }
    }

    /// carve() removes a wall from a given side of a cell and returns true if
    /// changes were made, or false if no changes were made.
    fn carve(&mut self, cell: &Point, side: &Edge) -> bool {
        false
    }

    /// The locate() method is used to translate regular grid coordinates into
    /// the optimized virtual representation of the grid (wherein a four-sided
    /// cell is created for every other grid cell; only every other cell is
    /// used in order to optimize the memory layout and space, as a four-sided
    /// cell for each cell would result in wasted space as each cell would have
    /// shared borders with its neighbors).
    ///
    /// locate() takes a MazeGrid (self), and a Location and determines and
    /// returns the cell and new edge that stores the data for that edge.
    // fn locate(&self, cell: &Point, side: &Edge) -> Location {
    fn locate(&self, loc: &Location) -> Option<FlatLocation> {
        let Location { side, point: cell } = loc;
        // destructure the Point in cell into col and row variables

        let row = cell.y();
        let col = cell.x();
        let full_index = (row as u16) * (self.length as u16) + (col as u16);

        let v_div = (full_index as f32) / 2f32;
        let v_side: Edge;
        let v_idx: u16;
        if v_div != v_div.ceil() {
            match side {
                Edge::N => {
                    if cell.on_top_border(self.length) {
                        return None;
                    } else {
                        v_idx = (v_div.floor() as u16) - (self.length as u16 / 2u16);
                        v_side = Edge::S;
                    }
                }
                Edge::E => {
                    if cell.on_right_border(self.length) {
                        return None;
                    } else {
                        v_idx = v_div.ceil() as u16;
                        v_side = Edge::W;
                    }
                }
                Edge::S => {
                    if cell.on_bottom_border(self.length) {
                        return None;
                    } else {
                        v_idx = (v_div.ceil() as u16) + (self.length as u16 / 2u16);
                        v_side = Edge::N;
                    }
                }
                Edge::W => {
                    if cell.on_left_border(self.length) {
                        return None;
                    } else {
                        v_idx = v_div.floor() as u16;
                        v_side = Edge::E;
                    }
                }
            }
            Some(FlatLocation::new(v_idx, v_side))
        } else {
            // index is even, so no change
            Some(FlatLocation::new(v_div as u16, side.clone()))
        }

        /*
        let adjusted_cell = match side {
            // does the negative point numbers work out with borders correctly?
            Edge::N => {
                if cell.y() != 0 {
                    Point::new(0, 0)
                } else {
                    Point::new(0, 0)
                }
            }
            Edge::W => {
                if cell.x() != 0 {
                    Point::new(0, 0)
                } else {
                    Point::new(0, 0)
                }
            }
            Edge::S => {
                if cell.x() < self.length as i16 {
                    Point::new(0, 0)
                } else {
                    Point::new(0, 0)
                }
            }
            Edge::E => {
                if cell.y() < self.length as i16 {
                    Point::new(0, 0)
                } else {
                    Point::new(0, 0)
                }
            }
        };
        // let Point { x: col, y: row } = adjusted_cell;
        let (x, y) = adjusted_cell.get();
        // let index = ;

        Some(FlatLocation::blank())
         */
    }
}

impl std::ops::Index<Location> for MazeGrid {
    type Output = Location;

    fn index(&self, loc: Location) -> &Location {
        unimplemented!() // TODO
    }
}

impl std::ops::IndexMut<Location> for MazeGrid {
    fn index_mut<'a>(&'a mut self, loc: Location) -> &'a mut Location {
        unimplemented!() // TODO
    }
}

impl std::ops::Index<&Location> for MazeGrid {
    type Output = Location;

    fn index(&self, loc: &Location) -> &Location {
        unimplemented!() // TODO
    }
}

impl std::ops::IndexMut<&Location> for MazeGrid {
    fn index_mut<'a>(&'a mut self, loc: &Location) -> &'a mut Location {
        unimplemented!() // TODO
    }
}
