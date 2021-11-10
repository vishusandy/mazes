use self::major::*;
use crate::maze::*;
use crate::util::*;
pub trait Transform {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize;
}
pub struct Ident;
impl Transform for Ident {
    fn transform<G: Grid>(id: usize, _grid: &G) -> usize {
        id
    }
}
pub struct Rev;
impl Transform for Rev {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        println!("Reversing {} to {}", id, grid.capacity().minus(id) - 1);
        grid.capacity().minus(id) - 1
    }
}
// Can only use Nw/Ne/Se/Sw transforms on CardinalGrids.
pub struct Nw;
impl Transform for Nw {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        *Ordinal::Nw.major_order_index(id.into(), row_size, RowMajor)
    }
}
pub struct Ne;
impl Transform for Ne {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        *Ordinal::Ne.major_order_index(id.into(), row_size, RowMajor)
    }
}
pub struct Se;
impl Transform for Se {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        *Ordinal::Se.major_order_index(id.into(), row_size, RowMajor)
    }
}
pub struct Sw;
impl Transform for Sw {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        *Ordinal::Sw.major_order_index(id.into(), row_size, RowMajor)
    }
}
pub struct NestedIter<A: Transform, B: Transform> {
    _phantom: std::marker::PhantomData<(A, B)>,
}
impl<A: Transform, B: Transform> Transform for NestedIter<A, B> {
    fn transform<G: Grid>(id: usize, grid: &G) -> usize {
        B::transform(A::transform(id, grid), grid)
    }
}
pub mod major {
    use crate::util::*;
    pub(in crate) fn div(v: Visit, s: RowSize) -> Index {
        (*v / *s).into()
    }
    pub(in crate) fn rem(v: Visit, s: RowSize) -> Index {
        (*v % *s).into()
    }
    pub trait Major {
        fn op_x() -> fn(Visit, RowSize) -> Index;
        fn op_y() -> fn(Visit, RowSize) -> Index;
    }
    pub struct RowMajor;
    impl Major for RowMajor {
        fn op_x() -> fn(Visit, RowSize) -> Index {
            self::rem
        }
        fn op_y() -> fn(Visit, RowSize) -> Index {
            self::div
        }
    }
    pub struct ColMajor;
    impl Major for ColMajor {
        fn op_x() -> fn(Visit, RowSize) -> Index {
            self::div
        }
        fn op_y() -> fn(Visit, RowSize) -> Index {
            self::rem
        }
    }
}
