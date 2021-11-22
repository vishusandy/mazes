use crate::render::{BasicOpts, Renderable, Renderer, RendererOps};
use crate::util::Index;
use image::{Rgba, RgbaImage};
use std::collections::HashMap;

pub type CustomFunc<R: Renderable> =
    fn(&R, Index, &<<R as RendererOps>::G as Renderable>::B, &mut RgbaImage);

pub struct CustomRenderer<
    'f,
    R: Renderer<'f>,
    F: Fn(&R, Index, &<<R as RendererOps>::G as Renderable>::B, &mut RgbaImage),
> {
    bg: HashMap<Index, Rgba<u8>>,
    label: HashMap<Index, String>,
    extra: Option<F>,
    renderer: R,
    __phantom: std::marker::PhantomData<&'f R>,
}
impl<'f, R: Renderer<'f>, F> CustomRenderer<'f, R, F>
where
    F: Fn(&R, Index, &<<R as RendererOps>::G as Renderable>::B, &mut RgbaImage),
{
    pub fn new(
        renderer: R,
        bg: HashMap<Index, Rgba<u8>>,
        label: HashMap<Index, String>,
        extra: Option<F>,
    ) -> Self {
        Self {
            bg,
            label,
            extra,
            renderer,
            __phantom: std::marker::PhantomData,
        }
    }
}

impl<'f, R, F> Renderer<'f> for CustomRenderer<'f, R, F>
where
    R: Renderer<'f>,
    F: Fn(&R, Index, &<<R as RendererOps>::G as Renderable>::B, &mut RgbaImage),
{
}
impl<'f, R, F> RendererOps<'f> for CustomRenderer<'f, R, F>
where
    R: Renderer<'f>,
    F: Fn(&R, Index, &<<R as RendererOps>::G as Renderable>::B, &mut RgbaImage),
{
    type G = R::G;
    fn options<'a>(&'a self) -> &'a BasicOpts<'f> {
        self.renderer.options()
    }
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        self.renderer.options_mut()
    }
    fn grid(&self) -> &Self::G {
        self.renderer.grid()
    }
    fn block_coords(&self, id: Index) -> <Self::G as Renderable>::B {
        self.renderer.block_coords(id)
    }
    fn block_label(&self, id: Index) -> String {
        self.label
            .get(&id)
            .cloned()
            .unwrap_or_else(|| self.renderer.block_label(id))
    }
    fn block_bg(&self, id: Index) -> Rgba<u8> {
        self.bg
            .get(&id)
            .copied()
            .unwrap_or_else(|| self.renderer.block_bg(id))
    }
    fn render_extra(&self, id: Index, block: &<Self::G as Renderable>::B, image: &mut RgbaImage) {
        if let Some(f) = &self.extra {
            f(&self.renderer, id, block, image);
        }
    }
}
