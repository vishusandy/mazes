use crate::util::{Capacity, Numerical, RowSize, Visit};
use parse_display::Display;
use std::cmp::Ordering;
use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Range, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, Display, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Index(pub(in crate::util) usize);
//  todo: impl Add/Sub/Mul/Div/AddAssign/SubAssign/MulAssign/DivAssign/Deref/Rem  Neg/Not??  From<Cell> - calls self.id
impl Index {
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
    pub fn to<R: Into<Index>>(self, to: R) -> Range<Index> {
        Range {
            start: self,
            end: to.into(),
        }
    }
}
impl Numerical for Index {
    fn num(&self) -> usize {
        self.0
    }
}
impl Add for Index {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self.0 + other.0).into()
    }
}
impl AddAssign for Index {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}
impl Sub for Index {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        (self.0 - other.0).into()
    }
}
impl SubAssign for Index {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0
    }
}
impl Mul for Index {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        (self.0 * rhs.0).into()
    }
}
impl MulAssign for Index {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}
impl Div for Index {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        (self.0 / rhs.0).into()
    }
}
impl DivAssign for Index {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}
impl Rem for Index {
    type Output = usize;
    fn rem(self, modulus: Self) -> Self::Output {
        self.0 % modulus.0
    }
}
impl RemAssign<usize> for Index {
    fn rem_assign(&mut self, rhs: usize) {
        self.0 %= rhs
    }
}
impl Deref for Index {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<usize> for Index {
    fn from(from: usize) -> Self {
        Self(from)
    }
}
impl From<Index> for usize {
    fn from(from: Index) -> Self {
        from.0
    }
}
impl From<Capacity> for Index {
    fn from(from: Capacity) -> Self {
        from.0.into()
    }
}
impl From<RowSize> for Index {
    fn from(from: RowSize) -> Self {
        from.0.into()
    }
}
impl From<Visit> for Index {
    fn from(from: Visit) -> Self {
        from.0.into()
    }
}
impl From<Index> for f32 {
    fn from(from: Index) -> Self {
        from.0 as f32
    }
}
impl From<Index> for u32 {
    fn from(from: Index) -> Self {
        from.0 as u32
    }
}
impl From<Index> for i32 {
    fn from(from: Index) -> Self {
        from.0 as i32
    }
}

pub struct InRange<R: Into<Index>>(pub Range<R>);
impl<R: Into<Index>> InRange<R> {
    pub fn into_range(self) -> Range<usize> {
        Range::from(self)
    }
}
impl<R: Into<Index>> From<InRange<R>> for Range<usize> {
    fn from(range: InRange<R>) -> Self {
        Self {
            start: *range.0.start.into(),
            end: *range.0.end.into(),
        }
    }
}

use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
use rand::prelude::*;
#[derive(Clone, Copy, Debug)]
pub struct UniformIndex(UniformInt<usize>);

impl UniformSampler for UniformIndex {
    type X = Index;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformIndex(UniformInt::<usize>::new(low.borrow().0, high.borrow().0))
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformSampler::new(low, high)
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Index(self.0.sample(rng))
    }
}

impl SampleUniform for Index {
    type Sampler = UniformIndex;
}
