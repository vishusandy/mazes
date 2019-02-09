use crate::maze_cell::MazeCell;
use crate::points::Point;
use crate::{Edge, FlatLocation, Location};

/// MazeGrid is primarily a Vector of MazeCells.  There are two good ways to
/// organize the cells in the vector.
///
/// 1. The layout is unoptimized, and every cell has a North, East, West, South
/// border.  This means that cells sharing a border will have to ensure the
/// neighboring border is changed to be the same, so if the top right cell has
/// its eastern border removed, the second cell in the top row must also have
/// its western border removed.  An alternative would to be to just ignore all
/// Western and Northern borders and rely on the neighbors below and/or east of
/// the current cell for shared border data.  Both approaches have an
/// inefficient memory layout.
///
/// 2. The cell grid would use an optimized layout so that every other cell is
/// empty, such that the zero index would be a MazeCell, but the second index
/// would merely refer to the MazeCell next to it (the one before hand for the
/// left, or the one next to it for the right).  The cells would be split into
/// rows; if the dimensions are 8x8 then the first 4 cells would represent the
/// 0, 2, 4, & 8th indicies on the top, the next 4 cells would be the 1, 3, 5,
/// 7th indicies of the second row, the next 4 cells would be the 0, 2, 4, 8th
/// indicies on the third row, the next 4 cells would be the 1, 3, 5, 7th
/// indicies on the fourth row, and this pattern will continue until the last
/// row is filled.  This approach will still have wasted memory but much less.
/// The amount of data wasted would be equal to: length plus one bit.  This is
/// twice as efficient in regards to memory size compared to the naive method.
#[derive(Clone, Debug)]
pub struct MazeGrid {
    length: u8,
    cells: Vec<MazeCell>,
}

impl MazeGrid {
    pub fn length(&self) -> u8 {
        self.length
    }

    pub fn cells(&self) -> &Vec<MazeCell> {
        &self.cells
    }

    pub fn modify(&mut self) -> &mut Vec<MazeCell> {
        &mut self.cells
    }

    /// Create a new square grid of a specified size.  The cells will be
    /// created with all of the borders visible, if this is not desired use
    /// MazeGrid::blank() instead.
    pub fn new(length: u8) -> Self {
        let mut cells: Vec<MazeCell> = Vec::with_capacity((length * length + 1) as usize);
        for y in 0..(length) {
            for x in 0..(length) {
                let idx = (y as u16) * (length as u16) + (x as u16);
                if idx % 2 == 0 {
                    let pt = Point::new(x as i16, y as i16);
                    cells.push(MazeCell::new(length, &pt, idx / 2));
                }
            }
        }
        Self { length, cells }
    }

    /// Create a new square grid of a specified size.  The cells will be
    /// created but none of the edges will be visible, if this is not desired
    /// use MazeGrid::new() instead.
    pub fn blank(length: u8) -> Self {
        let mut cells: Vec<MazeCell> = Vec::with_capacity((length * length + 1) as usize);
        for y in 0..(length) {
            for x in 0..(length) {
                let idx = (y as u16) * (length as u16) + (x as u16);
                if idx % 2 == 0 {
                    let pt = Point::new(x as i16, y as i16);
                    cells.push(MazeCell::blank(length, &pt, idx / 2));
                }
            }
        }
        Self { length, cells }
    }

    /// carve() removes a wall from a given side of a cell and returns true if
    /// changes were made, or false if no changes were made.
    pub fn carve(&mut self, cell: &Point, side: &Edge) -> bool {
        let loc = Location {
            side: (*side).clone(),
            point: cell.clone(),
        };
        let v_loc = self.locate(&loc);
        let FlatLocation { side, idx } = v_loc;
        let cell = &mut self.cells[idx as usize];
        cell.remove_edge(&side)
    }

    /// The locate() method is used to translate regular grid coordinates into
    /// the optimized virtual representation of the grid (wherein a four-sided
    /// cell is created for every other grid cell; only every other cell is
    /// used in order to optimize the memory layout and space, as a four-sided
    /// cell for each cell would result in wasted space as each cell would have
    /// shared borders with its neighbors).
    ///
    /// locate() takes a MazeGrid (self), and a Location and determines and
    /// returns the cell and new edge that stores the data for that edge.
    pub fn locate(&self, loc: &Location) -> FlatLocation {
        loc.locate(self.length)
    }

