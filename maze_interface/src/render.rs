use crate::{
    coords::RectCoords, maze_cell::MazeCell, maze_grid::MazeGrid, points::Point, Edge,
    FlatLocation, Location,
};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_line_segment_mut;

pub struct Render {
    /// The amount of space each cell should take up, excluding its border.
    cell_size: u16,
    /// The amount of padding around the outside of the grid border.
    padding: u8,
}

impl Render {
    pub fn new(cell_size: u16, padding: u8) -> Render {
        Render { cell_size, padding }
    }
    // #[inline]
    pub fn points_f32(&self, cell: &MazeCell, length: u8) -> (f32, f32, f32, f32) {
        let padding = self.padding as u32;
        let cell_size = self.cell_size;
        let size = self.cell_size as u32;

        let idx = cell.idx();
        let fidx = idx * 2;

        let row = cell.y();
        let col = cell.x();

        let r = row as u32;
        let c = col as u32;

        let xw = c * size + padding + 1 + c;
        let xe = xw + size + 1;
        let yn = r * size + padding + 1 + r;
        let ys = yn + size + 1;

        (xw as f32, xe as f32, yn as f32, ys as f32)
    }

    pub fn points_u32(&self, cell: &MazeCell, length: u8) -> (u32, u32, u32, u32) {
        let padding = self.padding as u32;
        let cell_size = self.cell_size;
        let size = self.cell_size as u32;

        let idx = cell.idx();
        let fidx = idx * 2;

        let row = cell.y();
        let col = cell.x();

        let r = row as u32;
        let c = col as u32;

        let xw = c * size + padding + 1 + c;
        let xe = xw + size + 1;
        let yn = r * size + padding + r + if r == 0 { 0 } else { 1 };
        let ys = yn + size + 1;

        (xw, xe, yn, ys)
    }

    pub fn corners_f32(&self, cell: &MazeCell, length: u8) -> crate::coords::RectCoords<f32> {
        unimplemented!() // TODO
    }
    pub fn corners_u32(&self, cell: &MazeCell, length: u8) -> crate::coords::RectCoords<u32> {
        unimplemented!() // TODO
    }

    pub fn cell_points_f32(&self, cell: &MazeCell, length: u8) -> (f32, f32, f32, f32) {
        let padding = self.padding as u32;
        let cell_size = self.cell_size;
        let size = cell_size as u32;

        let idx: u16 = cell.idx();
        let fidx: u16 = (idx as u16) * 2;

        let row: u16 = fidx / (length as u16);
        let col: u16 = idx * 2 + if row % 2 == 0 { 0 } else { 1 };
        let r: u32 = row as u32;
        let c: u32 = col as u32;

        let x_w = ((c * size) + padding + 1 + r) as f32;
        let x_e = x_w + ((cell_size + 1) as f32);
        let y_n = ((r * size) + padding + 1 + r) as f32;
        let y_s = y_n + ((cell_size + 1) as f32);
        (x_w, x_e, y_n, y_s)
    }
    // #[inline]
    pub fn cell_points_u32(&self, cell: &MazeCell, length: u8) -> (u32, u32, u32, u32) {
        let padding = self.padding as u32;
        let cell_size = self.cell_size;
        let size = cell_size as u32;

        let idx: u16 = cell.idx();
        let fidx: u16 = (idx as u16) * 2;

        let row: u16 = fidx / (length as u16);
        let col: u16 = idx * 2 + if row % 2 == 0 { 0 } else { 1 };
        let r: u32 = row as u32;
        let c: u32 = col as u32;

        let x_w = (c * size) + padding + 1 + r;
        let x_e = x_w + (cell_size as u32 + 1);
        let y_n = (r * size) + padding + 1 + r;
        let y_s = y_n + (cell_size as u32 + 1);
        (x_w, x_e, y_n, y_s)
    }
    pub fn cell_corners_f32(&self, cell: &MazeCell, length: u8) -> crate::coords::RectCoords<f32> {
        let (x_w, x_e, y_n, y_s) = self.cell_points_f32(cell, length);

        let nw = (x_w, y_n);
        let ne = (x_e, y_n);
        let se = (x_e, y_s);
        let sw = (x_w, y_s);
        RectCoords::new(nw, ne, se, sw)
    }
    pub fn cell_corners_u32(&self, cell: &MazeCell, length: u8) -> crate::coords::RectCoords<u32> {
        let (x_w, x_e, y_n, y_s) = self.cell_points_u32(cell, length);

        let nw = (x_w, y_n);
        let ne = (x_e, y_n);
        let se = (x_e, y_s);
        let sw = (x_w, y_s);
        RectCoords::new(nw, ne, se, sw)
    }
}
