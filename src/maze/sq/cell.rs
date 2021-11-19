use crate::maze::Cell;
use crate::util::*;
use parse_display::Display;
use std::cell::RefCell;
#[derive(Clone, Debug, Display)]
#[display("Cell {id}")]
// todo: impl PartialEq, PartialOrd, Add/Sub/Mul/Div/Rem/AddAssign/SubAssign/MulAssign/DivAssign/Deref
pub struct SqCell {
    id: Index,
    links: RefCell<Vec<Index>>,
    neighbors: Vec<Index>,
}
impl SqCell {
    pub(in crate::maze::sq) fn new(id: Index, neighbors: Vec<Index>) -> Self {
        Self {
            id,
            links: RefCell::from(Vec::new()),
            neighbors,
        }
    }
}
impl Cell for SqCell {
    fn id(&self) -> Index {
        self.id
    }
    fn neighbor_ids(&self) -> &[Index] {
        &self.neighbors
    }
    fn unchecked_link(&self, with: Index) {
        self.links.borrow_mut().push(with);
    }
    fn links(&self) -> &RefCell<Vec<Index>> {
        &self.links
    }
    fn unchecked_unlink(&self, with: Index) {
        if let Some(pos) = self.links.borrow().iter().position(|n| *n == with) {
            self.links.borrow_mut().remove(pos);
        }
    }
}
