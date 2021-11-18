use crate::trans::major::Major;
use crate::util::{Coord, Horizontal, Index, RowSize, Vertical, Visit};
use parse_display::{Display, FromStr};

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
