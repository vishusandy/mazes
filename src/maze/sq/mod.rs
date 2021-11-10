pub mod cell;
use crate::error::*;
use crate::maze::{CardinalGrid, CoordLookup, Grid, GridProps};
use crate::util::*;
pub use cell::SqCell;

// todo: impl Default, Index, Display
pub struct SqGrid {
    size: RowSize,
    cells: Vec<SqCell>,
}
impl SqGrid {
    fn new(size: usize) -> Self {
        Self {
            size: size.into(),
            cells: Vec::with_capacity(size * size),
        }
    }
    pub fn size(&self) -> RowSize {
        self.size
    }
    pub fn has_boundary(&self, id: Index, dir: Cardinal) -> bool {
        match dir {
            Cardinal::N => self.has_boundary_north(id),
            Cardinal::E => self.has_boundary_east(id),
            Cardinal::S => self.has_boundary_south(id),
            Cardinal::W => self.has_boundary_west(id),
        }
    }
    pub fn has_boundary_north(&self, id: Index) -> bool {
        id.lt(self.size)
    }
    pub fn has_boundary_east(&self, id: Index) -> bool {
        id.rem(self.size) == *self.size - 1
    }
    pub fn has_boundary_south(&self, id: Index) -> bool {
        id.div(self.size) == *self.size - 1
    }
    pub fn has_boundary_west(&self, id: Index) -> bool {
        id.rem(self.size) == 0
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
            false => Some(id.minus(self.size).into()),
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
            false => Some(id.plus(self.size).into()),
        }
    }
    fn calc_west(&self, id: Index) -> Option<Index> {
        match self.has_boundary_west(id) {
            true => None,
            false => Some(id.minus(1usize).into()),
        }
    }
    pub fn corner_id(&self, dir: Ordinal) -> Index {
        match dir {
            Ordinal::Nw => 0,
            Ordinal::Ne => self.size.minus(1),
            Ordinal::Se => self.capacity().minus(1),
            Ordinal::Sw => self.capacity().minus(self.size),
        }
        .into()
    }
}
impl CardinalGrid for SqGrid {
    fn row_size(&self) -> RowSize {
        self.size
    }
}
impl CoordLookup for SqGrid {
    fn get_id(&self, coord: &Coord) -> Index {
        coord.y() * self.size.into() + coord.x()
    }
    fn try_get_id(&self, coord: &Coord) -> Result<Index, OutOfBoundsCoordError> {
        let id = coord.y() * self.size.into() + coord.x();
        if (0..*self.size).contains(&id) {
            Ok(id)
        } else {
            Err(OutOfBoundsCoordError::new(*coord))
        }
    }
    fn get_coords(&self, id: Index) -> Coord {
        let x = id.rem(self.size);
        let y = id.div(self.size);
        Coord::new(x.into(), y.into())
    }
    fn try_get_coords(&self, id: Index) -> Result<Coord, OutOfBoundsError> {
        let x = id.rem(self.size);
        let y = id.div(self.size);
        if x < *self.size && y < *self.size {
            Ok(Coord::new(x.into(), y.into()))
        } else {
            Err(OutOfBoundsError::new(id))
        }
    }
}
impl Grid for SqGrid {}

impl GridProps for SqGrid {
    type C = SqCell;
    fn setup(size: usize) -> Self {
        let mut grid = SqGrid::new(size);
        let id = |row: usize, col: usize| Index::from(row * size + col);
        for r in 0..size {
            for c in 0..size {
                let mut neighbors = Vec::<Index>::new();
                let cur_id = id(r, c);
                for d in CardinalIter::iter() {
                    if let Some(neighbor) = grid.calc_dir(cur_id, d) {
                        neighbors.push(neighbor);
                    }
                }
                let cell = SqCell::new(cur_id, neighbors);
                grid.cells.push(cell);
            }
        }
        grid
    }
    fn capacity(&self) -> Capacity {
        self.size.cap()
    }
    fn cells(&self) -> &Vec<<Self as GridProps>::C> {
        &self.cells
    }
}
