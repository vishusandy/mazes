pub mod cell;
use crate::error::*;
use crate::maze::{CardinalGrid, Cell, CoordLookup, Grid, GridProps};
use crate::render::blocks::{FloatBlock, UnsignedIntBlock};
use crate::render::renderers::RenderGrid;
use crate::render::{BasicOpts, Renderable};
use crate::util::Index;
use crate::util::*;
pub use cell::SqCell;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;

// todo: impl Default, Index, Display
#[derive(Clone, Debug)]
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
                    if let Some(neighbor) = grid.calc_dir(cur_id, &d) {
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

impl SqGrid {
    pub fn render_options<'f, 'o, 'g>(
        &'g self,
        opts: &'o BasicOpts<'f>,
    ) -> RenderGrid<'f, 'o, 'g, SqGrid> {
        RenderGrid::with_options(self, opts)
    }
    pub fn render_defaults<'f, 'o, 'g>(&'g self) -> RenderGrid<'f, 'o, 'g, SqGrid> {
        RenderGrid::new(self)
    }
    fn draw_joint_corner(
        &self,
        block: &FloatBlock,
        d: &Ordinal,
        pad: u32,
        color: &Rgba<u8>,
        opts: &BasicOpts,
        image: &mut RgbaImage,
    ) {
        let corner = block.corner(d);
        let x = d.side_x();
        let y = d.side_y();
        let add_x = |x: f32, offset: f32, ax: &Horizontal| -> f32 {
            match ax {
                Horizontal::W => x - offset,
                Horizontal::E => x + offset,
            }
        };
        let add_y = |y: f32, offset: f32, ay: &Vertical| -> f32 {
            match ay {
                Vertical::N => y - offset,
                Vertical::S => y + offset,
            }
        };
        let floor = opts.border_width() / 2;
        let rem = opts.border_width() % 2;
        for i in 0..(floor + rem) {
            let pad = if opts.tri_joints() { pad + i } else { pad };
            let cx = add_x(corner.0, i as f32, &x);
            let cy = add_y(corner.1, i as f32, &y);
            //Horizontal joint line
            let line1 = ((cx, cy), (block.x_offset(pad, &x), cy));
            // Vertical joint line
            let line2 = ((cx, cy), (cx, block.y_offset(pad, &y)));
            draw_line_segment_mut(image, line1.0, line1.1, *color);
            draw_line_segment_mut(image, line2.0, line2.1, *color);
        }
    }
}

impl Renderable for SqGrid {
    type B = UnsignedIntBlock;
    fn draw_joint(&self, id: Index, block: &Self::B, image: &mut RgbaImage, opts: &BasicOpts) {
        let pad = opts.joint_size();
        if pad == 0 {
            return;
        }
        let mut block = FloatBlock::from(block);
        block.x1 -= 1.0;
        block.y1 -= 1.0;
        block.x2 -= (opts.border_width()) as f32 - 1.0;
        block.y2 -= (opts.border_width()) as f32 - 1.0;
        for d in Ordinal::iter() {
            let x: Cardinal = d.side_x().into();
            let y: Cardinal = d.side_y().into();
            let jc = opts.joint_color();
            match (self.has_boundary(id, x), self.has_boundary(id, y)) {
                (false, false) => self.draw_joint_corner(&block, &d, pad, jc, opts, image),
                (true, false) => self.draw_joint_corner(&block, &d, pad, jc, opts, image),
                (false, true) => self.draw_joint_corner(&block, &d, pad, jc, opts, image),
                _ => {}
            };
        }
    }
    fn draw_block_outline(
        &self,
        id: Index,
        block: &Self::B,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        if opts.border_width() == 0 {
            return;
        }
        let cell = self.lookup(id);
        let border = opts.border_width();
        for d in Cardinal::iter() {
            if d.north() || d.west() {
                // adjust northwest corner (otherwise there is an empty square in northwest corner)
                if self.has_boundary(id, Cardinal::N) && self.has_boundary(id, Cardinal::W) {
                    let mut block = block.clone();
                    block.y1 -= border;
                    block.x1 -= border;
                    for i in 0..(border as i32) {
                        block.draw_line(&d, i, image, opts)
                    }
                }
                if self.has_boundary(id, d) {
                    for i in 0 - (border as i32)..0 {
                        block.draw_line(&d, i, image, opts)
                    }
                }
                continue;
            }
            if let Some(n) = self.neighbor(id, &d) {
                if !cell.links().borrow().contains(&n) {
                    for i in 0..border {
                        block.draw_line(&d, i as i32, image, opts);
                    }
                }
            } else if self.has_boundary(id, d) {
                for i in 0..border {
                    block.draw_line(&d, i as i32, image, opts);
                }
            } else {
            }
        }
    }
    fn fill_block_bg(
        &self,
        _id: Index,
        block: &Self::B,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        let size = opts.block_size();
        draw_filled_rect_mut(
            image,
            Rect::at(block.x1 as i32, block.y1 as i32).of_size(size, size),
            *color,
        )
    }
    fn draw_block_text(
        &self,
        _id: Index,
        block: &Self::B,
        text: &str,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        if !opts.text_labels() {
            return;
        }
        let scale = opts.font_scale();
        let x: u32;
        let y: u32;
        if opts.center_labels() {
            let center = block.text_center(opts);
            x = center.0;
            y = center.1;
        } else {
            x = block.x1 + opts.block_padding();
            y = block.y1 + opts.block_padding();
        }
        draw_text_mut(image, *color, x, y, scale, opts.font(), text);
    }
    fn image_dimensions(&self, opts: &BasicOpts) -> (u32, u32) {
        let (rows, cols) = self.dimensions();
        let frame = opts.frame_size();
        let border = opts.border_width();
        let block = opts.block_size();
        let x = frame + border + cols.mul(block + border) as u32 + frame;
        let y = frame + border + rows.mul(block + border) as u32 + frame;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Renderer;
    use std::path::Path;
    #[test]
    fn render_sq_grid_defaults() -> Result<(), image::ImageError> {
        let grid = SqGrid::new(6);
        let file = "grid.png";
        let path = Path::new(file);
        grid.render_defaults().save_render(path)?;
        if !path.exists() {
            panic!("Render failed - image '{}' was not created", file);
        }
        Ok(())
    }
    #[test]
    fn render_sq_grid_options() -> Result<(), image::ImageError> {
        let grid = SqGrid::new(6);
        let file = "grid_opts.png";
        let path = Path::new(file);
        let mut options = BasicOpts::debug();
        options.set_center_labels(false);
        grid.render_options(&options).save_render(path)?;
        if !path.exists() {
            panic!("Render failed - image '{}' was not created", file);
        }
        Ok(())
        // panic!();
    }
}
