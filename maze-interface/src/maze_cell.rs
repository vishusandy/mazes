use crate::points::Point;
use crate::Edge;

/// The MazeCell structure records the state of a MazeGrid cell.  It contains
/// information about what borders the cell is on (top, left, topleft border)
/// as well as whether each side of the cell has a visible edge.
#[derive(Clone, Debug)]
pub struct MazeCell(u8);

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
    /// blank() function instead of new()
    pub fn new(grid_size: u8, pos: &Point) -> Self {
        Self(
            (pos.on_bottom_border(grid_size) as u8 * Self::BOTTOM_BORDER)
                + (pos.on_top_border(grid_size) as u8 * Self::TOP_BORDER)
                + (pos.on_left_border(grid_size) as u8 * Self::LEFT_BORDER)
                + (pos.on_right_border(grid_size) as u8 * Self::RIGHT_BORDER)
                + Self::ALL_EDGES,
        )
    }

    pub fn blank(grid_size: u8, pos: &Point) -> Self {
        Self(
            (pos.on_bottom_border(grid_size) as u8 * Self::BOTTOM_BORDER)
                + (pos.on_top_border(grid_size) as u8 * Self::TOP_BORDER)
                + (pos.on_left_border(grid_size) as u8 * Self::LEFT_BORDER)
                + (pos.on_right_border(grid_size) as u8 * Self::RIGHT_BORDER),
        )
    }

    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }

    /// Check if the cell is on the bottom border
    pub fn on_bottom(&self) -> bool {
        self.0 & Self::BOTTOM_BORDER != 0
    }
    /// Check if the cell is on the  leftborder
    pub fn on_left(&self) -> bool {
        self.0 & Self::LEFT_BORDER != 0
    }
    /// Check if the cell is on the right border
    pub fn on_right(&self) -> bool {
        self.0 & Self::RIGHT_BORDER != 0
    }
    /// Check if the cell is on the top border
    pub fn on_top(&self) -> bool {
        self.0 & Self::TOP_BORDER != 0
    }

    /// Check if the cell has its bottom edge visible
    pub fn has_bottom_edge(&self) -> bool {
        self.0 & Self::BOTTOM_EDGE != 0
    }

    /// Check if the cell has its left edge visible
    pub fn has_left_edge(&self) -> bool {
        self.0 & Self::LEFT_EDGE != 0
    }

    /// Check if the cell has its right edge visible
    pub fn has_right_edge(&self) -> bool {
        self.0 & Self::RIGHT_EDGE != 0
    }

    /// Check if the cell has its top edge visible
    pub fn has_top_edge(&self) -> bool {
        self.0 & Self::TOP_EDGE != 0
    }

    /// Check if the cell can have a specified edge removed.  If the cell lays
    /// on a border then that edge cannot be removed.
    fn can_modify_edge(&self, side: &Edge) -> bool {
        // check if the cell is on the edge to be removed,
        // do not allow the edge to be removed if it is on
        match side {
            Edge::N => self.0 & Self::TOP_BORDER != 0,
            Edge::W => self.0 & Self::LEFT_BORDER != 0,
            Edge::S => self.0 & Self::BOTTOM_BORDER != 0,
            Edge::E => self.0 & Self::RIGHT_BORDER != 0,
        }
    }

    /// Remove a specified edge from the cell, if the edge does not lay on an
    /// outer border.
    pub fn remove_edge(&mut self, side: &Edge) -> bool {
        if self.can_modify_edge(side) {
            let val = side.value();
            // check if the cell has the specified edge
            if self.0 & val != 0 {
                // and remove it
                self.0 -= val;
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
            if self.0 & val == 0 {
                self.0 += val;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
