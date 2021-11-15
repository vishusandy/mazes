pub mod opts;
pub mod renderers;
pub use self::opts::BasicOpts;
use crate::maze::{Cell, Grid};
use crate::util::Index;
use image::{Rgb, Rgba, RgbaImage};
const DEJAVU_BYTES: &[u8] = include_bytes!("../../assets/DejaVuSansMono.ttf");

/// Provides methods for rendering and saving as well as setting options.
///
/// `Renderer` is separate from [`RenderOps`] to allow publicly re-exporting the `Renderer` trait without explicitly exposing [`RenderOps`] methods.
/// `Renderer` must also implement [`RenderOps`].
///
/// # Example
/// ```
/// use mazes::maze::Grid;
/// use mazes::render::Renderer;
/// let grid = mazes::maze::sq::SqGrid::new(4);
/// let mut renderer = grid.render_defaults();
/// let opts = renderer.options_mut();
/// opts.set_block_padding(100);
/// ```
pub trait Renderer<'f>: RenderOps {
    fn options(&self) -> &BasicOpts<'f>;
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f>;
    fn render_rgb(&self) -> RgbaImage {
        let opts = self.options();
        let output = self.render_grid();
        if let Some(scale) = opts.scale_image() {
            scale.scale(&output)
        } else {
            output
        }
    }
    fn save_render(&self, path: &std::path::Path) -> Result<(), image::ImageError> {
        self.render_rgb().save(path)
    }
}
//
pub trait RenderCell: Cell {
    fn cell(
        &self,
        block: ((usize, usize), (usize, usize)),
        text: Option<&str>,
        color: Rgb<u8>,
        image: &mut RgbaImage,
    );
}
#[derive(Clone, Debug)]
pub struct Block {
    id: Index,
    x: i32,
    y: i32,
    sx: u32,
    sy: u32,
}

/// Abstraction for the rendering of an individual block using a given grid type.
pub trait Renderable: Grid {
    fn render_block(
        &self,
        id: Index,
        text: &str,
        bg_color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        self.fill_block_bg(id, bg_color, image, opts);
        self.draw_block_outline(id, image, opts);
        self.draw_joint(id, image, opts);
        self.draw_block_text(id, text, opts.label_color(), image, opts);
    }
    fn draw_joint(&self, id: Index, image: &mut RgbaImage, opts: &BasicOpts);
    fn draw_block_outline(&self, id: Index, image: &mut RgbaImage, opts: &BasicOpts);
    fn fill_block_bg(&self, id: Index, color: &Rgba<u8>, image: &mut RgbaImage, opts: &BasicOpts);
    fn draw_block_text(
        &self,
        id: Index,
        text: &str,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    );
    fn image_dimensions(&self, opts: &BasicOpts) -> (u32, u32);
}
/// Abstraction of rendering options to be used with different rendering methods.
/// E.g., a map of distances (from a given starting point) will require changing the text label
/// and possibly the background color of a block (to visually show the distance).
pub trait RenderOps {
    type G: Grid + Renderable;
    fn block_label(&self, id: Index) -> String;
    fn block_bg(&self, id: Index) -> &Rgba<u8>;
    fn render_grid(&self) -> RgbaImage;
    fn grid(&self) -> &Self::G;
}
