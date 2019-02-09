use maze_interface::maze_cell::MazeCell;
use maze_interface::maze_grid::MazeGrid;
use maze_interface::{Edge, EdgeFromStrError, FlatLocation, Location, LocationFromStrError};

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub fn maze() -> std::io::Result<usize> {
    let start_grid = Instant::now();
    let grid = MazeGrid::new(7u8);
    let grid_time = start_grid.elapsed();

    let start_render = Instant::now();
    let file: &Path = Path::new("/home/andrew/code/rust/proj/mazes/test_render.png");
    let size = grid.save_png(file, 15u16, 20u8);
    let render_time = start_render.elapsed();

    let debug = format!("{:#?}", grid);
    let debug_bytes = debug.as_bytes();
    let mut debug_file: File = File::create("/home/andrew/code/rust/proj/mazes/test_debug.txt")?;
    let end = start_grid.elapsed();
    println!(
        "Grid creation time: {}.{:09}\nPNG render & save time: {}.{:09}\nTotal time: {}.{:09}",
        grid_time.as_secs(),
        grid_time.subsec_nanos(),
        render_time.as_secs(),
        render_time.subsec_nanos(),
        end.as_secs(),
        end.subsec_nanos()
    );
    debug_file.write(debug_bytes)
    // Ok(())
}
