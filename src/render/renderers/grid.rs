use crate::maze::sq::SqGrid;
use crate::maze::Grid;
use crate::render::blocks::UnsignedIntBlock;
use crate::render::renderers::anim::{AnimOpts, Animation};
use crate::render::{BasicOpts, Renderable, Renderer, RendererOps};
use crate::util::Index;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct RenderGrid<'f, 'o, 'g, G: Grid + Renderable> {
    grid: &'g G,
    opts: Cow<'o, BasicOpts<'f>>,
}
impl<'f, 'o, 'g> Renderer<'f> for RenderGrid<'f, 'o, 'g, SqGrid> {}
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
    pub fn animation<'a>(self, anim: Option<&'a AnimOpts>) -> Animation<'a, 'f, Self> {
        Animation::new(self, anim)
    }
    pub fn animation_defaults<'a>(self) -> Animation<'a, 'f, Self> {
        Animation::new(self, None)
    }
    pub fn animation_options<'a>(self, anim: &'a AnimOpts) -> Animation<'a, 'f, Self> {
        Animation::new(self, Some(anim))
    }
}
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
