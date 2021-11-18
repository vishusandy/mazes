use crate::maze::sq::SqGrid;
use crate::maze::Grid;
use crate::render::blocks::UnsignedIntBlock;
use crate::render::{BasicOpts, Renderable, Renderer, RendererOps};
use crate::util::dist::Distances;
use crate::util::Index;
use image::Rgba;
use log::trace;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct DistMapRenderer<'f, 'g, 'm, G: Grid + Renderable + Clone> {
    grid: &'g G,
    // todo: change this to a cow
    opts: BasicOpts<'f>,
    settings: DistMapOpts,
    map: Cow<'m, Distances<'g, G>>,
    max: usize,
}
impl<'f, 'g, 'm, G: Grid + Renderable + Clone> DistMapRenderer<'f, 'g, 'm, G> {
    pub(in crate) fn new(
        map: &'m Distances<'g, G>,
        opts: Option<BasicOpts<'f>>,
        settings: Option<DistMapOpts>,
    ) -> Self {
        Self {
            grid: map.grid(),
            opts: opts.unwrap_or_default(),
            settings: settings.unwrap_or_default(),
            max: map.max_dist().1,
            map: Cow::Borrowed(map),
        }
    }
}
impl<'f, 'o, 'g, 'm> Renderer<'f> for DistMapRenderer<'f, 'g, 'm, SqGrid> {}
impl<'f, 'o, 'g, 'm> RendererOps<'f> for DistMapRenderer<'f, 'g, 'm, SqGrid> {
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
            calc_bg(dist, self.max as f32, &self.settings.mask)
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
pub struct DistMapOpts {
    pub mask: DistMask,
    pub text: DistText,
}
impl DistMapOpts {
    pub fn mask(&self) -> &DistMask {
        &self.mask
    }
    pub fn mask_mut(&mut self) -> &mut DistMask {
        &mut self.mask
    }
    pub fn show_id(&mut self) {
        self.text = DistText::Id;
    }
    pub fn show_dist(&mut self) {
        self.text = DistText::Dist;
    }
}
impl Default for DistMapOpts {
    fn default() -> Self {
        Self {
            mask: DistMask::default(),
            text: DistText::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DistMask {
    r: bool,
    g: bool,
    b: bool,
    a: u8,
}
impl Default for DistMask {
    fn default() -> Self {
        Self {
            r: true,
            g: false,
            b: false,
            a: 255,
        }
    }
}
impl DistMask {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn none() -> Self {
        Self {
            r: false,
            g: false,
            b: false,
            a: 255,
        }
    }
    pub fn tuple(&self) -> (f32, f32, f32, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }
    /// Turn on or off the red channel
    pub fn use_red(&mut self, value: bool) {
        self.r = value;
    }
    pub fn only_red() -> Self {
        Self {
            r: true,
            ..Self::none()
        }
    }
    /// Get the value of the red channel as a f32 (as f32 will be used for calculations).
    fn r(&self) -> f32 {
        u8::from(self.r).into()
    }
    /// Turn on or off the green channel
    pub fn use_green(&mut self, value: bool) {
        self.g = value;
    }
    pub fn only_green() -> Self {
        Self {
            g: true,
            ..Self::none()
        }
    }
    /// Get the value of the green channel as a f32 (as f32 will be used for calculations).
    fn g(&self) -> f32 {
        u8::from(self.g).into()
    }
    /// Turn on or off the blue channel
    pub fn use_blue(&mut self, value: bool) {
        self.b = value;
    }
    pub fn only_blue() -> Self {
        Self {
            b: true,
            ..Self::none()
        }
    }
    /// Get the value of the blue channel as a f32 (as f32 will be used for calculations).
    fn b(&self) -> f32 {
        u8::from(self.b).into()
    }
    /// Specify an alpha channel value
    pub fn set_alpha(&mut self, value: u8) {
        self.a = value;
    }
    /// Get the value of the alpha channel
    fn a(&self) -> u8 {
        self.a
    }
}
pub enum RgbChannel {
    R,
    G,
    B,
}
impl std::ops::Index<RgbChannel> for DistMask {
    type Output = bool;
    fn index(&self, index: RgbChannel) -> &Self::Output {
        match index {
            RgbChannel::R => &self.r,
            RgbChannel::G => &self.g,
            RgbChannel::B => &self.b,
        }
    }
}
#[derive(Clone, Debug)]
pub enum DistText {
    Id,
    Dist,
}
impl DistText {
    pub fn id() -> Self {
        Self::Id
    }
    pub fn dist() -> Self {
        Self::Dist
    }
}
impl Default for DistText {
    fn default() -> Self {
        Self::Dist
    }
}

#[allow(clippy::many_single_char_names)]
pub(in crate) fn calc_bg(dist: usize, max: f32, mask: &DistMask) -> Rgba<u8> {
    let (r, g, b, a) = mask.tuple();
    let i = (max - dist as f32) / max;
    let calc = |c: f32| -> u8 { ((255f32 - c * 127.0) * i + c * 127.0) as u8 };
    Rgba([calc(r), calc(g), calc(b), a])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::maze::sq::tests::new_maze;
    use crate::maze::Grid;
    use crate::render::{BasicOpts, Renderer};
    use crate::util::Index;
    #[test]
    fn render_new_maze() -> Result<(), image::ImageError> {
        let path = std::path::Path::new("winding.png");
        new_maze(5).render_defaults().save_render(path)
    }
    #[test]
    fn dist_map_renderer_defaults() -> Result<(), image::ImageError> {
        new_maze(5)
            .distances(Index::zero())
            .render_defaults()
            .save_render(std::path::Path::new("sq_distances.png"))
    }
    #[test]
    fn dist_renderer() -> Result<(), image::ImageError> {
        let basic = BasicOpts::default();
        let mut mask = DistMask::only_blue();
        mask.set_alpha(127);
        let dist_opts = DistMapOpts {
            mask,
            text: DistText::id(),
        };
        new_maze(5)
            .distances(Index::zero())
            .render_options(Some(basic), Some(dist_opts))
            .save_render(std::path::Path::new("sq_distances_options.png"))
    }
}
