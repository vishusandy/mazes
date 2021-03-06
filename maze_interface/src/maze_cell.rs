use crate::points::Point;
use crate::Edge;

/// The MazeCell structure records the state of a MazeGrid cell.  It contains
/// information about what borders the cell is on (top, left, topleft border)
/// as well as whether each side of the cell has a visible edge.
#[derive(Clone, Debug)]
// pub struct MazeCell(u8, u16);
pub struct MazeCell {
    state: u8,
    /// This field represents the number of the cell in the vector using the
    /// compact representation.
    index: u16,
    /// This represents the column number (starting at 0) of the current cell
    x: u16,
    /// This represents the row number (starting at 0) of the current cell
    y: u16,
}
impl MazeCell {
    // Border state constants
    /// Value indicating the cell is on the bottom border
    pub const BOTTOM_BORDER: u8 = 1;
    /// Value indicating the cell is on the left border
    pub const LEFT_BORDER: u8 = 2;
    /// Value indicating the cell is on the right border
    pub const RIGHT_BORDER: u8 = 4;
    /// Value indicating the cell is on the top border
    pub const TOP_BORDER: u8 = 8;

    /// Value indicating the cell has the bottom edge visible
    pub const BOTTOM_EDGE: u8 = 16;
    /// Value indicating the cell has the left edge visible
    pub const LEFT_EDGE: u8 = 32;
    /// Value indicating the cell has the right edge visible
    pub const RIGHT_EDGE: u8 = 64;
    /// Value indicating the cell has the top edge visible
    pub const TOP_EDGE: u8 = 128;

    const ALL_EDGES: u8 = Self::BOTTOM_EDGE + Self::LEFT_EDGE + Self::RIGHT_EDGE + Self::TOP_EDGE;

    /// Creates a new MazeCell with the given positions, assuming a given grid size.
    /// The grid size is used to correctly check if the cell is on a border.
    ///
    /// Only the borders the cell is touching are saved in the cell, the edges will
    /// be all on by default.  To create a new cell without all of the edges use the
    /// blank() function instead of new().
    pub fn new(length: u8, pos: &Point, index: u16) -> Self {
        // let x_offset = if  // TODO: need to offset x because of compact representation
        // let pos: Point = pos + Point::new();

        let len = length as u16;
        // the full index
        let fidx = index * 2;
        let y = fidx / len;
        let x = fidx - (len * y) + if length % 2 == 0 && y % 2 == 1 { 1 } else { 0 };
        let top = y == 0;
        let bottom = y == ((length - 1) as u16);
        let left = x == 0;
        let right = x == ((length - 1) as u16);
        let state = ((top as u8) * Self::TOP_BORDER)
            + ((right as u8) * Self::RIGHT_BORDER)
            + ((bottom as u8) * Self::BOTTOM_BORDER)
            + ((left as u8) * Self::LEFT_BORDER)
            + Self::ALL_EDGES;

        Self {
            // state: (pos.on_bottom_border(length) as u8 * Self::BOTTOM_BORDER)
            //     + (pos.on_top_border(length) as u8 * Self::TOP_BORDER)
            //     + (pos.on_left_border(length) as u8 * Self::LEFT_BORDER)
            //     + (pos.on_right_border(length) as u8 * Self::RIGHT_BORDER)
            //     + Self::ALL_EDGES,
            state,
            index,
            x,
            y,
        }
    }

    pub fn blank(length: u8, pos: &Point, index: u16) -> Self {
        let len = length as u16;
        // the full index
        let fidx = index * 2;
        let y = fidx / len;
        let x = fidx - (len * y) + if y % 2 == 1 { 1 } else { 0 };
        let top = y == 0;
        let bottom = y == ((length - 1) as u16);
        let left = x == 0;
        let right = x == ((length - 1) as u16);
        let state = ((top as u8) * Self::TOP_BORDER)
            + ((right as u8) * Self::RIGHT_BORDER)
            + ((bottom as u8) * Self::BOTTOM_BORDER)
            + ((left as u8) * Self::LEFT_BORDER);
        // + Self::ALL_EDGES;

        Self {
            // state: (pos.on_bottom_border(length) as u8 * Self::BOTTOM_BORDER)
            //     + (pos.on_top_border(length) as u8 * Self::TOP_BORDER)
            //     + (pos.on_left_border(length) as u8 * Self::LEFT_BORDER)
            //     + (pos.on_right_border(length) as u8 * Self::RIGHT_BORDER),
            state,
            index,
            x,
            y,
        }
    }

