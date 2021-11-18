use crate::maze::sq::SqGrid;
use crate::maze::{CardinalGrid, Grid};
use crate::render::blocks::{SignedIntBlock, UnsignedIntBlock};
use crate::render::renderers::dist_map::{calc_bg, DistMask};
use crate::render::{BasicOpts, Renderable, Renderer, RendererOps};
use crate::util::dist::Distances;
use crate::util::path::Path;
use crate::util::{Cardinal, Index};
use either::Either;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_antialiased_line_segment_mut;
use log::trace;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PathMapRenderer<'f, 'g, 'p, G: Grid + Renderable + Clone> {
    grid: &'g G,
    basic_opts: BasicOpts<'f>,
    path_opts: PathMapOpts,
    path: Cow<'p, Path<'g, G>>,
}
// first line = center - stroke_width/2

impl<'f, 'g, 'p> PathMapRenderer<'f, 'g, 'p, SqGrid> {
    pub(in crate) fn new(
        path: &'p Path<'g, SqGrid>,
        basic_opts: Option<BasicOpts<'f>>,
        path_opts: Option<PathMapOpts>,
    ) -> Self {
        Self {
            grid: path.grid(),
            basic_opts: basic_opts.unwrap_or_default(),
            path_opts: path_opts.unwrap_or_default(),
            path: Cow::Borrowed(path),
        }
    }
}
impl<'f, 'o, 'g, 'm> Renderer<'f> for PathMapRenderer<'f, 'g, 'm, SqGrid> {}
impl<'f, 'g, 'p> RendererOps<'f> for PathMapRenderer<'f, 'g, 'p, SqGrid> {
    type G = SqGrid;
    fn options<'a>(&'a self) -> &'a BasicOpts<'f> {
        &self.basic_opts
    }
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        &mut self.basic_opts
    }
    fn grid(&self) -> &Self::G {
        self.grid
    }
    fn block_coords(&self, id: Index) -> <Self::G as Renderable>::B {
        UnsignedIntBlock::new(self.grid, id, &self.basic_opts)
    }
    fn block_bg(&self, id: Index) -> Rgba<u8> {
        let bg = self.basic_opts.bg_color();
        match &self.path_opts.bg {
            PathMapBg::Uniform => *bg,
            PathMapBg::Solid(c) => *c,
            PathMapBg::Intensity(mask) => {
                let max = (self.path.len() - 1) as f32;
                match self.path.position(id) {
                    Some(dist) => calc_bg(dist, max, mask),
                    None => *bg,
                }
            }
        }
    }
    fn block_label(&self, id: Index) -> String {
        self.path
            .get(*id)
            .map(|d| match self.path_opts.show_distance {
                true => d.to_string(),
                false => id.to_string(),
            })
            .unwrap_or_else(|| match self.path_opts.show_outside_labels {
                true => id.to_string(),
                false => String::new(),
            })
    }
    fn render_extra(&self, id: Index, block: &<Self::G as Renderable>::B, image: &mut RgbaImage) {
        let block: SignedIntBlock = block.into();
        if let Some(d) = self.path.prev_dir(id) {
            block.draw_cardinal_line_to_center(
                &d,
                self.path_opts.pad_start(),
                self.path_opts.arrow_color(),
                image,
            );
        }
        if let Some(d) = self.path.next_dir(id) {
            block.draw_cardinal_line_to_center(
                &d,
                self.path_opts.pad_end(),
                self.path_opts.arrow_color(),
                image,
            );
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathMapOpts {
    draw_arrows: bool,
    arrow_color: Rgba<u8>,
    pad_start: i32,
    pad_end: i32,
    stroke_width: i32,
    /// The breadth, or width, of the arrow point
    breadth: i32,
    /// The depth, or length, of the arrow point
    depth: i32,
    /// Whether or not to show text labels for cells outside of the path
    show_outside_labels: bool,
    /// Whether to display the distance or the cell id for text labels in the path
    show_distance: bool,
    /// Determines how to determine the background color for blocks within the [`Path`].
    ///
    /// See [`PathMapBg`] for more.
    bg: PathMapBg,
}
impl Default for PathMapOpts {
    fn default() -> Self {
        Self {
            draw_arrows: false,
            arrow_color: Rgba([0, 0, 0, 255]),
            pad_start: 5,
            pad_end: 5,
            stroke_width: 1,
            breadth: 5,
            depth: 5,
            show_outside_labels: false,
            show_distance: true,
            bg: PathMapBg::default(),
        }
    }
}
impl PathMapOpts {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn draw_arrows(&self) -> bool {
        self.draw_arrows
    }
    pub fn arrow_color(&self) -> Rgba<u8> {
        self.arrow_color
    }
    pub fn pad_start(&self) -> i32 {
        self.pad_start
    }
    pub fn pad_end(&self) -> i32 {
        self.pad_end
    }
    pub fn stroke_width(&self) -> i32 {
        self.stroke_width
    }
    pub fn breadth(&self) -> i32 {
        self.breadth
    }
    pub fn depth(&self) -> i32 {
        self.depth
    }
    pub fn show_outside_labels(&self) -> bool {
        self.show_outside_labels
    }
    pub fn show_distance(&self) -> bool {
        self.show_distance
    }
    pub fn bg(&self) -> &PathMapBg {
        &self.bg
    }
    pub fn set_draw_arrows(&mut self, val: bool) {
        self.draw_arrows = val;
    }
    pub fn set_arrow_color(&mut self, color: Rgba<u8>) {
        self.arrow_color = color;
    }
    pub fn set_pad_start(&mut self, pad: i32) {
        self.pad_start = pad;
    }
    pub fn set_pad_end(&mut self, pad: i32) {
        self.pad_end = pad;
    }
    pub fn set_stroke_width(&mut self, width: i32) {
        self.stroke_width = width;
    }
    pub fn set_breadth(&mut self, breadth: i32) {
        self.breadth = breadth;
    }
    pub fn set_depth(&mut self, depth: i32) {
        self.depth = depth;
    }
    pub fn set_show_outside_labels(&mut self, show: bool) {
        self.show_outside_labels = show;
    }
    pub fn set_show_distance(&mut self, show: bool) {
        self.show_distance = show;
    }
    pub fn set_bg(&mut self, bg: PathMapBg) {
        self.bg = bg;
    }
}
#[derive(Clone, Debug)]
/// Specifies the background color for blocks inside the path.  Blocks outside of the path will
/// always use `bg_color()` as the background.  Blocks inside the cell can use one of:
/// - Uniform: uses `bg_color` - all blocks will have the same background color
/// - Solid(Rgba<u8>): use a specified color for all blocks inside the path
/// - Intensity(DistMask): compute the color using the `calc_bg` intentsity function from the [`dist_map`] module
pub enum PathMapBg {
    /// Uses the same background color for blocks outside and inside the [`Path`].
    Uniform,
    /// Use a specific color for blocks within the [`Path`]
    Solid(Rgba<u8>),
    /// Use the background color intensity function from the [`dist_map`] module to dynamically
    /// color the cells according to progress (distance from start).
    Intensity(DistMask),
}
impl PathMapBg {
    pub fn uniform() -> Self {
        Self::Uniform
    }
    pub fn solid(color: Rgba<u8>) -> Self {
        Self::Solid(color)
    }
    pub fn intensity(mask: DistMask) -> Self {
        Self::Intensity(mask)
    }
}
impl Default for PathMapBg {
    fn default() -> Self {
        Self::Intensity(DistMask::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::sq::tests::new_maze;
    use crate::maze::Grid;
    use crate::render::Renderer;
    use crate::util::Index;
    #[test]
    fn path_map_renderer_defaults() -> Result<(), image::ImageError> {
        new_maze(5)
            .distances(Index::zero())
            .shortest_path(24.into())
            .render_defaults()
            .save_render(std::path::Path::new("shortest_path.png"))
    }
}
