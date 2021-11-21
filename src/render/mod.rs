pub mod blocks;
pub mod opts;
pub mod renderers;
pub use self::opts::BasicOpts;
use crate::maze::{Cell, Grid};
use crate::render::blocks::BlockCoords;
use crate::util::Index;
use image::{Rgb, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
const DEJAVU_BYTES: &[u8] = include_bytes!("../../assets/DejaVuSansMono.ttf");

/// `Renderer` enables different types of images to be generated using the same grid data.
/// For example, a [`SqGrid`] can be saved as a distance map, a path through the grid, or a normal
/// grid image.  All of those are examples of types implementing [`Renderer`].
///
/// A `Renderer` is a type that can render a grid image and is composed of at least a [`BasicOpts`]
/// and a type implementing [`Grid`].  The type will also need
/// to implement /// [`RendererOps`].  `Renderer` contains the methods that users should have
/// access to /// (e.g., `save_render()`) while [`RendererOps`] contains methods necessary for
/// rendering.  A type implementing `Renderer` must contain (or have access to) an instance of
/// [`BasicOpts`] as well as a [`Grid`] type.
///
/// `Renderer` is separate from [`RendererOps`] to allow publicly re-exporting the `Renderer` trait
/// without also publicly exporting [`RendererOps`] methods.
///
/// A `Renderer` must also implement [`RendererOps`].
///
/// # Example
/// ```
/// use mazes::maze::Grid;
/// use mazes::render::Renderer;
/// let grid = mazes::maze::sq::SqGrid::new(4);
/// let mut renderer = grid.render_defaults();
/// let opts = renderer.opts_mut();
/// opts.set_block_padding(100);
/// ```
pub trait Renderer<'f>: RendererOps<'f> {
    fn opts(&self) -> &BasicOpts<'f> {
        <Self as RendererOps>::options(self)
    }
    fn opts_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f> {
        self.options_mut()
    }
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
///
/// These methods will be implemented on a [`Grid`] type and enable [`Renderer`]s to work work with
/// grids in a generic way.
pub trait Renderable: Grid {
    type B: BlockCoords;
    fn render_block(
        &self,
        id: Index,
        block: &Self::B,
        text: &str,
        bg_color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    ) {
        self.fill_block_bg(id, block, bg_color, image, opts);
        self.draw_block_outline(id, block, image, opts);
        if opts.show_joints() {
            self.draw_joint(id, block, image, opts);
        }
        self.draw_block_text(id, block, text, opts.label_color(), image, opts);
    }
    fn draw_joint(&self, id: Index, block: &Self::B, image: &mut RgbaImage, opts: &BasicOpts);
    fn draw_block_outline(
        &self,
        id: Index,
        block: &Self::B,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    );
    fn fill_block_bg(
        &self,
        id: Index,
        block: &Self::B,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    );
    fn draw_block_text(
        &self,
        id: Index,
        block: &Self::B,
        text: &str,
        color: &Rgba<u8>,
        image: &mut RgbaImage,
        opts: &BasicOpts,
    );
    fn image_dimensions(&self, opts: &BasicOpts) -> (u32, u32);
}
/// Abstraction of rendering options to be used with different rendering methods.
/// E.g., a map of distances (from a given starting point) will require changing the text label
/// and possibly the background color of a block (to visually show the distance), whereas rendering
/// a grid can just use the default trait implementations for `block_label()` and `block_bg()`.
pub trait RendererOps<'f> {
    type G: Grid + Renderable;
    fn options<'a>(&'a self) -> &'a BasicOpts<'f>;
    fn options_mut<'a>(&'a mut self) -> &'a mut BasicOpts<'f>;
    fn grid(&self) -> &Self::G;
    // fn block_coords(&self, id: Index) -> <<Self as RendererOps<'f>>::G as GridProps>::B;
    fn block_coords(&self, id: Index) -> <Self::G as Renderable>::B;
    fn block_label(&self, id: Index) -> String {
        id.to_string()
    }
    fn block_bg(&self, _id: Index) -> Rgba<u8> {
        *self.options().block_color()
    }
    fn render_extra(
        &self,
        _id: Index,
        _block: &<Self::G as Renderable>::B,
        _image: &mut RgbaImage,
    ) {
    }
    fn render_frame(&self) -> RgbaImage {
        let opts = self.options();
        let (x, y) = self.grid().image_dimensions(opts);
        let frame = opts.frame_size();
        let color = if frame == 0 {
            opts.bg_color()
        } else {
            opts.frame_color()
        };
        let mut image = RgbaImage::from_pixel(x, y, *color);
        if frame != 0 {
            let rect = Rect::at((opts.frame_size()) as i32, (opts.frame_size()) as i32)
                .of_size(x - (frame * 2), y - (frame * 2));
            draw_filled_rect_mut(&mut image, rect, *opts.bg_color());
        }
        image
    }
    fn render_grid(&self) -> RgbaImage {
        let mut image = self.render_frame();
        self.render_grid_blocks(&mut image);
        image
    }
    fn render_grid_blocks(&self, image: &mut RgbaImage) {
        for i in self.grid().iter() {
            let id = i.id();
            let block = self.block_coords(id);
            self.grid().render_block(
                id,
                &block,
                &self.block_label(id),
                &self.block_bg(id),
                image,
                self.options(),
            );
            self.render_extra(id, &block, image);
        }
    }
}
