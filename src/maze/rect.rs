use crate::iter::*;
use crate::maze::Grid;
use crate::trans::major::{Major, RowMajor};
use crate::trans::*;
use crate::util::*;

/// Describes grids that can be navigated using cardinal directions.
/// This allows starting iteration at an ordinal direction.
pub trait CardinalGrid: Grid {
    fn row_size(&self) -> RowSize;
    fn dimensions(&self) -> (RowSize, ColSize);
    fn major_order_fn<M: Major>(ordinal: Ordinal) -> fn(Visit, RowSize, ColSize, M) -> Index {
        match ordinal {
            Ordinal::Nw => Self::calc_major_order_nw,
            Ordinal::Ne => Self::calc_major_order_ne,
            Ordinal::Se => Self::calc_major_order_se,
            Ordinal::Sw => Self::calc_major_order_sw,
        }
    }
    fn calc_major_order_nw<M: Major>(id: Visit, rows: RowSize, _cols: ColSize, order: M) -> Index {
        Ordinal::Nw.major_order_index(id, rows, order)
    }
    fn calc_major_order_ne<M: Major>(id: Visit, rows: RowSize, _cols: ColSize, order: M) -> Index {
        Ordinal::Ne.major_order_index(id, rows, order)
    }
    fn calc_major_order_se<M: Major>(id: Visit, rows: RowSize, _cols: ColSize, order: M) -> Index {
        Ordinal::Se.major_order_index(id, rows, order)
    }
    fn calc_major_order_sw<M: Major>(id: Visit, rows: RowSize, _cols: ColSize, order: M) -> Index {
        Ordinal::Sw.major_order_index(id, rows, order)
    }
    fn nw(&self) -> Iter<Self, Nw<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn(Ordinal::Nw);
        Iter::new(self, Nw::new(rows, cols, m))
    }
    fn ne(&self) -> Iter<Self, Ne<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn(Ordinal::Ne);
        Iter::new(self, Ne::new(rows, cols, m))
    }
    fn se(&self) -> Iter<Self, Se<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn(Ordinal::Se);
        Iter::new(self, Se::new(rows, cols, m))
    }
    fn sw(&self) -> Iter<Self, Sw<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn(Ordinal::Sw);
        Iter::new(self, Sw::new(rows, cols, m))
    }
    fn has_boundary(&self, id: Index, dir: Cardinal) -> bool {
        match dir {
            Cardinal::N => self.has_boundary_north(id),
            Cardinal::E => self.has_boundary_east(id),
            Cardinal::S => self.has_boundary_south(id),
            Cardinal::W => self.has_boundary_west(id),
        }
    }
    fn has_boundary_north(&self, id: Index) -> bool {
        id.lt(self.row_size())
    }
    fn has_boundary_east(&self, id: Index) -> bool {
        id.rem(self.row_size()) == *self.row_size() - 1
    }
    fn has_boundary_south(&self, id: Index) -> bool {
        id.div(self.row_size()) == *self.row_size() - 1
    }
    fn has_boundary_west(&self, id: Index) -> bool {
        id.rem(self.row_size()) == 0
    }
    fn find_boundary(&self, id: Index) -> Option<Cardinal> {
        Cardinal::iter().find(|d| self.has_boundary(id, *d))
    }
    fn dir_from(&self, from: Index, to: Index) -> Option<Cardinal> {
        for d in Cardinal::iter() {
            if matches!(self.neighbor(from, &d), Some(n) if n == to) {
                return Some(d);
            }
        }
        None
    }
    fn neighbor(&self, id: Index, d: &Cardinal) -> Option<Index> {
        self.calc_dir(id, d)
    }
    fn calc_dir(&self, id: Index, dir: &Cardinal) -> Option<Index> {
        match dir {
            Cardinal::N => self.calc_north(id),
            Cardinal::E => self.calc_east(id),
            Cardinal::S => self.calc_south(id),
            Cardinal::W => self.calc_west(id),
        }
    }
    fn calc_north(&self, id: Index) -> Option<Index> {
        match self.has_boundary_north(id) {
            true => None,
            false => Some(id.minus(self.row_size()).into()),
        }
    }
    fn calc_east(&self, id: Index) -> Option<Index> {
        match self.has_boundary_east(id) {
            true => None,
            false => Some(id.plus(1usize).into()),
        }
    }
    fn calc_south(&self, id: Index) -> Option<Index> {
        match self.has_boundary_south(id) {
            true => None,
            false => Some(id.plus(self.row_size()).into()),
        }
    }
    fn calc_west(&self, id: Index) -> Option<Index> {
        match self.has_boundary_west(id) {
            true => None,
            false => Some(id.minus(1usize).into()),
        }
    }
    fn corner_id(&self, dir: Ordinal) -> Index {
        match dir {
            Ordinal::Nw => 0,
            Ordinal::Ne => self.row_size().minus(1usize),
            Ordinal::Se => self.capacity().minus(1usize),
            Ordinal::Sw => self.capacity().minus(self.row_size()),
        }
        .into()
    }
}
