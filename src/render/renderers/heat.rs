use crate::maze::Grid;
use crate::render::{BasicOpts, RenderOps};
use crate::util::Index;
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct HeatmapRenderer<'f, 'g, 'm, G: Grid + RenderOps> {
    grid: &'g G,
    opts: BasicOpts<'f>,
    settings: HeatMapOpts,
    map: &'m HashMap<Index, usize>,
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
    pub fn red(self, value: bool) -> Self {
        Self { r: value, ..self }
    }
    fn r(&self) -> u8 {
        self.r.into()
    }
    pub fn green(self, value: bool) -> Self {
        Self { g: value, ..self }
    }
    fn g(&self) -> u8 {
        self.g.into()
    }
    pub fn blue(self, value: bool) -> Self {
        Self { b: value, ..self }
    }
    fn b(&self) -> u8 {
        self.b.into()
    }
    pub fn alpha(self, value: bool) -> Self {
        Self { a: value, ..self }
    }
    fn a(&self) -> u8 {
        self.b.into()
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
