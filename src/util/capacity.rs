use crate::util::Numerical;
use crate::util::{Index, RowSize};
use parse_display::Display;
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Display, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Capacity(pub(in crate::util) usize);
impl Capacity {
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn one() -> Self {
        Self(0)
    }
    pub fn plus<T: Numerical>(&self, rhs: T) -> usize {
        self.0 + rhs.num()
    }
    pub fn minus<T: Numerical>(&self, rhs: T) -> usize {
        self.0 - rhs.num()
    }
    pub fn mul<T: Numerical>(&self, rhs: T) -> usize {
        self.0 * rhs.num()
    }
    pub fn div<T: Numerical>(&self, rhs: T) -> usize {
        self.0 / rhs.num()
    }
    pub fn rem<T: Numerical>(&self, rhs: T) -> usize {
        self.0 % rhs.num()
    }
    #[cfg(target_pointer_width = "32")]
    pub fn pow<T: Numerical>(&self, rhs: u32) -> usize {
        (self.0 as u32).pow(rhs) as usize
    }
    #[cfg(not(target_pointer_width = "32"))]
    pub fn pow<T: Numerical>(&self, rhs: u32) -> usize {
        self.0.pow(rhs)
    }
    pub fn lt<T: Numerical>(&self, rhs: T) -> bool {
        self.0 < rhs.num()
    }
    pub fn le<T: Numerical>(&self, rhs: T) -> bool {
        self.0 <= rhs.num()
    }
    pub fn gt<T: Numerical>(&self, rhs: T) -> bool {
        self.0 > rhs.num()
    }
    pub fn ge<T: Numerical>(&self, rhs: T) -> bool {
        self.0 >= rhs.num()
    }
    #[allow(clippy::should_implement_trait)]
    pub fn eq<T: Numerical>(&self, rhs: T) -> bool {
        self.0 == rhs.num()
    }
    #[allow(clippy::should_implement_trait)]
    pub fn cmp<T: Numerical>(&self, rhs: T) -> Ordering {
        self.0.cmp(&rhs.num())
    }
}
impl Add for Capacity {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self.0 + other.0).into()
    }
}
impl AddAssign for Capacity {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}
impl Sub for Capacity {
    type Output = Index;
    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0).into()
    }
}
impl SubAssign for Capacity {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}
impl Mul for Capacity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        (self.0 * rhs.0).into()
    }
}
impl MulAssign for Capacity {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}
impl Div for Capacity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        (self.0 / rhs.0).into()
    }
}
impl DivAssign for Capacity {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
impl Rem for Capacity {
    type Output = usize;
    fn rem(self, modulus: Self) -> Self::Output {
        self.0 % modulus.0
    }
}
impl RemAssign<usize> for Capacity {
    fn rem_assign(&mut self, rhs: usize) {
        self.0 %= rhs
    }
}
impl From<usize> for Capacity {
    fn from(from: usize) -> Self {
        Self(from)
    }
}
impl From<Capacity> for usize {
    fn from(from: Capacity) -> Self {
        from.0
    }
}
impl From<Index> for Capacity {
    fn from(from: Index) -> Self {
        from.0.into()
    }
}
impl From<RowSize> for Capacity {
    fn from(from: RowSize) -> Self {
        from.0.into()
    }
}
impl Numerical for Capacity {
    fn num(&self) -> usize {
        self.0
    }
}
impl Deref for Capacity {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
