use crate::error::NotNeighborsError;
use crate::iter::*;
use crate::maze::{Cell, Grid};
use crate::trans::major::{Major, RowMajor};
use crate::trans::*;
use crate::util::*;
use rand::seq::SliceRandom;
use rand::Rng;

/// Describes grids that can be navigated using cardinal directions.
/// This allows starting iteration at an ordinal direction.
pub trait CardinalGrid: Grid {
    fn row_size(&self) -> RowSize;
    fn col_size(&self) -> ColSize;
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
    fn has_dir_link(&self, id: Index, d: &Cardinal) -> bool {
        if let Some(cell) = self.get(id) {
            if let Some(n) = self.neighbor(id, d) {
                return cell.has_link(n);
            }
        }
        false
    }
    fn neighbor(&self, id: Index, d: &Cardinal) -> Option<Index> {
        self.calc_dir(id, d)
    }
    fn link_neighbor(&self, id: Index, d: &Cardinal) -> Result<(), NotNeighborsError<Cardinal>> {
        if let Some(n) = self.neighbor(id, d) {
            self.link(id, n).unwrap();
            Ok(())
        } else {
            Err(NotNeighborsError::new(
                id,
                *d,
                "Cell does not have a neighbor in specified direction",
            ))
        }
    }
    fn unlink_neighbor(&self, id: Index, d: &Cardinal) -> Result<(), NotNeighborsError<Cardinal>> {
        if let Some(n) = self.neighbor(id, d) {
            self.unlink(id, n).unwrap();
            Ok(())
        } else {
            Err(NotNeighborsError::new(
                id,
                *d,
                "Cell does not have a neighbor in specified direction",
            ))
        }
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
    fn binary_tree<R: Rng + ?Sized>(size: usize, rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let grid = Self::setup(size);
        // for cell in (0..grid.row_size())
        for cell in (0..*grid.capacity()).filter_map(|i| grid.get(i.into())) {
            let id = cell.id();
            let flip: bool = rng.gen();
            match (grid.has_boundary_east(id), grid.has_boundary_south(id)) {
                (false, false) => {
                    if flip {
                        grid.link_neighbor(id, &Cardinal::E).unwrap();
                    } else {
                        grid.link_neighbor(id, &Cardinal::S).unwrap();
                    }
                }
                (false, true) => {
                    grid.link_neighbor(id, &Cardinal::E).unwrap();
                }
                (true, false) => {
                    grid.link_neighbor(id, &Cardinal::S).unwrap();
                }
                (true, true) => {}
            }
        }
        grid
    }
    fn sidewinder<R: Rng + ?Sized>(size: usize, rng: &mut R) -> Self
    where
        Self: Sized,
    {
        let grid = Self::setup(size);
        let mut run = Vec::new();
        for cell in (0..*grid.capacity()).filter_map(|i| grid.get(i.into())) {
            let id = cell.id();
            run.push(id);
            let flip: bool = rng.gen();
            if grid.has_boundary_east(id) || (!grid.has_boundary_south(id) && !flip) {
                let run_id = *run.choose(rng).unwrap();
                if let Some(n) = grid.neighbor(run_id, &Cardinal::S) {
                    grid.link(run_id, n).unwrap();
                    run.truncate(0);
                }
            } else if let Some(n) = grid.neighbor(cell.id(), &Cardinal::E) {
                grid.link(id, n).unwrap();
            }
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::sq::SqGrid;
    use crate::render::Renderer;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;
    #[test]
    fn binary_tree() -> Result<(), image::ImageError> {
        let mut rng = SplitMix64::seed_from_u64(80);
        let grid = SqGrid::binary_tree(5, &mut rng);
        grid.render_defaults()
            .save_render(std::path::Path::new("binary_tree.png"))
    }
    #[test]
    fn sidewinder() -> Result<(), image::ImageError> {
        let mut rng = SplitMix64::seed_from_u64(583);
        let grid = SqGrid::sidewinder(10, &mut rng);
        grid.render_defaults()
            .save_render(std::path::Path::new("sidwinder.png"))
    }
}
