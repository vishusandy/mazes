
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
