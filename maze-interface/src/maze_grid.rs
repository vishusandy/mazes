use crate::maze_cell::MazeCell;
use crate::points::Point;
use crate::{Edge, FlatLocation, Location};

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
        let loc = Location {
            side: (*side).clone(),
            point: cell.clone(),
        };
        let v_loc = self.locate(&loc);
        let FlatLocation { side, idx } = v_loc;
        let cell = &mut self.cells[idx as usize];
        cell.remove_edge(&side)
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
    fn locate(&self, loc: &Location) -> FlatLocation {
        // destructure the Point in cell into col and row variables
        let Location { side, point: cell } = loc;
        let row = cell.y();
        let col = cell.x();
        // multiply length by the row number and add the column number to
        // get the actual index in an inefficient grid implementation.
        // this is divided by 2 to get the efficient implementation index,
        // which may be a whole integer number or may have a .5 remainder
        // (which indicates it does not refer to )
        let full_index = (row as u16) * (self.length as u16) + (col as u16);

        let v_div = (full_index as f32) / 2f32;
        let v_side: Edge;
        let v_idx: u16;
        if v_div != v_div.trunc() {
            let v_len = self.length as u16 / 2u16;
            match side {
                Edge::N => {
                    if cell.on_top_border(self.length) {
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
                    if cell.on_right_border(self.length) {
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
                    if cell.on_bottom_border(self.length) {
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
                    if cell.on_left_border(self.length) {
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

/// This operation will only return a reference to the cell containing
/// the data for the side in question, it will NOT tell you which side
/// of that reference you should use.  For this reason you should be
/// careful when using this method.  Use MazeGrid.locate() to get the
/// index and correct side.
impl std::ops::Index<&Location> for MazeGrid {
    type Output = MazeCell;

    /// This method will only return a reference to the cell containing
    /// the data for the side in question, it will NOT tell you which side
    /// of that reference you should use.  For this reason you should be
    /// careful when using this method.  Use MazeGrid.locate() to get the
    /// index and correct side.
    fn index(&self, loc: &Location) -> &MazeCell {
        // unimplemented!() // TODO
        let v_loc = self.locate(loc);
        &self.cells[v_loc.idx as usize]
    }
}

/// This operation will only return a reference to the cell containing
/// the data for the side in question, it will NOT tell you which side
/// of that reference you should use.  For this reason you should be
/// careful when using this method.  Use MazeGrid.locate() to get the
/// index and correct side.
impl std::ops::IndexMut<&Location> for MazeGrid {
    /// This method will only return a reference to the cell containing
    /// the data for the side in question, it will NOT tell you which side
    /// of that reference you should use.  For this reason you should be
    /// careful when using this method.  Use MazeGrid.locate() to get the
    /// index and correct side.
    fn index_mut<'a>(&'a mut self, loc: &Location) -> &'a mut MazeCell {
        unimplemented!() // TODO
    }
}

// The code below is left commented out to document why this is not and should
// not be implemented.
//
//
// Do not implement Index operations for MazeGrid[Location] as the Location
// will be destroyed and the caller would not be able to call grid.locate() to
// determine the correct side and would be left with only the cell index.
//
// If an Index operation is needed use MazeGrid[&Location] instead.
//
// impl std::ops::Index<Location> for MazeGrid {
//     type Output = MazeCell;
//
//     fn index(&self, loc: Location) -> &MazeCell {
//         unimplemented!() // DO NOT IMPLEMENT.  THIS IS LEFT UNIMPLEMENTED INTENTIONALLY
//     }
// }
//
// Do not implement Index operations for MazeGrid[Location] as the Location
// will be destroyed and the caller would not be able to call grid.locate() to
// determine the correct side and would be left with only the cell index.
//
// If an Index operation is needed use MazeGrid[&Location] instead.
//
// impl std::ops::IndexMut<Location> for MazeGrid {
//     fn index_mut<'a>(&'a mut self, loc: Location) -> &'a mut MazeCell {
//         unimplemented!() // DO NOT IMPLEMENT.  THIS IS LEFT UNIMPLEMENTED INTENTIONALLY
//     }
// }
