#![allow(dead_code)]
use crate::maze::{CoordLookup, Grid};
use crate::render::{BasicOpts, Renderable};
use crate::util::{Cardinal, Horizontal, Index, Ordinal, Vertical};
use image::RgbaImage;
use imageproc::drawing::draw_line_segment_mut;
use parse_display::Display;

pub trait BlockCoords {
    // fn new<G: Grid + Renderable>(grid: &G, id: Index, opts: &BasicOpts) -> Self;
}
impl BlockCoords for FloatBlock {}
impl BlockCoords for UnsignedIntBlock {}
impl BlockCoords for SignedIntBlock {}

/// Stores coordinates to draw a block.
#[derive(Clone, Debug, Display)]
#[display("(({x1},{y1}), ({x2},{y2}))")]
pub struct FloatBlock {
    pub(in crate) x1: f32,
    pub(in crate) y1: f32,
    pub(in crate) x2: f32,
    pub(in crate) y2: f32,
}
impl FloatBlock {
    pub(in crate) fn new<G: Grid + CoordLookup + Renderable>(
        grid: &G,
        id: Index,
        opts: &BasicOpts,
    ) -> Self {
        let block = opts.block_size() as f32;
        let border = opts.border_width() as f32;
        let frame = opts.frame_size() as f32;
        let coords = grid.get_coords(id);
        let (col, row) = coords.float_tuple();
        let x1 = col * (block + border) + border + frame;
        let y1 = row * (block + border) + border + frame;
        let x2 = x1 + block;
        let y2 = y1 + block;
        Self { x1, y1, x2, y2 }
    }
    pub(in crate) fn x_offset(&self, offset: u32, d: &Horizontal) -> f32 {
        match d {
            Horizontal::W => self.left(offset),
            Horizontal::E => self.right(offset),
        }
    }
    pub(in crate) fn y_offset(&self, offset: u32, d: &Vertical) -> f32 {
        match d {
            Vertical::N => self.top(offset),
            Vertical::S => self.bottom(offset),
        }
    }
    pub(in crate) fn x_offset_signed(&self, offset: i32, d: &Horizontal) -> f32 {
        match d {
            Horizontal::W => self.left_signed(offset),
            Horizontal::E => self.right_signed(offset),
        }
    }
    pub(in crate) fn y_offset_signed(&self, offset: i32, d: &Vertical) -> f32 {
        match d {
            Vertical::N => self.top_signed(offset),
            Vertical::S => self.bottom_signed(offset),
        }
    }
    pub(in crate) fn left_signed(&self, offset: i32) -> f32 {
        self.x1 + offset as f32
    }
    pub(in crate) fn right_signed(&self, offset: i32) -> f32 {
        self.x2 - offset as f32
    }
    pub(in crate) fn top_signed(&self, offset: i32) -> f32 {
        self.y1 + offset as f32
    }
    pub(in crate) fn bottom_signed(&self, offset: i32) -> f32 {
        self.y2 - offset as f32
    }
    pub(in crate) fn left(&self, offset: u32) -> f32 {
        self.x1 + offset as f32
    }
    pub(in crate) fn right(&self, offset: u32) -> f32 {
        self.x2 - offset as f32
    }
    pub(in crate) fn top(&self, offset: u32) -> f32 {
        self.y1 + offset as f32
    }
    pub(in crate) fn bottom(&self, offset: u32) -> f32 {
        self.y2 - offset as f32
    }
    pub(in crate) fn corner(&self, d: &Ordinal) -> (f32, f32) {
        d.map(
            || (self.x1, self.y1),
            || (self.x2, self.y1),
            || (self.x2, self.y2),
            || (self.x1, self.y2),
        )
    }
    pub(in crate) fn corner_offset(&self, d: &Ordinal, offset_x: u32, offset_y: u32) -> (f32, f32) {
        d.map(
            || self.top_left(offset_x, offset_y),
            || self.top_right(offset_x, offset_y),
            || self.bottom_right(offset_x, offset_y),
            || self.bottom_left(offset_x, offset_y),
        )
    }
    pub(in crate) fn top_left(&self, offset_x: u32, offset_y: u32) -> (f32, f32) {
        (self.left(offset_x), self.bottom(offset_y))
    }
    pub(in crate) fn top_right(&self, offset_x: u32, offset_y: u32) -> (f32, f32) {
        (self.right(offset_x), self.bottom(offset_y))
    }
    pub(in crate) fn bottom_left(&self, offset_x: u32, offset_y: u32) -> (f32, f32) {
        (self.left(offset_x), self.top(offset_y))
    }
    pub(in crate) fn bottom_right(&self, offset_x: u32, offset_y: u32) -> (f32, f32) {
        (self.right(offset_x), self.top(offset_y))
    }
    pub(in crate) fn center_x(&self) -> f32 {
        (self.x1 + self.x2) / 2.0
    }
    pub(in crate) fn center_y(&self) -> f32 {
        (self.y1 + self.y2) / 2.0
    }
    pub(in crate) fn center(&self) -> (f32, f32) {
        (self.center_x(), self.center_y())
    }
    pub(in crate) fn side(&self, d: &Cardinal, offset: f32) -> ((f32, f32), (f32, f32)) {
        match d {
            Cardinal::N => ((self.x1, self.y1 + offset), (self.x2, self.y1 + offset)),
            Cardinal::E => ((self.x2 - offset, self.y1), (self.x2 - offset, self.y2)),
            Cardinal::S => ((self.x1, self.y2 - offset), (self.x2, self.y2 - offset)),
            Cardinal::W => ((self.x1 + offset, self.y1), (self.x1 + offset, self.y2)),
        }
    }
    // fn top_center(&self) -> (f32, f32) {
    // }
    // fn tuple(&self) -> ((f32, f32), (f32, f32)) {
    //     todo!()
    // }
}
#[derive(Clone, Debug, Display)]
#[display("(({x1},{y1}), ({x2},{y2}))")]
pub struct UnsignedIntBlock {
    pub(in crate) x1: u32,
    pub(in crate) y1: u32,
    pub(in crate) x2: u32,
    pub(in crate) y2: u32,
}
impl UnsignedIntBlock {
    pub(in crate) fn new<G: Grid + CoordLookup + Renderable>(
        grid: &G,
        id: Index,
        opts: &BasicOpts,
    ) -> Self {
        let block = opts.block_size() as u32;
        let border = opts.border_width() as u32;
        let frame = opts.frame_size() as u32;
        let coords = grid.get_coords(id);
        let (col, row) = coords.unsigned_tuple();
        // let x1 = col * (block + border) + border + frame;
        let x1 = col * (block + border) + border + frame;
        // let y1 = row * (block + border) + border + frame;
        let y1 = row * (block + border) + border + frame;
        // let x2 = x1 + block;
        let x2 = x1 + block + border - 1;
        // let y2 = y1 + block;
        let y2 = y1 + block + border - 1;
        Self { x1, y1, x2, y2 }
    }
    pub(in crate) fn float_side(&self, d: &Cardinal, offset: i32) -> ((f32, f32), (f32, f32)) {
        match d {
            Cardinal::N => (
                (self.x1 as f32, (self.y1 as i32 + offset) as f32),
                (self.x2 as f32, (self.y1 as i32 + offset) as f32),
            ),
            Cardinal::E => (
                ((self.x2 as i32 - offset) as f32, self.y1 as f32),
                ((self.x2 as i32 - offset) as f32, self.y2 as f32),
            ),
            Cardinal::S => (
                (self.x1 as f32, (self.y2 as i32 - offset) as f32),
                (self.x2 as f32, (self.y2 as i32 - offset) as f32),
            ),
            Cardinal::W => (
                ((self.x1 as i32 + offset) as f32, self.y1 as f32),
                ((self.x1 as i32 + offset) as f32, self.y2 as f32),
            ),
        }
    }
    pub(in crate) fn center_x(&self) -> u32 {
        (self.x1 + self.x2) / 2
    }
    pub(in crate) fn center_y(&self) -> u32 {
        (self.y1 + self.y2) / 2
    }
    pub(in crate) fn center(&self) -> (u32, u32) {
        (self.center_x(), self.center_y())
    }
    pub(in crate) fn draw_line(
        &self,
        d: &Cardinal,
        offset: i32,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        let line = self.float_side(d, offset);
        draw_line_segment_mut(image, line.0, line.1, *opts.border_color())
    }
}
#[derive(Clone, Debug, Display)]
#[display("(({x1},{y1}), ({x2},{y2}))")]
pub struct SignedIntBlock {
    pub(in crate) x1: i32,
    pub(in crate) y1: i32,
    pub(in crate) x2: i32,
    pub(in crate) y2: i32,
}
impl SignedIntBlock {
    pub(in crate) fn new<G: Grid + CoordLookup + Renderable>(
        grid: &G,
        id: Index,
        opts: &BasicOpts,
    ) -> Self {
        let block = opts.block_size() as i32;
        let border = opts.border_width() as i32;
        let frame = opts.frame_size() as i32;
        let coords = grid.get_coords(id);
        let (col, row) = coords.signed_tuple();
        let x1 = col * (block + border) + border + frame;
        let y1 = row * (block + border) + border + frame;
        let x2 = x1 + block;
        let y2 = y1 + block;
        Self { x1, y1, x2, y2 }
    }
    pub(in crate) fn center_x(&self) -> i32 {
        (self.x1 + self.x2) / 2
    }
    pub(in crate) fn center_y(&self) -> i32 {
        (self.y1 + self.y2) / 2
    }
    pub(in crate) fn center(&self) -> (i32, i32) {
        (self.center_x(), self.center_y())
    }
}

impl From<&UnsignedIntBlock> for FloatBlock {
    fn from(from: &UnsignedIntBlock) -> Self {
        FloatBlock {
            x1: from.x1 as f32,
            y1: from.y1 as f32,
            x2: from.x2 as f32,
            y2: from.y2 as f32,
        }
    }
}
