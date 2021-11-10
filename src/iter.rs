use crate::maze::{CardinalGrid, Grid};
use crate::trans::*;
use crate::util::*;
use std::marker::PhantomData;

pub struct Iter<'g, G: Grid, T: Transform> {
    pub(in crate) grid: &'g G,
    pub(in crate) count: Visit,
    pub(in crate) _phantom: std::marker::PhantomData<T>,
}
impl<'g, G: Grid, T: Transform> Iter<'g, G, T> {
    pub(in crate) fn new(grid: &'g G) -> Iter<'g, G, T> {
        Iter {
            count: Visit::zero(),
            grid,
            _phantom: PhantomData,
        }
    }
    pub(in crate) fn nest<U: Transform>(grid: &'g G) -> Iter<'g, G, NestedIter<T, U>> {
        Iter {
            count: Visit::zero(),
            grid,
            _phantom: PhantomData,
        }
    }
    pub fn iter(&'g self) -> Iter<'g, G, NestedIter<T, Ident>> {
        Self::nest(self.grid)
    }
    pub fn reverse(&'g self) -> Iter<'g, G, NestedIter<T, Rev>> {
        Self::nest(self.grid)
    }
    pub fn nw(&'g self) -> Iter<'g, G, NestedIter<T, Nw>>
    where
        G: Grid + CardinalGrid,
    {
        Self::nest(self.grid)
    }
    pub fn ne(&'g self) -> Iter<'g, G, NestedIter<T, Ne>>
    where
        G: Grid + CardinalGrid,
    {
        Self::nest(self.grid)
    }
    pub fn se(&'g self) -> Iter<'g, G, NestedIter<T, Se>>
    where
        G: Grid + CardinalGrid,
    {
        Self::nest(self.grid)
    }
    pub fn sw(&'g self) -> Iter<'g, G, NestedIter<T, Sw>>
    where
        G: Grid + CardinalGrid,
    {
        Self::nest(self.grid)
    }
}
impl<'g, G: Grid, T: Transform> Iterator for Iter<'g, G, T> {
    type Item = &'g G::C;
    fn next(&mut self) -> Option<Self::Item> {
        if *self.count == *self.grid.capacity() {
            self.count = Visit::zero();
            None
        } else {
            let count = *self.count;
            self.count = self.count.plus(1).into();
            let id = T::transform(count, self.grid);
            Some(self.grid.lookup(id.into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::sq::SqGrid;
    use crate::maze::{Cell, Grid, GridProps};
    #[test]
    fn iters_iter() {
        let grid = SqGrid::setup(4);
        let ids = grid.iter().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_rev() {
        let grid = SqGrid::setup(4);
        let ids = grid.reverse().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_nested_rev() {
        let grid = SqGrid::setup(4);
        let ids = grid.iter().reverse().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(&ids, expected);
        let ids = grid.reverse().iter().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_nw() {
        let grid = SqGrid::setup(4);
        let ids = grid.nw().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_ne() {
        let grid = SqGrid::setup(4);
        let ids = grid.ne().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_se() {
        let grid = SqGrid::setup(4);
        let ids = grid.se().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_se_rev() {
        let grid = SqGrid::setup(4);
        let expected = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let ids = grid.se().reverse().map(|c| *c.id()).collect::<Vec<_>>();
        assert_eq!(&ids, expected);
        let ids = grid.reverse().se().map(|c| *c.id()).collect::<Vec<_>>();
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_sw() {
        let grid = SqGrid::setup(4);
        let ids = grid.sw().map(|c| *c.id()).collect::<Vec<_>>();
        let expected = &[12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3];
        assert_eq!(&ids, expected);
    }
}
