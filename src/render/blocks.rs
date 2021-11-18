#![allow(dead_code)]
use crate::maze::{CoordLookup, Grid};
use crate::render::renderers::path_map::PathMapOpts;
use crate::render::{BasicOpts, Renderable};
use crate::util::{Cardinal, Horizontal, Index, Ordinal, Vertical};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut};
use imageproc::pixelops::interpolate;
use parse_display::Display;

pub trait BlockCoords {}
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
    pub(in crate) fn center_x(&self, opts: &BasicOpts) -> f32 {
        self.x1 + (opts.block_size() as f32 / 2.0)
    }
    pub(in crate) fn center_y(&self, opts: &BasicOpts) -> f32 {
        self.y1 + (opts.block_size() as f32 / 2.0)
    }
    pub(in crate) fn center(&self, opts: &BasicOpts) -> (f32, f32) {
        (self.center_x(opts), self.center_y(opts))
    }
    pub(in crate) fn side(&self, d: &Cardinal, offset: f32) -> ((f32, f32), (f32, f32)) {
        match d {
            Cardinal::N => ((self.x1, self.y1 + offset), (self.x2, self.y1 + offset)),
            Cardinal::E => ((self.x2 - offset, self.y1), (self.x2 - offset, self.y2)),
            Cardinal::S => ((self.x1, self.y2 - offset), (self.x2, self.y2 - offset)),
            Cardinal::W => ((self.x1 + offset, self.y1), (self.x1 + offset, self.y2)),
        }
    }
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
        let x1 = col * (block + border) + border + frame;
        let y1 = row * (block + border) + border + frame;
        let x2 = x1 + block + border - 1;
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
    pub(in crate) fn center_x(&self, opts: &BasicOpts) -> u32 {
        self.x1 + opts.border_width() / 2
    }
    pub(in crate) fn center_y(&self, opts: &BasicOpts) -> u32 {
        self.y1 + opts.border_width() / 2
    }
    // Returns the center of a block (`border_width` is not included).
    pub(in crate) fn center(&self, opts: &BasicOpts) -> (u32, u32) {
        (self.center_x(opts), self.center_y(opts))
    }
    /// Calculate where to place text when `center_labels = true` in order for the text to appear
    /// in the visual center of the block.
    ///
    /// This will apply the `label_offset` and subtract `font_size / 2` which will return the
    /// approximate position for text for it to appear centered.
    pub(in crate) fn text_center(&self, opts: &BasicOpts) -> (u32, u32) {
        let offset = opts.label_offset() - (opts.font_size() as i32 / 2);
        (
            ((self.x1 + opts.block_size() / 2) as i32 + offset) as u32,
            ((self.y1 + opts.block_size() / 2) as i32 + offset) as u32,
        )
    }
    pub(in crate) fn draw_edge(
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
    pub(in crate) fn center_x(&self, opts: &BasicOpts) -> i32 {
        self.x1 + (opts.block_size() as i32 / 2)
    }
    pub(in crate) fn center_y(&self, opts: &BasicOpts) -> i32 {
        self.y1 + (opts.block_size() as i32 / 2)
    }
    pub(in crate) fn center(&self, opts: &BasicOpts) -> (i32, i32) {
        (self.center_x(opts), self.center_y(opts))
    }
    fn left_edge(&self) -> i32 {
        self.x1
    }
    fn right_edge(&self) -> i32 {
        self.x2
    }
    fn top_edge(&self) -> i32 {
        self.y1
    }
    fn bottom_edge(&self) -> i32 {
        self.y2
    }
    pub fn draw_cardinal_line_to_center(
        &self,
        dir: &Cardinal,
        offset: i32,
        color: Rgba<u8>,
        opts: &BasicOpts,
        image: &mut RgbaImage,
    ) {
        let center = self.center(opts);
        let pt = self.cardinal_arrow_edge(dir, offset, center);
        draw_antialiased_line_segment_mut(image, center, pt, color, interpolate);
    }
    fn cardinal_arrow_edge(&self, dir: &Cardinal, offset: i32, center: (i32, i32)) -> (i32, i32) {
        let (cx, cy) = center;
        match dir {
            Cardinal::N => (cx, self.top_edge() + offset),
            Cardinal::E => (self.right_edge() - offset, cy),
            Cardinal::S => (cx, self.bottom_edge() - offset),
            Cardinal::W => (self.left_edge() + offset, cy),
        }
    }
    fn cardinal_arrow_tip(
        &self,
        dir: &Cardinal,
        breadth: i32,
        depth: i32,
        offset: i32,
        opts: &BasicOpts,
    ) -> ((i32, i32), (i32, i32)) {
        let center = self.center(opts);
        let start = self.cardinal_arrow_edge(dir, offset, center);
        match dir {
            Cardinal::N => (sw(start, breadth, depth), se(start, breadth, depth)),
            Cardinal::E => (nw(start, depth, breadth), sw(start, depth, breadth)),
            Cardinal::S => (nw(start, breadth, depth), ne(start, breadth, depth)),
            Cardinal::W => (ne(start, depth, breadth), se(start, depth, breadth)),
        }
    }
    pub fn draw_cardinal_arrow_tip(
        &self,
        dir: &Cardinal,
        color: Rgba<u8>,
        basic: &BasicOpts,
        opts: &PathMapOpts,
        image: &mut RgbaImage,
    ) {
        let start = self.cardinal_arrow_edge(dir, opts.pad_end(), self.center(basic));
        let pt = self.cardinal_arrow_tip(dir, opts.breadth(), opts.depth(), opts.pad_end(), basic);
        draw_antialiased_line_segment_mut(image, start, pt.0, color, interpolate);
        draw_antialiased_line_segment_mut(image, start, pt.1, color, interpolate);
    }
}
fn down(pt: (i32, i32), offset: i32) -> (i32, i32) {
    (pt.0, pt.1 + offset)
}
fn up(pt: (i32, i32), offset: i32) -> (i32, i32) {
    (pt.0, pt.1 - offset)
}
fn left(pt: (i32, i32), offset: i32) -> (i32, i32) {
    (pt.0 - offset, pt.1)
}
fn right(pt: (i32, i32), offset: i32) -> (i32, i32) {
    (pt.0 + offset, pt.1)
}
fn sw(start: (i32, i32), breadth: i32, depth: i32) -> (i32, i32) {
    down(left(start, breadth), depth)
}
fn se(start: (i32, i32), breadth: i32, depth: i32) -> (i32, i32) {
    down(right(start, breadth), depth)
}
fn nw(start: (i32, i32), breadth: i32, depth: i32) -> (i32, i32) {
    up(left(start, breadth), depth)
}
fn ne(start: (i32, i32), breadth: i32, depth: i32) -> (i32, i32) {
    up(right(start, breadth), depth)
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
impl From<&UnsignedIntBlock> for SignedIntBlock {
    fn from(from: &UnsignedIntBlock) -> Self {
        SignedIntBlock {
            x1: from.x1 as i32,
            y1: from.y1 as i32,
            x2: from.x2 as i32,
            y2: from.y2 as i32,
        }
    }
}
