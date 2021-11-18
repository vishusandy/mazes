use crate::maze::sq::SqGrid;
use crate::maze::CardinalGrid;
use crate::maze::Grid;
use crate::render::renderers::DistMapRenderer;
use crate::render::renderers::{PathMapOpts, PathMapRenderer};
use crate::render::{BasicOpts, Renderable};
use crate::util::dist::Distances;
use crate::util::{Cardinal, Index};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Path<'g, G: Grid> {
    grid: &'g G,
    path: Vec<Index>,
}

impl<'g, G: Grid> Path<'g, G> {
    pub fn new(path: Vec<Index>, grid: &'g G) -> Self {
        Self { grid, path }
    }
    pub(in crate) fn grid(&self) -> &'g G {
        self.grid
    }
    pub fn string(&self) -> String {
        let mut string = String::new();
        for (i, node) in self.path.iter().enumerate() {
            if i != 0 {
                string.push_str(" -> ");
            }
            let s = format!("{}", node);
            string.push_str(&s);
        }
        string
    }
    pub fn iter(&self) -> std::slice::Iter<Index> {
        self.path.iter()
    }
    pub fn as_map(&self) -> HashMap<Index, usize> {
        let mut map = HashMap::new();
        self.path.iter().enumerate().for_each(|(dist, id)| {
            map.insert(*id, dist);
        });
        map
    }
    pub fn first(&self) -> Index {
        self.path[0]
    }
    pub fn last(&self) -> Index {
        self.path[self.path.len() - 1]
    }
    pub fn get(&self, step: usize) -> Option<Index> {
        self.path.get(step).copied()
    }
    pub fn get_ref(&self) -> &[Index] {
        &self.path
    }
    pub fn prev(&self, id: Index) -> Option<Index> {
        let pos = self.position(id)?;
        if pos == 0 {
            return None;
        }
        self.get(pos - 1)
    }
    pub fn next(&self, id: Index) -> Option<Index> {
        let pos = self.position(id)?;
        if pos == self.len() - 1 {
            return None;
        }
        self.get(pos + 1)
    }
    pub fn reverse(&mut self) {
        self.path.reverse();
    }
    pub fn position(&self, id: Index) -> Option<usize> {
        self.path.iter().position(|&i| i == id)
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}
impl<'g, G: Grid + CardinalGrid> Path<'g, G> {
    pub fn prev_dir(&self, id: Index) -> Option<Cardinal> {
        // println!("id={}", id);
        // if let Some(prev) = self.prev(id) {
        //     if let Some(dir) = self.grid.dir_from(id, prev) {
        //         println!("\tprev_id={} prev_dir={}", prev, dir);
        //         Some(dir)
        //     } else {
        //         println!("\tboundary fallback: {:?}", self.grid.find_boundary(id));
        //         self.grid.find_boundary(id)
        //     }
        // } else {
        //     None
        // }
        self.prev(id)
            .map(|a| self.grid.dir_from(id, a))
            .flatten()
            .or_else(|| self.grid.find_boundary(id))
    }
    pub fn next_dir(&self, id: Index) -> Option<Cardinal> {
        self.next(id)
            .map(|a| self.grid.dir_from(id, a))
            .flatten()
            .or_else(|| self.grid.find_boundary(id))
    }
}

impl<'g, G: Grid> std::fmt::Display for Path<'g, G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl<'f, 'g, 'm> Path<'g, SqGrid> {
    pub fn render_defaults(&'m self) -> PathMapRenderer<'f, 'g, 'm, SqGrid> {
        PathMapRenderer::new(self, None, None)
    }
    pub fn render_options(
        &'m self,
        basic: Option<BasicOpts<'f>>,
        path_opts: Option<PathMapOpts>,
    ) -> PathMapRenderer<'f, 'g, 'm, SqGrid> {
        PathMapRenderer::new(self, basic, path_opts)
    }
}
