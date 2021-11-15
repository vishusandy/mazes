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
    fn blank(size: usize) -> Self {
        Self {
            size: size.into(),
            cells: Vec::with_capacity(size * size),
        }
    }
    pub fn new(size: usize) -> Self {
        let mut grid = SqGrid::blank(size);
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
}
impl CardinalGrid for SqGrid {
    fn row_size(&self) -> RowSize {
        self.size
    }
    fn dimensions(&self) -> (RowSize, ColSize) {
        (self.row_size(), self.row_size())
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
        Self::new(size)
    }
    fn capacity(&self) -> Capacity {
        self.size.cap()
    }
    fn cells(&self) -> &Vec<<Self as GridProps>::C> {
        &self.cells
    }
}

use crate::render::renderers::RenderGrid;
use crate::render::{BasicOpts, Renderable};
use crate::util::Index;
use image::{Rgba, RgbaImage};
impl SqGrid {
    pub fn render_options<'f, 'g>(&'g self, opts: BasicOpts<'f>) -> RenderGrid<'f, 'g, SqGrid> {
        RenderGrid::with_options(self, opts)
    }
    pub fn render_defaults<'f, 'g>(&'g self) -> RenderGrid<'f, 'g, SqGrid> {
        RenderGrid::new(self)
    }
}
// #[sealed]
impl Renderable for SqGrid {
    fn draw_joint(&self, id: Index, image: &mut RgbaImage, opts: &BasicOpts) {
        todo!()
    }
    fn draw_block_outline(&self, id: Index, image: &mut RgbaImage, opts: &BasicOpts) {
        todo!()
    }
    fn fill_block_bg(&self, id: Index, color: &Rgba<u8>, image: &mut RgbaImage, opts: &BasicOpts) {
        todo!()
    }
    fn draw_block_text(
        &self,
        id: Index,
        text: &str,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        todo!()
    }
    fn image_dimensions(&self, opts: &BasicOpts) -> (u32, u32) {
        todo!()
    }
}
