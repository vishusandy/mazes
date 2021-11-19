pub mod rect;
pub mod sq;

pub use self::rect::CardinalGrid;
use crate::error::*;
use crate::iter::*;
use crate::trans::*;
use crate::util::dist::Distances;
use crate::util::path::Path;
use crate::util::*;
use std::cell::RefCell;

/// Trait to access fields of the implementing type.  These methods were separated from [`Grid`]
/// to allow more generic use cases.
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
    fn link(&self, a: Index, b: Index) -> Result<(), CellLinkError> {
        let cell_a = self
            .get(a)
            .ok_or_else(|| CellLinkError::new(a, b, "Link failed - `a` could not be retrieved"))?;
        let cell_b = self
            .get(b)
            .ok_or_else(|| CellLinkError::new(a, b, "Link failed - `b` could not be retrieved"))?;
        cell_a.unchecked_link(b);
        cell_b.unchecked_link(a);
        Ok(())
    }
    fn unlink(&self, a: Index, b: Index) -> Result<(), CellLinkError> {
        let cell_a = self
            .get(a)
            .ok_or_else(|| CellLinkError::new(a, b, "Unlink failed - `a` not found"))?;
        let cell_b = self
            .get(b)
            .ok_or_else(|| CellLinkError::new(a, b, "Unlink failed - `b` not found"))?;
        cell_a.unchecked_unlink(b);
        cell_b.unchecked_unlink(a);
        Ok(())
    }
    /// Produces an [`Iter`] to iterate the grid using the [`Ident`] transform, which does
    /// not change iteration order while still allowing [`Iter`] to be generic over `T: Transform`.
    fn iter(&self) -> Iter<Self, Ident>
    where
        Self: Sized + Grid,
    {
        Iter::new(self, Ident)
    }
    /// Produces a [`Rev`] iterator to reverse the direction of iteration.
    fn reverse(&self) -> Iter<Self, Rev>
    where
        Self: Sized + Grid,
    {
        Iter::new(self, Rev)
    }
    /// Produces a [`Rand`] iterator to randomly iterate a grid.
    fn rand<'g, 'r, R: rand::Rng + Sized>(&'g self, rng: &'r mut R) -> Rand<'g, 'r, R, Self>
    where
        Self: Sized + Grid,
    {
        Rand::new(self, rng)
    }
    /// Returns the first cell stored in the grid.
    fn first(&self) -> &<Self as GridProps>::C {
        &self.cells()[0]
    }
    /// Returns the last cell stored in the grid.
    fn last(&self) -> &<Self as GridProps>::C {
        &self.cells()[self.capacity().minus(1usize)]
    }
    /// Returns the nth cell in a grid.
    fn nth(&self, n: Index) -> &<Self as GridProps>::C {
        &self.cells()[*n]
    }
    /// Attempts to get a given grid cell by `id`.
    fn get(&self, id: Index) -> Option<&<Self as GridProps>::C> {
        if *id < *self.capacity() {
            Some(&self.cells()[*id])
        } else {
            None
        }
    }
    ///
    fn distances(&self, start: Index) -> Distances<'_, Self>
    where
        Self: Sized,
    {
        let mut dist = Distances::new(self, start);
        let mut frontier = vec![start];
        while !frontier.is_empty() {
            let mut new_frontier: Vec<Index> = Vec::new();
            for cell in frontier.iter().filter_map(|id| self.get(*id)) {
                for link in cell.links().borrow().iter() {
                    if !dist.has_entry(link) {
                        let d = dist[cell.id()];
                        dist.set(*link, d + 1);
                        new_frontier.push(*link);
                    }
                }
            }
            frontier = new_frontier;
        }
        dist
    }
    fn shortest_path(&self, start: Index, end: Index) -> Path<'_, Self>
    where
        Self: Sized,
    {
        self.distances(start).shortest_path(end)
    }
    fn longest_path(&self, start: Index) -> Path<'_, Self>
    where
        Self: Sized,
    {
        let dist = self.distances(start);
        let (max, _) = dist.max_dist();
        let dist2 = self.distances(max);
        let (end, _) = dist2.max_dist();
        dist2.shortest_path(end)
    }
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

/// All cells in a grid must implement `Cell`.  This allows linking and navigation between cells.
///
/// `Cell` is intentionally very simple to allow more flexibility in regards to grid types.
pub trait Cell {
    fn id(&self) -> Index;
    /// List cells that are near the current cell, without regards to whether they are linked.
    fn neighbor_ids(&self) -> &[Index];
    /// Link a cell with another cell
    // fn link<T: Grid>(&self, with: Index, grid: &T) -> Result<(), OutOfBoundsError>;
    fn unchecked_link(&self, with: Index);
    // Return ids of neighboring cells linked with the current cell.
    fn links(&self) -> &RefCell<Vec<Index>>;
    fn has_link(&self, link: Index) -> bool {
        self.links().borrow().contains(&link)
    }
    fn has_neighbor(&self, neighbor: Index) -> bool {
        self.neighbor_ids().contains(&neighbor)
    }
    fn unchecked_unlink(&self, with: Index);
}

/// Any `Grid` type implementing `CoordLookup` can use `Coord` to lookup a cell's `Index`
pub trait CoordLookup: Grid {
    fn get_id(&self, coord: &Coord) -> Index;
    fn try_get_id(&self, coord: &Coord) -> Result<Index, OutOfBoundsCoordError>;
    fn get_coords(&self, id: Index) -> Coord;
    fn try_get_coords(&self, id: Index) -> Result<Coord, OutOfBoundsError>;
}

#[cfg(test)]
mod test {}
