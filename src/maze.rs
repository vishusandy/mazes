pub mod rect;
pub mod sq;

pub use self::rect::CardinalGrid;
use crate::error::*;
use crate::iter::*;
use crate::trans::*;
use crate::util::dist::Distances;
use crate::util::path::Path;
use crate::util::*;
use linked_hash_set::LinkedHashSet;
use rand::Rng;
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
    fn get_unchecked(&self, id: Index) -> &<Self as GridProps>::C {
        self.lookup(id)
    }
    fn random<R: Rng + ?Sized>(&self, rng: &mut R) -> &<Self as GridProps>::C {
        self.lookup(self.random_id(rng))
    }
    fn random_id<R: Rng + ?Sized>(&self, rng: &mut R) -> Index {
        rng.gen_range(Index::zero().to(self.capacity()))
    }
    fn random_neighbor_id<R: Rng + ?Sized>(&self, id: Index, rng: &mut R) -> Index {
        use rand::seq::SliceRandom;
        *self.lookup(id).neighbor_ids().choose(rng).unwrap()
    }
    fn random_neighbor<R: Rng + ?Sized>(&self, id: Index, rng: &mut R) -> &<Self as GridProps>::C {
        self.lookup(self.random_neighbor_id(id, rng))
    }
    /// Flood fill to find out how far each block is away from a starting point.
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
    /// Implementation of the Aldous-Broder algorithm.  It uses a random walk to link unvisited
    /// cells to neighbors.
    ///
    /// See also: [Aldous-Broder Algorithm][aldous-broder]
    ///
    /// [aldous-broder]: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Aldous-Broder_algorithm
    fn aldous_broder<R: Rng + ?Sized>(size: usize, rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let grid = Self::setup(size);
        let mut cell = grid.random_id(rng);
        let mut unvisited = grid.capacity().minus(1usize);
        while unvisited > 0 {
            let neighbor = grid.random_neighbor(cell, rng);
            if neighbor.not_linked() {
                grid.link(cell, neighbor.id()).unwrap();
                unvisited -= 1;
            }
            cell = neighbor.id();
        }
        grid
    }
    /// An implementation of [Wilson's Algorithm][wilsons].  It uses a loop-erased walk.
    ///
    /// See also: [Wilson's Algorithm][wilsons]
    ///
    /// [wilsons]: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Wilson's_algorithm
    fn wilsons<R: Rng + ?Sized>(size: usize, rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let grid = Self::setup(size);
        let mut unvisited: LinkedHashSet<Index> = (0..*grid.capacity()).map(Index::from).collect();
        let first = *unvisited
            .iter()
            .nth(rng.gen_range(0..unvisited.len()))
            .unwrap();
        unvisited.remove(&first);
        while !unvisited.is_empty() {
            let mut cell = *unvisited
                .iter()
                .nth(rng.gen_range(0..unvisited.len()))
                .unwrap();
            let mut path = vec![cell];
            while unvisited.contains(&cell) {
                cell = grid.random_neighbor_id(cell, rng);
                if let Some(pos) = path.iter().position(|i| *i == cell) {
                    path.truncate(pos + 1);
                } else {
                    path.push(cell);
                }
            }
            for i in 0..path.len() - 1 {
                let a = path[i];
                grid.link(a, path[i + 1]).unwrap();
                unvisited.remove(&a);
            }
        }
        grid
    }
}

/// Helper methods that make the [`Grid`] trait actually work.
///
/// This is separate from [`Grid`] to allow more granular control over which methods are visible
/// by default.
///
/// Types implementing the [`Grid`] trait must also implememnt [`GridProps`].
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
    fn is_linked(&self) -> bool {
        !self.links().borrow().is_empty()
    }
    fn not_linked(&self) -> bool {
        self.links().borrow().is_empty()
    }
}

/// Any `Grid` type implementing `CoordLookup` can use `Coord` to lookup a cell's `Index`
pub trait CoordLookup: Grid {
    fn get_id(&self, coord: &Coord) -> Index;
    fn try_get_id(&self, coord: &Coord) -> Result<Index, OutOfBoundsCoordError>;
    fn get_coords(&self, id: Index) -> Coord;
    fn try_get_coords(&self, id: Index) -> Result<Coord, OutOfBoundsError>;
}

#[cfg(test)]
mod test {
    use crate::maze::sq::SqGrid;
    use crate::maze::Grid;
    use crate::render::Renderer;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;

    #[test]
    fn aldous_broder() -> Result<(), image::ImageError> {
        let mut rng = SplitMix64::seed_from_u64(852);
        let grid = SqGrid::aldous_broder(6, &mut rng);
        grid.render_defaults()
            .save_render(std::path::Path::new("aldous_broder.png"))
    }
    #[test]
    fn wilsons() -> Result<(), image::ImageError> {
        let mut rng = SplitMix64::seed_from_u64(852);
        let grid = SqGrid::wilsons(7, &mut rng);
        grid.render_defaults()
            .save_render(std::path::Path::new("wilsons.png"))
    }
}
