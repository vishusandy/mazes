pub mod sq;

use crate::error::*;
use crate::iter::*;
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
        Iter::new(self)
    }
    fn reverse(&self) -> Iter<Self, Rev>
    where
        Self: Sized + Grid,
    {
        Iter::new(self)
    }
    fn nw(&self) -> Iter<Self, Nw>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        Iter::new(self)
    }
    fn ne(&self) -> Iter<Self, Ne>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        Iter::new(self)
    }
    fn se(&self) -> Iter<Self, Se>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        Iter::new(self)
    }
    fn sw(&self) -> Iter<Self, Sw>
    where
        Self: Sized + Grid + CardinalGrid,
    {
        Iter::new(self)
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
}
