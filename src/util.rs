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

#[derive(Clone, Copy, Debug, Display, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[display("({x}, {y})")]
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
    pub fn id(&self, row_size: RowSize) -> Index {
        Index::from(self.y.mul(row_size)) + self.x
    }
}

#[derive(Debug, Display, FromStr, Clone, Copy)]
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
#[derive(Debug, Display, FromStr, Clone, Copy)]
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

#[derive(Display, FromStr)]
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
    /// Find what direction we are moving (horizontally)
    pub fn x_axis(&self) -> Horizontal {
        match self {
            // Nw becomes Horizontal::E because it represents eastward movement, not the starting point
            Self::Nw | Self::Sw => Horizontal::E,
            Self::Ne | Self::Se => Horizontal::W,
        }
    }
    /// Find what direction we are moving (vertically)
    pub fn y_axis(&self) -> Vertical {
        match self {
            Self::Nw | Self::Ne => Vertical::S,
            Self::Sw | Self::Se => Vertical::N,
        }
    }
    pub fn major_order_coord<M: Major>(&self, v: Visit, s: RowSize, _major: M) -> Coord {
        let x = self.x_axis().x(v, s, M::op_x());
        let y = self.y_axis().y(v, s, M::op_y());
        Coord::new(x, y)
    }
    pub fn major_order_index<M: Major>(&self, v: Visit, s: RowSize, major: M) -> Index {
        self.major_order_coord(v, s, major).id(s)
    }
}
#[derive(Display)]
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
