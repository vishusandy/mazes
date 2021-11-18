use crate::maze::{Cell, Grid};
use crate::render::renderers::{DistMapOpts, DistMapRenderer};
use crate::render::{BasicOpts, Renderable};
use crate::util::path::Path;
use crate::util::Index;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Distances<'g, G: Grid> {
    grid: &'g G,
    root: Index,
    map: HashMap<Index, usize>,
}

impl<'g, G> Distances<'g, G>
where
    G: Grid,
{
    pub fn new(grid: &'g G, start: Index) -> Self {
        let mut map = HashMap::new();
        map.insert(start, 0);
        Self {
            grid,
            root: start,
            // map: HashMap::new(),
            // map: HashMap::from([(start, 0)]),
            map,
        }
    }
    pub fn root(&self) -> Index {
        self.root
    }
    pub(in crate) fn grid(&self) -> &'g G {
        self.grid
    }
    pub fn has_entry(&self, id: &Index) -> bool {
        self.map.contains_key(id)
    }
    pub fn set(&mut self, id: Index, val: usize) {
        self.map.entry(id).or_insert(val);
    }
    pub fn get(&self, id: Index) -> Option<usize> {
        self.map.get(&id).copied()
    }
    pub fn map_ref(&self) -> &HashMap<Index, usize> {
        &self.map
    }
    pub fn max_dist(&self) -> (Index, usize) {
        self.map_ref()
            .iter()
            .max_by(|(_, av), (_, bv)| av.cmp(bv))
            .map(|(id, dist)| (*id, *dist))
            .unwrap()
    }
    pub fn shortest_path(&self, end: Index) -> Path<'g, G> {
        let mut path: Vec<Index> = vec![end];
        let mut cur = end;
        let root = self.root;
        while cur != root {
            let cell = self.grid.get(cur).expect("cell could not be retrieved");
            for link in cell.links().borrow().iter() {
                if self[*link] < self[cur] {
                    path.push(*link);
                    cur = *link;
                }
            }
        }
        path.reverse();
        Path::new(path, self.grid)
    }
}

impl<'g, G> Distances<'g, G>
where
    G: Grid + Renderable + Clone,
{
    pub fn render_defaults<'f, 'm>(&'m self) -> DistMapRenderer<'f, 'g, 'm, G> {
        DistMapRenderer::new(self, Some(BasicOpts::default()), None)
    }
    pub fn render_options<'f, 'm>(
        &'m self,
        opts: Option<BasicOpts<'f>>,
        dist_opts: Option<DistMapOpts>,
    ) -> DistMapRenderer<'f, 'g, 'm, G> {
        DistMapRenderer::new(self, opts, dist_opts)
    }
}

impl<'g, G: Grid> std::ops::Index<Index> for Distances<'g, G> {
    type Output = usize;
    fn index(&self, index: Index) -> &Self::Output {
        &self.map[&(index)]
    }
}
impl<'g, G: Grid> std::ops::IndexMut<Index> for Distances<'g, G> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.map.entry(index).or_insert(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::sq::tests::new_maze;
    use crate::maze::Grid;
    use crate::render::Renderer;
    use crate::util::Index;
}
