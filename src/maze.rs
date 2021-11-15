pub mod sq;

use crate::error::*;
use crate::iter::*;
use crate::trans::major::{Major, RowMajor};
use crate::trans::*;
use crate::util::*;
use std::cell::RefCell;
pub trait Grid: GridProps {
    fn lookup(&self, id: Index) -> &<Self as GridProps>::C {
        &self.cells()[*id]
    }
    fn try_lookup(&self, id: Index) -> Result<&<Self as GridProps>::C, OutOfBoundsError> {
        if *id < self.cells().len() {
            Ok(&self.cells()[*id])
        } else {
            Err(OutOfBoundsError::new(id))
        }
    }
    fn iter(&self) -> Iter<Self, Ident>
    where
        Self: Sized + Grid,
    {
        Iter::new(self, Ident)
    }
    fn reverse(&self) -> Iter<Self, Rev>
    where
        Self: Sized + Grid,
    {
        Iter::new(self, Rev)
    }
    fn rand<'g, 'r, R: rand::Rng + Sized>(&'g self, rng: &'r mut R) -> Rand<'g, 'r, R, Self>
    where
        Self: Sized + Grid,
    {
        Rand::new(self, rng)
    }
    fn first(&self) -> &<Self as GridProps>::C {
        &self.cells()[0]
    }
    fn last(&self) -> &<Self as GridProps>::C {
        &self.cells()[self.capacity().minus(1)]
    }
    fn nth(&self, n: Index) -> &<Self as GridProps>::C {
        &self.cells()[*n]
    }
    fn get(&self, id: Index) -> Option<&<Self as GridProps>::C> {
        if *id < *self.capacity() {
            Some(&self.cells()[*id])
        } else {
            None
        }
    }
}

/// All cells in a grid must implement `Cell`.  This allows linking and navigation between cells.
///
/// `Cell` is intentionally very simple to allow more flexibility in regards to grid types.
pub trait Cell {
    fn id(&self) -> Index;
    fn neighbor_ids(&self) -> &[Index];
    fn link<T: Grid>(&mut self, with: Index, grid: &T) -> Result<(), OutOfBoundsError>;
    // fn links(&self) -> &[RefCell<Index>];
    fn links(&self) -> &RefCell<Vec<Index>>;
}

/// Any `Grid` type implementing `CoordLookup` can use `Coord` to lookup a cell's `Index`
pub trait CoordLookup: Grid {
    fn get_id(&self, coord: &Coord) -> Index;
    fn try_get_id(&self, coord: &Coord) -> Result<Index, OutOfBoundsCoordError>;
    fn get_coords(&self, id: Index) -> Coord;
    fn try_get_coords(&self, id: Index) -> Result<Coord, OutOfBoundsError>;
}

/// Methods that rely on having access to the struct's fields.
/// The `Grid` trait provides additional methods but is intended
/// to be used generically and thus cannot access its data.
pub trait GridProps {
    type C: Cell;
    fn setup(size: usize) -> Self;
    fn capacity(&self) -> Capacity;
    fn cells(&self) -> &Vec<Self::C>;
}

/// Describes grids that can be navigated using cardinal directions.
/// This allows starting iteration at an ordinal direction.
pub trait CardinalGrid: Grid {
    fn row_size(&self) -> RowSize;
    fn dimensions(&self) -> (RowSize, ColSize);
    fn major_order_fn<M: Major>() -> fn(Visit, RowSize, ColSize, M) -> Index {
        Self::calc_major_order
    }
    fn calc_major_order<M: Major>(id: Visit, rows: RowSize, cols: ColSize, order: M) -> Index {
        Ordinal::Nw.major_order_index(id.into(), rows, order)
    }
    fn nw(&self) -> Iter<Self, Nw<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn();
        Iter::new(self, Nw::new(rows, cols, m))
    }
    fn ne(&self) -> Iter<Self, Ne<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn();
        Iter::new(self, Ne::new(rows, cols, m))
    }
    fn se(&self) -> Iter<Self, Se<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn();
        Iter::new(self, Se::new(rows, cols, m))
    }
    fn sw(&self) -> Iter<Self, Sw<RowMajor>>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        let (rows, cols) = self.dimensions();
        let m = Self::major_order_fn();
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
    fn calc_dir(&self, id: Index, dir: Cardinal) -> Option<Index> {
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
            Ordinal::Ne => self.row_size().minus(1),
            Ordinal::Se => self.capacity().minus(1),
            Ordinal::Sw => self.capacity().minus(self.row_size()),
        }
        .into()
    }
}
