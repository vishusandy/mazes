enum Sides {
    N,
    W,
    S,
    E,
}

struct MazeCell(u8);

impl MazeCell {
    const BOTTOM_BORDER: u8 = 1;
    const LEFT_BORDER: u8 = 2;
    const RIGHT_BORDER: u8 = 4;
    const TOP_BORDER: u8 = 8;
    fn on_bottom(&self) -> bool {
        self.0 & Self::BOTTOM_BORDER != 0
    }
    fn on_left(&self) -> bool {
        self.0 & Self::LEFT_BORDER != 0
    }
    fn on_right(&self) -> bool {
        self.0 & Self::RIGHT_BORDER != 0
    }
    fn on_top(&self) -> bool {
        self.0 & Self::TOP_BORDER != 0
    }
    const BOTTOM_EDGE: u8 = 16; // 32 64 128
    const LEFT_EDGE: u8 = 32;
    const RIGHT_EDGE: u8 = 64;
    const TOP_EDGE: u8 = 128;
    fn has_bottom_edge(&self) -> bool {
        self.0 & Self::BOTTOM_EDGE != 0
    }
    fn has_left_edge(&self) -> bool {
        self.0 & Self::LEFT_EDGE != 0
    }
    fn has_right_edge(&self) -> bool {
        self.0 & Self::RIGHT_EDGE != 0
    }
    fn has_top_edge(&self) -> bool {
        self.0 & Self::TOP_EDGE != 0
    }

    fn can_remove_edge(&self, side: Sides) -> bool {
        match side {
            Sides::N => false,
            Sides::W => false,
            Sides::S => false,
            Sides::E => false,
        }
    }
}

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
/// row is filled.
struct MazeGrid {
    width: u8,
    cells: Vec<MazeCell>,
}

impl MazeGrid {
    fn carve(&mut self, side: Sides) {}
}