    /// Returns the size of the file written to disk
    pub fn save_png(
        &self,
        output_file: &std::path::Path,
        cell_size: u16,
        border_padding: u8,
    ) -> Option<usize> {
        use image::{Rgb, RgbImage};
        use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
        use imageproc::rect::Rect;
        // use imageproc::rect::{Rect, Region}; // Region could come in handy

        let rows = self.length;
        let padding = border_padding as u32;
        // Use the formula: rows*cell_size+1+(border*2)
        let img_size = ((rows as u32) * (cell_size as u32)) + (rows as u32) + 1 + padding + padding;
        let grid_length = img_size - padding - padding;

        #[allow(unused_mut)]
        let mut image = RgbImage::new(img_size as u32, img_size as u32);
        let fg = Rgb([255u8, 255u8, 255u8]);
        let bg = Rgb([0u8, 0u8, 0u8]);
        let rect = Rect::at(0, 0).of_size(img_size, img_size);
        draw_filled_rect_mut(&mut image, rect, bg);

        // Before adding 1 to the border grid lines on the top and left
        // let nw_grid_corner = (padding as f32, padding as f32);
        // let sw_grid_corner = (padding as f32, (img_size - padding) as f32);
        // let ne_grid_corner = (((img_size - padding) as f32), padding as f32);
        // let se_grid_corner = (((img_size - padding) as f32), (img_size - padding) as f32);
        let nw_grid_corner = ((padding + 1) as f32, (padding + 1) as f32);
        let sw_grid_corner = ((padding + 1) as f32, (img_size - padding) as f32);
        let ne_grid_corner = (((img_size - padding) as f32), (padding + 1) as f32);
        let se_grid_corner = (((img_size - padding) as f32), (img_size - padding) as f32);

        // Draw top border line
        draw_line_segment_mut(&mut image, nw_grid_corner, ne_grid_corner, fg);

        // Draw bottom border line
        draw_line_segment_mut(&mut image, sw_grid_corner, se_grid_corner, fg);

        // Draw left border line
        draw_line_segment_mut(&mut image, nw_grid_corner, sw_grid_corner, fg);

        // Draw right border line
        draw_line_segment_mut(&mut image, ne_grid_corner, se_grid_corner, fg);

        let mut row = 0u8;
        let mut col = 0u8;
        for cell in self.cells.iter() {
            // TODO: scatter the grid cells - every other cell is skipped and the
            // surrounding cells will fill the skipped cells' borders.

            // let nw_corner = ((col as u32) * (cell_size as u32)) + 1 + padding;
            let render = crate::render::Render::new(cell_size, border_padding);
            // let crate::coords::RectCoords { nw, ne, se, sw } =
            //     render.cell_corners_f32(&cell, self.length);
            // let x_w = nw.0;
            // let x_e = ne.0;
            // let y_n = nw.1;
            // let y_s = sw.1;

            let (x_w, x_e, y_n, y_s) = render.points_f32(cell, self.length);

            if cell.has_top_edge() {
                draw_line_segment_mut(&mut image, (x_w, y_n), (x_e, y_n), fg);
            }
            if cell.has_right_edge() {
                draw_line_segment_mut(&mut image, (x_e, y_n), (x_e, y_s), fg);
            }
            if cell.has_bottom_edge() {
                draw_line_segment_mut(&mut image, (x_w, y_s), (x_e, y_s), fg);
            }
            if cell.has_left_edge() {
                draw_line_segment_mut(&mut image, (x_w, y_n), (x_w, y_s), fg);
            }

            let center_x = x_w as u32 + 1 + (cell_size as u32 / 2);
            let center_y = y_n as u32 + 1 + (cell_size as u32 / 2);
            image.put_pixel(center_x, center_y, fg);

            // Increment or reset the column, and increment row if end of row
            col += 1;
            if col == rows {
                col = 0;
                row += 1;
            }
        }

        if let Ok(_) = image.save(output_file) {
            Some(0) // TODO: read in actual file size here
        } else {
            None
        }
    }
}

/// This operation will only return a reference to the cell containing
/// the data for the side in question, it will NOT tell you which side
/// of that reference you should use.  For this reason you should be
/// careful when using this method.  Use MazeGrid.locate() to get the
/// index and correct side.
impl std::ops::Index<&Location> for MazeGrid {
    type Output = MazeCell;

    /// This method will only return a reference to the cell containing
    /// the data for the side in question, it will NOT tell you which side
    /// of that reference you should use.  For this reason you should be
    /// careful when using this method.  Use MazeGrid.locate() to get the
    /// index and correct side.
    fn index(&self, loc: &Location) -> &MazeCell {
        // unimplemented!() // TODO
        let v_loc = self.locate(loc);
        &self.cells[v_loc.idx as usize]
    }
}

/// This operation will only return a reference to the cell containing
/// the data for the side in question, it will NOT tell you which side
/// of that reference you should use.  For this reason you should be
/// careful when using this method.  Use MazeGrid.locate() to get the
/// index and correct side.
impl std::ops::IndexMut<&Location> for MazeGrid {
    /// This method will only return a reference to the cell containing
    /// the data for the side in question, it will NOT tell you which side
    /// of that reference you should use.  For this reason you should be
    /// careful when using this method.  Use MazeGrid.locate() to get the
    /// index and correct side.
    fn index_mut<'a>(&'a mut self, loc: &Location) -> &'a mut MazeCell {
        unimplemented!() // TODO
    }
}

// The code below is left commented out to document why this is not and should
// not be implemented.
//
//
// Do not implement Index operations for MazeGrid[Location] as the Location
// will be destroyed and the caller would not be able to call grid.locate() to
// determine the correct side and would be left with only the cell index.
//
// If an Index operation is needed use MazeGrid[&Location] instead.
//
// impl std::ops::Index<Location> for MazeGrid {
//     type Output = MazeCell;
//
//     fn index(&self, loc: Location) -> &MazeCell {
//         unimplemented!() // DO NOT IMPLEMENT.  THIS IS LEFT UNIMPLEMENTED INTENTIONALLY
//     }
// }
//
// Do not implement Index operations for MazeGrid[Location] as the Location
// will be destroyed and the caller would not be able to call grid.locate() to
// determine the correct side and would be left with only the cell index.
//
// If an Index operation is needed use MazeGrid[&Location] instead.
//
// impl std::ops::IndexMut<Location> for MazeGrid {
//     fn index_mut<'a>(&'a mut self, loc: Location) -> &'a mut MazeCell {
//         unimplemented!() // DO NOT IMPLEMENT.  THIS IS LEFT UNIMPLEMENTED INTENTIONALLY
//     }
// }
