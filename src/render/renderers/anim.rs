use crate::maze::{Grid, GridProps};
use crate::render::{Renderable, Renderer};
use crate::util::Index;
use image::RgbaImage;
use linked_hash_set::LinkedHashSet;
use rand::Rng;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use webp_animation::prelude::*;
use webp_animation::{Error, WebPData};

/// A structure used only for extracting data out of an [`RgbaImage`].
///
/// See also: https://docs.rs/image/0.23.14/image/struct.ImageBuffer.html
///
/// [`RgbaImage`]: https://docs.rs/image/0.23.14/image/type.RgbaImage.html
struct Buffer {
    _width: u32,
    _height: u32,
    data: Vec<u8>,
}

pub struct Animation<'a, 'f, R: Renderer<'f>> {
    renderer: R,
    anim: Cow<'a, AnimOpts>,
    /// how long to show each frame
    enc: RefCell<Encoder>,
    counter: RefCell<i32>,
    phantom: std::marker::PhantomData<&'f R>,
}

impl<'a, 'f, R: Renderer<'f>> Animation<'a, 'f, R> {
    pub fn new(renderer: R, opts: Option<&'a AnimOpts>) -> Self {
        Self {
            enc: RefCell::from(
                Encoder::new(renderer.grid().image_dimensions(renderer.opts())).unwrap(),
            ),
            renderer,
            counter: RefCell::from(0),
            anim: opts.map(|a| Cow::Borrowed(a)).unwrap_or_else(Cow::default),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn add(&self, frame: &[u8], time: i32) -> Result<(), Error> {
        let counter = *self.counter.borrow();
        *self.counter.borrow_mut() += time;
        self.enc.borrow_mut().add_frame(frame, counter + time)
    }
    pub fn add_rgba_frame(&self, img: RgbaImage, time: i32) -> Result<(), Error> {
        // Using unsafe here to avoid unnecessary allocation and processing.
        // The [`image`](https://docs.rs/image/) crate does not provide any way to reference the
        // underlying data, which is why unsafe is being used as a workaround.
        // Both types have the same fields and size, so this should be safe.
        let frame: Buffer = unsafe { std::mem::transmute(img) };
        // An alternative to using unsafe is:
        //      let frame = self.renderer.render_rgba();
        //      let dimensions = self.renderer.grid().image_dimensions(self.renderer.opts());
        //      let mut vec = Vec::with_capacity((dimensions.0 * dimensions.1) as usize);
        //      for (r, g, b, a) in frame.pixels().map(|p| p.channels4()) {
        //          vec.push(r);
        //          vec.push(g);
        //          vec.push(b);
        //          vec.push(a);
        //      }
        //      self.add(vec);
        self.add(&frame.data, time)
    }
    pub fn render(self) -> Result<WebPData, Error> {
        let enc = self.enc.into_inner();
        let counter = self.counter.into_inner();
        enc.finalize(counter + self.anim.end_delay())
    }
    pub fn save(self, path: &std::path::Path) -> Result<(), Error> {
        let mut f = File::create(path).expect("Error creating file");
        let bytes = self.render()?;
        f.write_all(&bytes).expect("Error writing to file");
        Ok(())
    }
    /// An implementation of [Wilson's Algorithm][wilsons].  It uses a loop-erased walk.
    ///
    /// See also: [Wilson's Algorithm][wilsons]
    ///
    /// [wilsons]: https://en.wikipedia.org/wiki/Maze_generation_algorithm#Wilson's_algorithm
    pub fn animated_wilsons<Rand: Rng + ?Sized>(
        self,
        path: &Path,
        rng: &mut Rand,
    ) -> Result<(), Error> {
        let grid = self.renderer.grid();
        let mut unvisited: LinkedHashSet<Index> = (0..*grid.capacity()).map(Index::from).collect();
        self.add_rgba_frame(self.renderer.render_rgba(), self.anim.time())?;
        let first = *unvisited
            .iter()
            .nth(rng.gen_range(0..unvisited.len()))
            .unwrap();
        unvisited.remove(&first);
        while !unvisited.is_empty() {
            let mut cell = *unvisited
                .iter()
                .nth(rng.gen_range(0..unvisited.len()))
                .unwrap();
            let mut path = vec![cell];
            while unvisited.contains(&cell) {
                cell = grid.random_neighbor_id(cell, rng);
                if let Some(pos) = path.iter().position(|i| *i == cell) {
                    path.truncate(pos + 1);
                } else {
                    path.push(cell);
                }
            }
            for i in 0..path.len() - 1 {
                let a = path[i];
                grid.link(a, path[i + 1]).unwrap();
                unvisited.remove(&a);
                self.add_rgba_frame(self.renderer.render_rgba(), self.anim.time())?;
            }
        }
        self.save(path)
    }
}

#[derive(Clone, Debug)]
pub struct AnimOpts {
    /// how long to show each frame
    time: i32,
    end_delay: i32,
}
impl AnimOpts {
    pub fn new() -> Self {
        Self::default()
    }
    fn time(&self) -> i32 {
        self.time
    }
    fn end_delay(&self) -> i32 {
        self.end_delay
    }
    pub fn set_time(self, time: i32) -> Self {
        Self { time, ..self }
    }
    pub fn set_end_delay(self, end_delay: i32) -> Self {
        Self { end_delay, ..self }
    }
}
impl Default for AnimOpts {
    fn default() -> Self {
        Self {
            time: 250,
            end_delay: 2500,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::sq::SqGrid;
    use crate::render::renderers::anim::*;
    use rand::SeedableRng;
    use rand_xoshiro::SplitMix64;
    use std::path::Path;
    #[test]
    fn wilsons_animation() -> Result<(), Error> {
        let mut rng = SplitMix64::seed_from_u64(852);
        let file = Path::new("animated_wilsons.webp");
        let grid = SqGrid::new(4);
        grid.render_defaults()
            .animation_defaults()
            .animated_wilsons(file, &mut rng)
    }
    #[test]
    fn wilsons_animation_options() -> Result<(), Error> {
        let mut rng = SplitMix64::seed_from_u64(852);
        let file = Path::new("animated_wilsons_options.webp");
        let anim = AnimOpts::new().set_time(150).set_end_delay(5000);
        let grid = SqGrid::new(4);
        grid.render_defaults()
            .animation(Some(&anim))
            .animated_wilsons(file, &mut rng)
    }
}
