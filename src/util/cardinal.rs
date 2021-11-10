pub use crate::util::capacity::Capacity;
pub use crate::util::index::Index;
pub use crate::util::row_size::RowSize;
use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, Clone, Copy)]
pub enum Cardinal {
    #[display("North")]
    N,
    #[display("East")]
    E,
    #[display("South")]
    S,
    #[display("West")]
    W,
}
impl Cardinal {
    pub fn iter(&self) -> CardinalIter {
        CardinalIter::new(self)
    }
    pub fn clockwise(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Self::N => Self::W,
            Self::W => Self::S,
            Self::S => Self::E,
            Self::E => Self::N,
        }
    }
    pub fn char(&self) -> char {
        match self {
            Self::N => 'N',
            Self::E => 'E',
            Self::S => 'S',
            Self::W => 'W',
        }
    }
    pub fn string(&self) -> &str {
        match self {
            Self::N => "North",
            Self::E => "East",
            Self::S => "South",
            Self::W => "West",
        }
    }
    pub fn clockwise_start() -> Self {
        Self::N
    }
    pub fn counter_clockwise_start() -> Self {
        Self::W
    }
    pub fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let dirs = &['N', 'E', 'S', 'W'];
        Cardinal::from(dirs[rng.gen_range(0usize..4usize)])
    }
}
impl From<u8> for Cardinal {
    fn from(i: u8) -> Self {
        match i {
            0 => Self::N,
            1 => Self::E,
            2 => Self::S,
            3 => Self::W,
            _ => panic!("Invalid cardinal index; cannot convert '{}' to Cardinal", i),
        }
    }
}
impl From<char> for Cardinal {
    fn from(c: char) -> Self {
        match c {
            'N' => Self::N,
            'E' => Self::E,
            'S' => Self::S,
            'W' => Self::W,
            'n' => Self::N,
            'e' => Self::E,
            's' => Self::S,
            'w' => Self::W,
            _ => panic!("Invalid character; cannot convert '{}' to Cardinal", c),
        }
    }
}
impl From<&str> for Cardinal {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();
        match &*s {
            "n" => Self::N,
            "e" => Self::E,
            "s" => Self::S,
            "w" => Self::W,
            "north" => Self::N,
            "east" => Self::E,
            "south" => Self::S,
            "west" => Self::W,
            _ => panic!("Invalid string; cannot convert '{}' to Cardinal", s),
        }
    }
}
impl std::ops::Neg for Cardinal {
    type Output = Self;
    /// Returns the opposite direction on the same axis (north => south, east => west)
    fn neg(self) -> Self::Output {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::E => Self::W,
            Self::W => Self::E,
        }
    }
}
impl IntoIterator for Cardinal {
    type Item = Cardinal;
    type IntoIter = CardinalIter;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Debug)]
pub struct CardinalIter {
    cur: Cardinal,
    start: Cardinal,
    count: u8,
}
impl CardinalIter {
    pub fn new(start: &Cardinal) -> Self {
        Self {
            cur: *start,
            start: *start,
            count: 0,
        }
    }
    fn inc(&mut self) -> Option<Cardinal> {
        let cur = self.cur;
        if self.count == 4 {
            self.count = 0;
            self.cur = self.start;
            None
        } else {
            self.count += 1;
            self.cur = self.cur.clockwise();
            Some(cur)
        }
    }
    pub fn iter() -> Self {
        Self::default()
    }
}
impl Default for CardinalIter {
    fn default() -> Self {
        Self::new(&Cardinal::clockwise_start())
    }
}
impl Iterator for CardinalIter {
    type Item = Cardinal;
    fn next(&mut self) -> Option<Self::Item> {
        self.inc()
    }
}

#[derive(Clone, Debug)]
pub struct CardinalRev {
    cur: Cardinal,
    start: Cardinal,
    count: u8,
}
impl CardinalRev {
    pub fn new(start: &Cardinal) -> Self {
        Self {
            cur: *start,
            start: *start,
            count: 0,
        }
    }
    fn inc(&mut self) -> Option<Cardinal> {
        let cur = self.cur;
        if self.count == 4 {
            self.count = 0;
            self.cur = self.start;
            None
        } else {
            self.count += 1;
            self.cur = self.cur.counter_clockwise();
            Some(cur)
        }
    }
    pub fn iter() -> Self {
        Self::default()
    }
}
impl Default for CardinalRev {
    fn default() -> Self {
        Self::new(&Cardinal::counter_clockwise_start())
    }
}
impl Iterator for CardinalRev {
    type Item = Cardinal;
    fn next(&mut self) -> Option<Self::Item> {
        self.inc()
    }
}
