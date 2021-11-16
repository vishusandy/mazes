pub mod capacity;
pub mod cardinal;
pub mod index;
pub mod row_size;
pub mod visit;

use crate::trans::major::Major;
pub use capacity::Capacity;
pub use cardinal::{Cardinal, CardinalIter, CardinalRev};
pub use index::Index;
use parse_display::{Display, FromStr};
pub use row_size::{ColSize, RowSize};
pub use visit::Visit;

// todo: seal numerical
pub trait Numerical {
    fn num(&self) -> usize;
}
impl Numerical for usize {
    fn num(&self) -> usize {
        *self
    }
}
impl Numerical for u32 {
    fn num(&self) -> usize {
        *self as usize
    }
}

#[derive(Clone, Copy, Debug, Display, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[display("(x={x},y={y})")]
pub struct Coord {
    x: Index,
    y: Index,
}
impl Coord {
    pub fn new(x: Index, y: Index) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> Index {
        self.x
    }
    pub fn y(&self) -> Index {
        self.y
    }
    pub fn row(&self) -> RowSize {
        self.y.into()
    }
    pub fn col(&self) -> ColSize {
        self.x.into()
    }
    pub fn tuple(&self) -> (RowSize, ColSize) {
        (self.x.into(), self.y.into())
    }
    pub fn float_tuple(&self) -> (f32, f32) {
        (self.x.into(), self.y.into())
    }
    pub fn unsigned_tuple(&self) -> (u32, u32) {
        (self.x.into(), self.y.into())
    }
    pub fn signed_tuple(&self) -> (i32, i32) {
        (self.x.into(), self.y.into())
    }
    pub fn id(&self, row_size: RowSize) -> Index {
        Index::from(self.y.mul(row_size)) + self.x
    }
}

#[derive(Debug, Display, FromStr, Clone, Copy, PartialEq, Eq)]
pub enum Horizontal {
    #[display("East")]
    E,
    #[display("West")]
    W,
}
impl Horizontal {
    pub fn x(&self, v: Visit, s: RowSize, x: fn(Visit, RowSize) -> Index) -> Index {
        match self {
            Self::E => x(v, s),
            Self::W => (s.minus(x(v, s)) - 1).into(),
        }
    }
}
impl From<Horizontal> for Cardinal {
    fn from(from: Horizontal) -> Self {
        match from {
            Horizontal::W => Cardinal::W,
            Horizontal::E => Cardinal::E,
        }
    }
}
#[derive(Debug, Display, FromStr, Clone, Copy, PartialEq, Eq)]
pub enum Vertical {
    #[display("South")]
    S,
    #[display("North")]
    N,
}
impl Vertical {
    pub fn y(&self, v: Visit, s: RowSize, y: fn(Visit, RowSize) -> Index) -> Index {
        match self {
            Self::S => y(v, s),
            Self::N => (s.minus(y(v, s)) - 1).into(),
        }
    }
}
impl From<Vertical> for Cardinal {
    fn from(from: Vertical) -> Self {
        match from {
            Vertical::N => Cardinal::N,
            Vertical::S => Cardinal::S,
        }
    }
}

