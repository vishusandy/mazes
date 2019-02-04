struct Location(u8);
impl Location {
    const BOTTOM: u8 = 1;
    const LEFT: u8 = 2;
    const RIGHT: u8 = 4;
    const TOP: u8 = 8;
    fn is_bottom(&self) -> bool {
        self.0 & Self::BOTTOM != 0
    }
    fn is_left(&self) -> bool {
        self.0 & Self::LEFT != 0
    }
    fn is_right(&self) -> bool {
        self.0 & Self::RIGHT != 0
    }
    fn is_top(&self) -> bool {
        self.0 & Self::TOP != 0
    }
}

/// Indicate if the cell is located on a border, and if so which.
enum CellLocation {
    Top,
    Bottom,
    Left,
    Right,
    Middle,
}
impl CellLocation {
    /// Check if the cell can have a specified border removed.
    /// If the cell is the top cell, you obviously cannot remove
    /// the top border, etc.
    fn remove_side_check(&self, side: Sides) -> bool {
        match side {
            Sides::N => !self.top(),
            Sides::W => !self.left(),
            Sides::S => !self.bottom(),
            Sides::E => !self.right(),
        }
    }
    /// Check if it is on the bottom row.
    fn bottom(&self) -> bool {
        match self {
            CellLocation::Bottom => true,
            CellLocation::Left => false,
            CellLocation::Right => false,
            CellLocation::Top => false,
            _ => false,
        }
    }
    /// Check if it is on the leftmost column.
    fn left(&self) -> bool {
        match self {
            CellLocation::Bottom => false,
            CellLocation::Left => true,
            CellLocation::Right => false,
            CellLocation::Top => false,
            _ => false,
        }
    }
    /// Check if it is on the rightmost column.
    fn right(&self) -> bool {
        match self {
            CellLocation::Bottom => false,
            CellLocation::Left => false,
            CellLocation::Right => true,
            CellLocation::Top => false,
            _ => false,
        }
    }
    /// Check if it is on the top row.
    fn top(&self) -> bool {
        match self {
            CellLocation::Bottom => false,
            CellLocation::Left => false,
            CellLocation::Right => false,
            CellLocation::Top => true,
            _ => false,
        }
    }
}

struct MazeCell {
    location: CellLocation,
    state: CellState,
}

struct CellState {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}
impl CellState {
    fn check(&self, side: Sides) -> bool {
        match side {
            Sides::N => self.top,
            Sides::W => self.left,
            Sides::S => self.bottom,
            Sides::E => self.right,
        }
    }
}

enum Sides {
    N,
    W,
    S,
    E,
}

impl MazeCell {
    fn carve(&mut self, side: Sides) {}
}
