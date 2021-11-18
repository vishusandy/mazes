pub mod capacity;
pub mod cardinal;
pub mod dist;
pub mod index;
pub mod ordinal;
pub mod path;
pub mod row_size;
pub mod visit;

pub use capacity::Capacity;
pub use cardinal::{Cardinal, CardinalIter, CardinalRev};
pub use index::Index;
pub use ordinal::{Ordinal, OrdinalIter};
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
