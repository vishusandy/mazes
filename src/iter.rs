use crate::maze::{CardinalGrid, Cell, Grid};
use crate::trans::major::RowMajor;
use crate::trans::*;
use crate::util::*;
use rand::Rng;

pub struct Iter<'g, G: Grid, T: Transform> {
    pub(in crate) grid: &'g G,
    pub(in crate) count: Visit,
    pub(in crate) t: T,
    // pub(in crate) _phantom: std::marker::PhantomData<T>,
}
impl<'g, G: Grid, T: Transform> Iter<'g, G, T> {
    pub(in crate) fn new(grid: &'g G, t: T) -> Iter<'g, G, T> {
        Iter {
            count: Visit::zero(),
            grid,
            t,
            // _phantom: PhantomData,
        }
    }
    pub(in crate) fn nest<U: Transform>(self, grid: &'g G, u: U) -> Iter<'g, G, NestedIter<T, U>> {
        Iter {
            count: Visit::zero(),
            grid,
            t: NestedIter::new(self.t, u), // _phantom: PhantomData,
        }
    }
    pub fn iter(self) -> Iter<'g, G, NestedIter<T, Ident>> {
        let g = self.grid;
        self.nest(g, Ident)
    }
    pub fn reverse(self) -> Iter<'g, G, NestedIter<T, Rev>> {
        let g = self.grid;
        self.nest(g, Rev)
    }
    pub fn nw(self) -> Iter<'g, G, NestedIter<T, Nw<RowMajor>>>
    where
        G: Grid + CardinalGrid,
    {
        let g = self.grid;
        let (rows, cols) = g.dimensions();
        self.nest(g, Nw::new(rows, cols, G::major_order_fn()))
    }
    pub fn ne(self) -> Iter<'g, G, NestedIter<T, Ne<RowMajor>>>
    where
        G: Grid + CardinalGrid,
    {
        let g = self.grid;
        let (rows, cols) = g.dimensions();
        self.nest(g, Ne::new(rows, cols, G::major_order_fn()))
    }
    pub fn se(self) -> Iter<'g, G, NestedIter<T, Se<RowMajor>>>
    where
        G: Grid + CardinalGrid,
    {
        let g = self.grid;
        let (rows, cols) = g.dimensions();
        self.nest(g, Se::new(rows, cols, G::major_order_fn()))
    }
    pub fn sw(self) -> Iter<'g, G, NestedIter<T, Sw<RowMajor>>>
    where
        G: Grid + CardinalGrid,
    {
        let g = self.grid;
        let (rows, cols) = g.dimensions();
        self.nest(g, Sw::new(rows, cols, G::major_order_fn()))
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
            let id = self.t.transform(count, self.grid);
            Some(self.grid.lookup(id.into()))
        }
    }
}

pub struct Rand<'g, 'r, R: Rng + ?Sized, G: Grid> {
    rng: &'r mut R,
    count: Visit,
    grid: &'g G,
    shuffle: Vec<Index>,
}
impl<'g, 'r, R: Rng + ?Sized, G: Grid> Rand<'g, 'r, R, G> {
    pub(in crate) fn new(grid: &'g G, rng: &'r mut R) -> Self {
        use rand::seq::SliceRandom;
        let mut shuffle = (0..*grid.capacity())
            .map(|i| i.into())
            .collect::<Vec<Index>>();
        shuffle.shuffle(rng);
        Self {
            rng,
            count: Visit::zero(),
            grid,
            shuffle,
        }
    }
    pub fn random_id(&mut self) -> Index {
        self.rng.gen_range(0..*self.grid.capacity()).into()
    }
    pub fn random_cell(&mut self) -> &'g G::C {
        self.grid.lookup(self.random_id())
    }
    pub fn id_from_list(&mut self, list: &[Index]) -> Index {
        use rand::seq::SliceRandom;
        *list.choose(self.rng).unwrap()
    }
    pub fn cell_from_list(&mut self, list: &[Index]) -> &'g G::C {
        self.grid.lookup(self.id_from_list(list))
    }
    pub fn random_neighbor_id(&mut self, cell: &'g G::C) -> Index {
        self.id_from_list(&*cell.neighbor_ids())
    }
}
impl<'g, 'r, R: Rng + ?Sized, G: Grid> Iterator for Rand<'g, 'r, R, G> {
    type Item = &'g G::C;
    fn next(&mut self) -> Option<Self::Item> {
        if *self.count == *self.grid.capacity() {
            self.count = Visit::zero();
            None
        } else {
            let count = *self.count;
            self.count = self.count.plus(1).into();
            let id = self.shuffle[count];
            Some(self.grid.lookup(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::sq::SqGrid;
    use crate::*;
    // use crate::maze::{CardinalGrid, Cell, Grid, GridProps};
    use crate::util::Index;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;
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
    const RAND_SEED: u64 = 8080;
    #[test]
    fn iters_random() {
        let mut rng = SplitMix64::seed_from_u64(RAND_SEED);
        let grid = SqGrid::setup(4);
        let ids: Vec<usize> = grid.rand(&mut rng).map(|cell| *cell.id()).collect();
        let expected: &[usize] = &[10, 1, 8, 7, 3, 11, 0, 6, 15, 14, 12, 4, 13, 9, 5, 2];
        assert_eq!(&ids, expected);
    }
    #[test]
    fn iters_random_id() {
        let mut rng = SplitMix64::seed_from_u64(RAND_SEED);
        let grid = SqGrid::setup(4);
        let id = grid.rand(&mut rng).random_id();
        let expected: Index = 9.into();
        assert_eq!(id, expected);
    }
    #[test]
    fn iters_random_id_from_list() {
        let mut rng = SplitMix64::seed_from_u64(RAND_SEED);
        let grid = SqGrid::setup(4);
        let list: &[Index] = &[1.into(), 3.into(), 5.into(), 7.into()];
        let id = grid.rand(&mut rng).id_from_list(list);
        let expected: Index = 7.into();
        assert_eq!(id, expected);
    }
    #[test]
    fn iters_random_neighbor() {
        let mut rng = SplitMix64::seed_from_u64(RAND_SEED);
        let grid = SqGrid::setup(4);
        let mut rand = grid.rand(&mut rng);
        let cell = rand.random_cell();
        let neighbor = rand.random_neighbor_id(cell);
        let expected: Index = 8.into();
        assert_eq!(neighbor, expected);
    }
}
