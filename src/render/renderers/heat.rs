use crate::maze::sq::SqGrid;
use crate::maze::Grid;
use crate::render::blocks::UnsignedIntBlock;
use crate::render::{BasicOpts, Renderable, RendererOps};
use crate::util::Index;
use image::Rgba;
use log::trace;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HeatMap<'g, G: Grid + Renderable> {
    grid: &'g G,
    map: HashMap<Index, usize>,
    max: usize,
}
impl<'g, G: Grid + Renderable> HeatMap<'g, G> {
    pub fn dijkstra_simple(grid: &'g G, start: Index) -> Self {
        todo!()
    }
    pub fn get(&self, id: Index) -> Option<usize> {
        self.map.get(&id).copied()
    }
    pub fn max(&self) -> usize {
        self.max
    }
}

#[derive(Clone, Debug)]
pub struct HeatmapRenderer<'f, 'g, 'm, G: Grid + Renderable + Clone> {
    grid: &'g G,
    opts: BasicOpts<'f>,
    settings: HeatMapOpts,
    map: Cow<'m, HeatMap<'g, G>>,
}
impl<'f, 'g, 'm> RendererOps<'f> for HeatmapRenderer<'f, 'g, 'm, SqGrid> {
    type G = SqGrid;
    fn options<'a>(&'a self) -> &'a BasicOpts<'f> {
        &self.opts
    }
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        &mut self.opts
    }
    fn block_label(&self, id: Index) -> String {
        self.map.get(id).map(|i| i.to_string()).unwrap_or_default()
    }
    #[allow(clippy::many_single_char_names)]
    fn block_bg(&self, id: Index) -> Rgba<u8> {
        if let Some(dist) = self.map.get(id) {
            let (r, g, b, a) = self.settings.mask.tuple();
            let max = self.map.max() as f32;
            let i = (max - dist as f32) / max;
            let calc = |c: f32| -> u8 { ((255f32 - c) * i + c) as u8 };
            Rgba([calc(r), calc(g), calc(b), calc(a)])
        } else {
            trace!("Could not find distance for id={}", id);
            *self.opts.block_color()
        }
    }
    fn grid(&self) -> &Self::G {
        self.grid
    }
    fn block_coords(&self, id: Index) -> <Self::G as Renderable>::B {
        UnsignedIntBlock::new(self.grid, id, &self.opts)
    }
}

#[derive(Clone, Debug)]
pub struct HeatMapOpts {
    mask: HeatMask,
    text: HeatText,
}
#[derive(Clone, Debug)]
pub struct HeatMask {
    r: bool,
    g: bool,
    b: bool,
    a: bool,
}
impl HeatMask {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tuple(&self) -> (f32, f32, f32, f32) {
        (self.r(), self.g(), self.b(), self.a())
    }
    /// Turn on or off the red channel
    pub fn red(self, value: bool) -> Self {
        Self { r: value, ..self }
    }
    /// Get the value of the red channel as a f32 (as f32 will be used for calculations).
    fn r(&self) -> f32 {
        u8::from(self.r).into()
    }
    /// Turn on or off the green channel
    pub fn green(self, value: bool) -> Self {
        Self { g: value, ..self }
    }
    /// Get the value of the green channel as a f32 (as f32 will be used for calculations).
    fn g(&self) -> f32 {
        u8::from(self.g).into()
    }
    /// Turn on or off the blue channel
    pub fn blue(self, value: bool) -> Self {
        Self { b: value, ..self }
    }
    /// Get the value of the blue channel as a f32 (as f32 will be used for calculations).
    fn b(&self) -> f32 {
        u8::from(self.b).into()
    }
    /// Turn on or off the alpha channel
    pub fn alpha(self, value: bool) -> Self {
        Self { a: value, ..self }
    }
    /// Get the value of the alpha channel as a f32 (as f32 will be used for calculations).
    fn a(&self) -> f32 {
        u8::from(self.b).into()
    }
}
impl Default for HeatMask {
    fn default() -> Self {
        Self {
            r: true,
            g: false,
            b: false,
            a: false,
        }
    }
}
pub enum RgbaChannel {
    R,
    G,
    B,
    A,
}
impl std::ops::Index<RgbaChannel> for HeatMask {
    type Output = bool;
    fn index(&self, index: RgbaChannel) -> &Self::Output {
        match index {
            RgbaChannel::R => &self.r,
            RgbaChannel::G => &self.g,
            RgbaChannel::B => &self.b,
            RgbaChannel::A => &self.a,
        }
    }
}
#[derive(Clone, Debug)]
pub enum HeatText {
    Id,
    Dist,
}
impl Default for HeatText {
    fn default() -> Self {
        Self::Dist
    }
}
