use crate::render::DEJAVU_BYTES;
use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};
#[derive(Clone, Debug)]
/// If resizing the image is desired [`ScaleImage`] will resize the image using
/// with a given scaling factor (an `f32`) and
/// [`image::imageops::FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html).
///
/// For more information see
/// [`image::imageops::resize()`](https://docs.rs/image/latest/image/imageops/fn.resize.html)
pub enum ScaleImage {
    Triangle(f32),
    CatmullRom(f32),
    Gaussian(f32),
    Lanczos3(f32),
    None,
}
use image::imageops::{resize, FilterType};
impl ScaleImage {
    /// Calls [`image::imageops::resize()`](https://docs.rs/image/latest/image/imageops/fn.resize.html)
    /// with a given scaling factor and
    /// [`image::imageops::FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html).
    pub(in crate) fn scale(&self, image: &RgbaImage) -> RgbaImage {
        let scale_with = |s: f32, f: FilterType| -> RgbaImage {
            let h = ((image.height() as f32) * s) as u32;
            let w = ((image.width() as f32) * s) as u32;
            resize(image, h, w, f)
        };
        match self {
            Self::Triangle(s) => scale_with(*s, FilterType::Triangle),
            Self::CatmullRom(s) => scale_with(*s, FilterType::CatmullRom),
            Self::Gaussian(s) => scale_with(*s, FilterType::Gaussian),
            Self::Lanczos3(s) => scale_with(*s, FilterType::Lanczos3),
            Self::None => image.clone(),
        }
    }
    /// Use the highest quality, and slowest, resizing algorithm
    pub fn high_quality(scale: f32) -> Self {
        Self::Lanczos3(scale)
    }
    /// Use a medium quality resizing algorithm with decent performance.
    pub fn medium_quality(scale: f32) -> Self {
        Self::CatmullRom(scale)
    }
}
#[derive(Clone, Debug)]
pub struct BasicOpts<'f> {
    /// Background color of the grid (inside of frame).
    bg_color: Rgba<u8>,
    /// Not sure if this will be used yet :/
    fg_color: Rgba<u8>,
    /// Color of joint intersections (if `show_joints` is `true`).
    joint_color: Rgba<u8>,
    /// Text color fof the labels.
    label_color: Rgba<u8>,
    /// Color for the image frame.
    frame_color: Rgba<u8>,
    /// The default background color for grid blocks.
    block_color: Rgba<u8>,
    /// The color of a block's borders.
    border_color: Rgba<u8>,
    border_width: u32,
    /// How much padding should be added around the grid.
    /// This is used to increase visibility of the outside grid borders, which may be hard to see
    /// when using an image viewer with a background color similar to the border color.
    frame_size: u32,
    /// The default size for each block.  For non-square cells this will be used to scale the block
    /// to ensure the cell is no larger than the given value.
    block_size: u32,
    /// Block padding applies to joints, arrows, and text.  Does not apply to borders.
    block_padding: u32,
    /// The length of each section of a joint.  The actual size of each joint will be double this value.
    joint_size: u32,
    /// When `true` this will draw joints with pointed edges instead of square.
    /// Using `tri_joints` is only visibile when `border_width > 1`.
    ///
    /// Note: when set to `true` joints will use `joint_width` as a minimum length, and will
    /// increase in size until the length is equal to `joint_size + border_width`
    tri_joints: bool,
    /// Whether to draw joints or not
    show_joints: bool,
    /// Whether to draw text labels or not
    text_labels: bool,
    /// Whether to center text or not.
    center_labels: bool,
    /// An offset applied to the x and y position of text labels.
    ///
    /// When using `center_labels=true` a good default value for `label_offset` is `-5`.
    /// This value may need to be adjusted depending on the renderer, font scaling, and font size used.
    label_offset: i32,
    /// Font to use when rendering text labels.
    ///
    /// See [rusttype](https://docs.rs/crate/rusttype) for more information.
    font: Font<'f>,
    /// This determines the size of the font when rendering.  See [rusttype] for more information.
    font_size: f32,
    /// This determines the horizontal scaling of the rendered font.
    ///
    /// For more information see
    /// [rusttype::Scale](https://docs.rs/rusttype/latest/rusttype/struct.Scale.html).
    font_x: f32,
    /// This determines the vertical scaling of the rendered font.  
    ///
    /// For more information see
    /// [rusttype::Scale](https://docs.rs/rusttype/latest/rusttype/struct.Scale.html).
    font_y: f32,
    /// If resizing the image is desired [`ScaleImage`] will resize the image using
    /// with a given scaling factor and
    /// [`image::imageops::FilterType`](https://docs.rs/image/latest/image/imageops/enum.FilterType.html).
    ///
    /// For more information see
    /// [`image::imageops::resize()`](https://docs.rs/image/latest/image/imageops/fn.resize.html)
    scale_image: Option<ScaleImage>,
}
impl<'f> Default for BasicOpts<'f> {
    fn default() -> Self {
        Self {
            bg_color: Rgba([255u8, 255u8, 255u8, 255u8]),
            fg_color: Rgba([50u8, 50u8, 50u8, 255u8]),
            joint_color: Rgba([0u8, 0u8, 0u8, 255u8]),
            label_color: Rgba([100u8, 100u8, 100u8, 255u8]),
            frame_color: Rgba([192u8, 192u8, 192u8, 255u8]),
            block_color: Rgba([220u8, 220u8, 220u8, 255u8]),
            border_color: Rgba([0u8, 0u8, 0u8, 255u8]),
            border_width: 1,
            frame_size: 40,
            block_size: 70,
            block_padding: 5,
            joint_size: 6,
            tri_joints: false,
            show_joints: true,
            text_labels: true,
            center_labels: true,
            label_offset: 8,
            font: Font::try_from_bytes(DEJAVU_BYTES).unwrap(),
            font_size: 15.2f32,
            font_x: 15.2f32,
            font_y: 15.2f32,
            scale_image: None,
        }
    }
}
impl<'f> BasicOpts<'f> {
    /// Settings to use when debugging a renderer (e.g., to more easily see if borders and joints are properly drawn)
    pub fn debug() -> Self {
        Self {
            bg_color: Rgba([192u8, 192u8, 255u8, 255u8]),
            fg_color: Rgba([50u8, 50u8, 50u8, 255u8]),
            joint_color: Rgba([0u8, 192u8, 0u8, 255u8]),
            label_color: Rgba([100u8, 100u8, 100u8, 255u8]),
            frame_color: Rgba([201u8, 239u8, 255u8, 255u8]),
            block_color: Rgba([220u8, 220u8, 220u8, 255u8]),
            border_color: Rgba([192u8, 0u8, 0u8, 255u8]),
            tri_joints: true,
            show_joints: true,
            joint_size: 10,
            block_size: 70,
            text_labels: true,
            center_labels: true,
            label_offset: 0,
            ..Self::default()
        }
    }
    pub fn bg_color(&self) -> &Rgba<u8> {
        &self.bg_color
    }
    pub fn fg_color(&self) -> &Rgba<u8> {
        &self.fg_color
    }
    pub fn joint_color(&self) -> &Rgba<u8> {
        &self.joint_color
    }
    pub fn label_color(&self) -> &Rgba<u8> {
        &self.label_color
    }
    pub fn frame_color(&self) -> &Rgba<u8> {
        &self.frame_color
    }
    pub fn block_color(&self) -> &Rgba<u8> {
        &self.block_color
    }
    pub fn border_color(&self) -> &Rgba<u8> {
        &self.border_color
    }
    pub fn border_width(&self) -> u32 {
        self.border_width
    }
    pub fn frame_size(&self) -> u32 {
        self.frame_size
    }
    pub fn block_size(&self) -> u32 {
        self.block_size
    }
    pub fn block_padding(&self) -> u32 {
        self.block_padding
    }
    pub fn joint_size(&self) -> u32 {
        self.joint_size
    }
    pub fn tri_joints(&self) -> bool {
        self.tri_joints
    }
    pub fn show_joints(&self) -> bool {
        self.show_joints
    }
    pub fn text_labels(&self) -> bool {
        self.text_labels
    }
    pub fn center_labels(&self) -> bool {
        self.center_labels
    }
    pub fn label_offset(&self) -> i32 {
        self.label_offset
    }
    pub fn font(&self) -> &Font<'f> {
        &self.font
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_x(&self) -> f32 {
        self.font_x
    }
    pub fn font_y(&self) -> f32 {
        self.font_y
    }
    pub fn scale_image(&self) -> &Option<ScaleImage> {
        &self.scale_image
    }
    pub fn set_bg_color(&mut self, color: Rgba<u8>) {
        self.bg_color = color;
    }
    pub fn set_fg_color(&mut self, color: Rgba<u8>) {
        self.fg_color = color;
    }
    pub fn set_joint_color(&mut self, color: Rgba<u8>) {
        self.joint_color = color;
    }
    pub fn set_label_color(&mut self, color: Rgba<u8>) {
        self.label_color = color;
    }
    pub fn set_frame_color(&mut self, color: Rgba<u8>) {
        self.frame_color = color;
    }
    pub fn set_block_color(&mut self, color: Rgba<u8>) {
        self.block_color = color;
    }
    pub fn set_border_color(&mut self, color: Rgba<u8>) {
        self.border_color = color;
    }
    pub fn set_border_width(&mut self, size: u32) {
        self.border_width = size;
    }
    pub fn set_frame_size(&mut self, size: u32) {
        self.frame_size = size;
    }
    pub fn set_block_size(&mut self, size: u32) {
        self.block_size = size;
    }
    pub fn set_block_padding(&mut self, size: u32) {
        self.block_padding = size;
    }
    pub fn set_joint_size(&mut self, size: u32) {
        self.joint_size = size;
    }
    pub fn set_tri_joints(&mut self, show: bool) {
        self.tri_joints = show;
    }
    pub fn set_show_joints(&mut self, show: bool) {
        self.show_joints = show;
    }
    pub fn set_text_labels(&mut self, show: bool) {
        self.text_labels = show;
    }
    pub fn set_center_labels(&mut self, value: bool) {
        self.center_labels = value;
    }
    pub fn set_label_offset(&mut self, size: i32) {
        self.label_offset = size;
    }
    pub fn set_font(&mut self, font: Font<'f>) {
        self.font = font;
    }
    pub fn set_font_size(&mut self, size: f32) {
        self.font_size = size;
    }
    pub fn set_font_x(&mut self, scale: f32) {
        self.font_x = scale;
    }
    pub fn set_font_y(&mut self, scale: f32) {
        self.font_y = scale;
    }
    pub fn set_scale_image(&mut self, scaler: Option<ScaleImage>) {
        self.scale_image = scaler;
    }
    pub fn font_scale(&self) -> Scale {
        Scale {
            x: self.font_x(),
            y: self.font_y(),
        }
    }
}
