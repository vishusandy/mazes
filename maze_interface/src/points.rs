use super::maze_grid::MazeGrid;

/// A (x,y) point in the grid.
// struct Point(u8, u8);
#[derive(Clone, Debug)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    /// Creates a new point without any safety checks.  The point is not
    /// guaranteed to be within the bounds of the grid.
    ///
    /// The x and y parameters describe a point within the actual full grid,
    /// not the compressed representation of the grid.
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
    /// A safe, checked version of new() which will ensure that the point is
    /// within the grid bounds.
    pub fn point(grid: &MazeGrid, x: i16, y: i16) -> Self {
        assert!(x < grid.length() as i16);
        assert!(y < grid.length() as i16);
        Self { x, y }
    }

    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }

    pub fn get(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }

    pub fn on_top_border(&self, grid_size: u8) -> bool {
        self.y == 0
    }

    pub fn on_bottom_border(&self, grid_size: u8) -> bool {
        self.y == (grid_size - 1) as i16
    }

    pub fn on_left_border(&self, grid_size: u8) -> bool {
        self.x == 0
    }

    pub fn on_right_border(&self, grid_size: u8) -> bool {
        self.x == (grid_size - 1) as i16
    }
    pub fn on_any_border(&self, grid_size: u8) -> bool {
        self.on_right_border(grid_size)
            || self.on_left_border(grid_size)
            || self.on_top_border(grid_size)
            || self.on_bottom_border(grid_size)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::ops::Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