#[derive(Debug, Display, FromStr, Clone, Copy, PartialEq, Eq)]
pub enum Ordinal {
    #[display("Northwest")]
    Nw,
    #[display("Northeast")]
    Ne,
    #[display("Southeast")]
    Se,
    #[display("Southwest")]
    Sw,
}
impl Ordinal {
    pub fn iter() -> OrdinalIter {
        OrdinalIter::default()
    }
    pub fn inc_cw(&self) -> Self {
        match self {
            Self::Nw => Self::Ne,
            Self::Ne => Self::Se,
            Self::Se => Self::Sw,
            Self::Sw => Self::Nw,
        }
    }
    pub fn inc_ccw(&self) -> Self {
        match self {
            Self::Sw => Self::Se,
            Self::Se => Self::Ne,
            Self::Ne => Self::Nw,
            Self::Nw => Self::Sw,
        }
    }
    /// Returns the horizontal component as a specific cardinal direction
    pub fn side_x(&self) -> Horizontal {
        match self {
            // NO this is wrong: Nw becomes Horizontal::E because it represents eastward movement, not the starting point
            Self::Nw | Self::Sw => Horizontal::W,
            Self::Ne | Self::Se => Horizontal::E,
        }
    }
    /// Returns the vertical component as a specific cardinal direction
    pub fn side_y(&self) -> Vertical {
        match self {
            Self::Nw | Self::Ne => Vertical::N,
            Self::Sw | Self::Se => Vertical::S,
        }
    }
    /// Find what direction we are moving (horizontally)
    pub fn direction_x(&self) -> Horizontal {
        match self {
            // NO this is wrong: Nw becomes Horizontal::E because it represents eastward movement, not the starting point
            Self::Nw | Self::Sw => Horizontal::E,
            Self::Ne | Self::Se => Horizontal::W,
        }
    }
    /// Find what direction we are moving (vertically)
    pub fn direction_y(&self) -> Vertical {
        match self {
            Self::Nw | Self::Ne => Vertical::S,
            Self::Sw | Self::Se => Vertical::N,
        }
    }
    pub fn major_order_coord<M: Major>(&self, v: Visit, s: RowSize, _major: M) -> Coord {
        let x = self.direction_x().x(v, s, M::op_x());
        let y = self.direction_y().y(v, s, M::op_y());
        Coord::new(x, y)
    }
    pub fn major_order_index<M: Major>(&self, v: Visit, s: RowSize, major: M) -> Index {
        self.major_order_coord(v, s, major).id(s)
    }
    pub fn map<T, Nw, Ne, Se, Sw>(&self, nw: Nw, ne: Ne, se: Se, sw: Sw) -> T
    where
        Nw: Fn() -> T,
        Ne: Fn() -> T,
        Se: Fn() -> T,
        Sw: Fn() -> T,
    {
        match self {
            Self::Nw => nw(),
            Self::Ne => ne(),
            Self::Se => se(),
            Self::Sw => sw(),
        }
    }
}
impl From<&str> for Ordinal {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();
        match &*s {
            "nw" => Self::Nw,
            "ne" => Self::Ne,
            "se" => Self::Se,
            "sw" => Self::Sw,
            "northwest" => Self::Nw,
            "northeast" => Self::Ne,
            "southeast" => Self::Se,
            "southwest" => Self::Sw,
            _ => panic!("Invalid string; cannot convert '{}' to Ordinal", s),
        }
    }
}
#[derive(Clone, Debug)]
pub struct OrdinalIter {
    cur: Ordinal,
    start: Ordinal,
    count: u8,
}
impl OrdinalIter {
    pub fn new(start: &Ordinal) -> Self {
        Self {
            cur: *start,
            start: *start,
            count: 0,
        }
    }
    fn inc(&mut self) -> Option<Ordinal> {
        let cur = self.cur;
        if self.count == 4 {
            self.count = 0;
            self.cur = self.start;
            None
        } else {
            self.count += 1;
            self.cur = self.cur.inc_cw();
            Some(cur)
        }
    }
}
impl Default for OrdinalIter {
    fn default() -> Self {
        Self::new(&Ordinal::Nw)
    }
}
impl Iterator for OrdinalIter {
    type Item = Ordinal;
    fn next(&mut self) -> Option<Self::Item> {
        self.inc()
    }
}

#[derive(Display, Debug, Clone)]
pub enum StartAt {
    #[display("StartCorner={0}")]
    Corner(Cardinal),
    #[display("StartId={0}")]
    Id(Index),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trans::major::*;
    #[test]
    fn major_order_coord_nw() {
        assert_eq!(
            Ordinal::Nw.major_order_coord(0.into(), 4.into(), RowMajor),
            Coord::new(0.into(), 0.into())
        );
    }
    #[test]
    fn major_order_coord_ne() {
        assert_eq!(
            Ordinal::Ne.major_order_coord(0.into(), 4.into(), RowMajor),
            Coord::new(3.into(), 0.into())
        );
    }
    #[test]
    fn major_order_coord_se() {
        assert_eq!(
            Ordinal::Se.major_order_coord(0.into(), 4.into(), RowMajor),
            Coord::new(3.into(), 3.into())
        );
    }
    #[test]
    fn major_order_coord_sw() {
        assert_eq!(
            Ordinal::Sw.major_order_coord(0.into(), 4.into(), RowMajor),
            Coord::new(0.into(), 3.into())
        );
    }
}
