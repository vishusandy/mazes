use crate::maze::{Cell, Grid};
use crate::render::{BasicOpts, RenderOps, Renderable, Renderer};
use crate::util::Index;
use image::{Rgba, RgbaImage};
#[derive(Clone, Debug)]
pub struct RenderGrid<'f, 'g, G: Grid + Renderable> {
    grid: &'g G,
    opts: BasicOpts<'f>,
}
impl<'f, 'g, G: Grid + Renderable> Renderer<'f> for RenderGrid<'f, 'g, G> {
    fn options(&self) -> &BasicOpts<'f> {
        &self.opts
    }
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        &mut self.opts
    }
}
impl<'f, 'g, G: Grid + Renderable> RenderGrid<'f, 'g, G> {
    pub(in crate) fn new(grid: &'g G) -> Self {
        Self {
            grid,
            opts: BasicOpts::default(),
        }
    }
    pub(in crate) fn with_options(grid: &'g G, opts: BasicOpts<'f>) -> Self {
        Self { grid, opts }
    }
}
impl<'f, 'g, G: Grid + Renderable> RenderOps for RenderGrid<'f, 'g, G> {
    type G = G;
    fn block_label(&self, id: Index) -> String {
        id.to_string()
    }
    fn block_bg(&self, _id: Index) -> &Rgba<u8> {
        self.opts.block_color()
    }
    fn render_grid(&self) -> RgbaImage {
        let opts = self.options();
        let (x, y) = self.grid().image_dimensions(opts);
        let mut image = RgbaImage::from_pixel(x, y, *opts.bg_color());
        for i in self.grid().iter() {
            let id = i.id();
            self.grid().render_block(
                id,
                &self.block_label(id),
                self.block_bg(id),
                &mut image,
                opts,
            );
        }
        image
    }
    fn grid(&self) -> &G {
        self.grid
    }
}
