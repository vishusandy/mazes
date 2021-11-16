use crate::maze::sq::SqGrid;
use crate::maze::Grid;
use crate::render::blocks::UnsignedIntBlock;
use crate::render::{BasicOpts, Renderable, Renderer, RendererOps};
use crate::util::Index;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct RenderGrid<'f, 'o, 'g, G: Grid + Renderable> {
    grid: &'g G,
    opts: Cow<'o, BasicOpts<'f>>,
}
// impl<'f, 'o, 'g, G: Grid + Renderable> Renderer<'f> for RenderGrid<'f, 'o, 'g, G> {}
impl<'f, 'o, 'g> Renderer<'f> for RenderGrid<'f, 'o, 'g, SqGrid> {}
// impl<'f, 'o, 'g, G: Grid + Renderable> RenderGrid<'f, 'o, 'g, G> {
impl<'f, 'o, 'g> RenderGrid<'f, 'o, 'g, SqGrid> {
    pub(in crate) fn new(grid: &'g SqGrid) -> Self {
        Self {
            grid,
            opts: Cow::Owned(BasicOpts::default()),
        }
    }
    pub(in crate) fn with_options(grid: &'g SqGrid, opts: &'o BasicOpts<'f>) -> Self {
        Self {
            grid,
            opts: Cow::Borrowed(opts),
        }
    }
}
// impl<'f, 'o, 'g, G: Grid + Renderable> RendererOps<'f> for RenderGrid<'f, 'o, 'g, G> {
impl<'f, 'o, 'g> RendererOps<'f> for RenderGrid<'f, 'o, 'g, SqGrid> {
    type G = SqGrid;
    fn options<'a>(&'a self) -> &'a BasicOpts<'f> {
        &self.opts
    }
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        self.opts.to_mut()
    }
    fn grid(&self) -> &SqGrid {
        self.grid
    }
    fn block_coords(&self, id: Index) -> <Self::G as Renderable>::B {
        UnsignedIntBlock::new(self.grid, id, &self.opts)
    }
}