    /// Get the raw value of the cell's state.
    ///
    /// The constants Self::TOP_EDGE, Self::BOTTOM_EDGE, Self::LEFT_EDGE,
    /// Self::RIGHT_EDGE, Self::TOP, Self::BOTTOM, Self::LEFT, and Self::RIGHT
    /// are assigned values 1-128, these values are used with bitwise AND/OR
    /// operations to record the cell's state.

    #[inline]
    pub fn get(&self) -> u8 {
        self.state
    }

    /// Set the raw value of the cell's state.
    ///
    /// The constants Self::TOP_EDGE, Self::BOTTOM_EDGE, Self::LEFT_EDGE,
    /// Self::RIGHT_EDGE, Self::TOP, Self::BOTTOM, Self::LEFT, and Self::RIGHT
    /// are assigned values 1-128, these values are used with bitwise AND/OR
    /// operations to record the cell's state.
    #[inline]
    pub fn set(&mut self, val: u8) {
        self.state = val;
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    pub fn index(&self) -> u16 {
        self.index
    }
    pub fn idx(&self) -> u16 {
        self.index
    }
    // BORDERS

    /// Check if the cell is on the bottom border
    #[inline]
    pub fn on_bottom(&self) -> bool {
        self.state & Self::BOTTOM_BORDER != 0
    }
    /// Check if the cell is on the  leftborder
    #[inline]
    pub fn on_left(&self) -> bool {
        self.state & Self::LEFT_BORDER != 0
    }
    /// Check if the cell is on the right border
    #[inline]
    pub fn on_right(&self) -> bool {
        self.state & Self::RIGHT_BORDER != 0
    }
    /// Check if the cell is on the top border
    #[inline]
    pub fn on_top(&self) -> bool {
        self.state & Self::TOP_BORDER != 0
    }
    /// Check if the cell is on a specific border
    #[inline]
    pub fn on_border(&self, edge: &Edge) -> bool {
        (self.state & !Self::ALL_EDGES) != 0
    }

    // EDGES

    /// Check if the cell is on any of the 4 borders (sides/edges)
    #[inline]
    pub fn on_any_border(&self) -> bool {
        (!Self::ALL_EDGES & self.state) != 0
    }

    /// Check if the cell has its bottom edge visible
    #[inline]
    pub fn has_bottom_edge(&self) -> bool {
        self.state & Self::BOTTOM_EDGE != 0
    }

    /// Check if the cell has its left edge visible
    #[inline]
    pub fn has_left_edge(&self) -> bool {
        self.state & Self::LEFT_EDGE != 0
    }

    /// Check if the cell has its right edge visible
    #[inline]
    pub fn has_right_edge(&self) -> bool {
        self.state & Self::RIGHT_EDGE != 0
    }

    /// Check if the cell has its top edge visible
    #[inline]
    pub fn has_top_edge(&self) -> bool {
        self.state & Self::TOP_EDGE != 0
    }

    #[inline]
    pub fn has_edge(&self, edge: &Edge) -> bool {
        (self.state & Self::ALL_EDGES) != 0
    }

    #[inline]
    pub fn has_any_edge(&self) -> bool {
        (self.state & Self::ALL_EDGES) != 0
    }

    pub fn have_all_edges(&self) -> bool {
        self.state & Self::ALL_EDGES == Self::ALL_EDGES
    }

    /// Check if the cell can have a specified edge removed.  If the cell lays
    /// on a border then that edge cannot be removed.
    fn can_modify_edge(&self, side: &Edge) -> bool {
        // check if the cell is on the edge to be removed,
        // do not allow the edge to be removed if it is on
        self.on_any_border()
    }

    /// Remove a specified edge from the cell, if the edge does not lay on an
    /// outer border.
    pub fn remove_edge(&mut self, side: &Edge) -> bool {
        if self.can_modify_edge(side) {
            let val = side.value();
            // check if the cell has the specified edge
            if self.state & val != 0 {
                // and remove it
                self.state -= val;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Add a edge to a cell if it does not lay on a border and is not visible yet
    pub fn add_edge(&mut self, side: &Edge) -> bool {
        if self.can_modify_edge(side) {
            let val = side.value();
            if self.state & val == 0 {
                self.state += val;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
