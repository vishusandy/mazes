use crate::render::DEJAVU_BYTES;
use image::{Rgba, RgbaImage};
use rusttype::Font;
#[derive(Clone, Debug)]
pub enum ScaleImage {
    Triangle(f32),
    CatmullRom(f32),
    Gaussian(f32),
    Lanczos3(f32),
    None,
}
use image::imageops::{resize, FilterType};
impl ScaleImage {
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
    pub fn high_quality(scale: f32) -> Self {
        Self::Lanczos3(scale)
    }
    pub fn medium_quality(scale: f32) -> Self {
        Self::CatmullRom(scale)
    }
}
#[derive(Clone, Debug)]
pub struct BasicOpts<'f> {
    /// Background color of the grid (inside of frame)
    bg_color: Rgba<u8>,
    /// Not sure if this will be used yet :/
    fg_color: Rgba<u8>,
    /// Color of joint intersections (if `show_joints` is `true`)
    joint_color: Rgba<u8>,
    /// Text color fof the labels
    label_color: Rgba<u8>,
    frame_color: Rgba<u8>,
    block_color: Rgba<u8>,
    border_color: Rgba<u8>,
    frame_size: u32,
    block_size: u32,
    /// Block padding applies to joints, arrows, and text.  Does not apply to borders.
    block_padding: u32,
    joint_size: u32,
    show_joints: bool,
    text_labels: bool,
    center_labels: bool,
    label_offset: u32,
    font: Font<'f>,
    font_size: f32,
    font_x: f32,
    font_y: f32,
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
            frame_size: 20,
            block_size: 70,
            block_padding: 10,
            joint_size: 6,
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
    pub fn debug() -> Self {
        Self {
            bg_color: Rgba([255u8, 255u8, 255u8, 255u8]),
            fg_color: Rgba([50u8, 50u8, 50u8, 255u8]),
            joint_color: Rgba([0u8, 192u8, 0u8, 255u8]),
            label_color: Rgba([100u8, 100u8, 100u8, 255u8]),
            frame_color: Rgba([192u8, 192u8, 192u8, 255u8]),
            block_color: Rgba([220u8, 220u8, 220u8, 255u8]),
            border_color: Rgba([192u8, 0u8, 0u8, 255u8]),
            show_joints: true,
            text_labels: true,
            center_labels: true,
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
    pub fn show_joints(&self) -> bool {
        self.show_joints
    }
    pub fn text_labels(&self) -> bool {
        self.text_labels
    }
    pub fn center_labels(&self) -> bool {
        self.center_labels
    }
    pub fn label_offset(&self) -> u32 {
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
    pub fn set_show_joints(&mut self, show: bool) {
        self.show_joints = show;
    }
    pub fn set_text_labels(&mut self, show: bool) {
        self.text_labels = show;
    }
    pub fn set_center_labels(&mut self, value: bool) {
        self.center_labels = value;
    }
    pub fn set_label_offset(&mut self, size: u32) {
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
}
