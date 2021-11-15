use self::major::*;
use crate::maze::*;
use crate::util::*;

/// `Transform`s allow iteration over a grid using a navigational component (like [`Rev`] or [`Sw`]).
/// The `transform()` method replaces one `Index` with another, often using a simple calculation
/// with the `id` and `grid.capacity()`.
///
/// Transforms can be applied on top of each other, like:
/// ```
/// use mazes::*;
/// let grid = SqGrid::new(4);
/// for cell in grid.se().reverse() {
///     // ...
/// }
/// ```
///
/// Iterators over a [`Grid`] use the [`Iter`] struct, which is generic over `T: Transform`.
/// This means that the single [`Iter`] struct will be used for iterators over all of the different types of transforms.
/// Multiple transforms can be applied by using the [`NestedIter`] struct.
///
/// Since `Iter` is generic over `T: Transform` a transform must be supplied even if the user does not
/// want to apply a transformation.  For this use the [`Ident`] transform; it will not apply any
/// transformation to the `id` and will simply return it.
///
pub trait Transform {
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize;
}
/// Since `Iter` is generic over `T: Transform` a transform must be supplied even if the user does not
/// want to apply a transformation.  For this use the [`Ident`] transform; it will not apply any
/// transformation to the `id` and will simply return it.
///
pub struct Ident;
impl Transform for Ident {
    fn transform<G: Grid>(&self, id: usize, _grid: &G) -> usize {
        id
    }
}
/// Reverse the iteration order of a grid.  The method for calling this on a grid is `.reverse()`
/// instead of `.rev()` due to conflicts with the standard library `.rev()` method on iterators.
///
/// Calculates the new `id` using: `grid.capacity() - id`.
pub struct Rev;
impl Transform for Rev {
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        println!("Reversing {} to {}", id, grid.capacity().minus(id) - 1);
        grid.capacity().minus(id) - 1
    }
}

pub struct Nw<M: Major> {
    rows: RowSize,
    cols: ColSize,
    m: fn(Visit, RowSize, ColSize, M) -> Index,
}
impl<M: Major> Nw<M> {
    pub(crate) fn new(
        rows: RowSize,
        cols: ColSize,
        m: fn(Visit, RowSize, ColSize, M) -> Index,
    ) -> Self {
        Self { rows, cols, m }
    }
}
impl<M: Major> Transform for Nw<M> {
    /// Transform for iterating from the NW corner.
    ///
    /// # Panics
    ///
    /// Can only be used on a SqGrid, otherwise a panic will occur.
    ///
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        let t = self.m;
        *(t(id.into(), self.rows, self.cols, M::new()))
    }
}

pub struct Ne<M: Major> {
    rows: RowSize,
    cols: ColSize,
    m: fn(Visit, RowSize, ColSize, M) -> Index,
}
impl<M: Major> Ne<M> {
    pub(crate) fn new(
        rows: RowSize,
        cols: ColSize,
        m: fn(Visit, RowSize, ColSize, M) -> Index,
    ) -> Self {
        Self { rows, cols, m }
    }
}
impl<M: Major> Transform for Ne<M> {
    /// Transform for iterating from the NE corner.
    ///
    /// # Panics
    ///
    /// Can only be used on a SqGrid, otherwise a panic will occur.
    ///
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        let t = self.m;
        *(t(id.into(), self.rows, self.cols, M::new()))
    }
}

pub struct Se<M: Major> {
    rows: RowSize,
    cols: ColSize,
    m: fn(Visit, RowSize, ColSize, M) -> Index,
}
impl<M: Major> Se<M> {
    pub(crate) fn new(
        rows: RowSize,
        cols: ColSize,
        m: fn(Visit, RowSize, ColSize, M) -> Index,
    ) -> Self {
        Self { rows, cols, m }
    }
}
impl<M: Major> Transform for Se<M> {
    /// Transform for iterating from the SE corner.
    ///
    /// # Panics
    ///
    /// Can only be used on a SqGrid, otherwise a panic will occur.
    ///
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        let t = self.m;
        *(t(id.into(), self.rows, self.cols, M::new()))
    }
}

pub struct Sw<M: Major> {
    rows: RowSize,
    cols: ColSize,
    m: fn(Visit, RowSize, ColSize, M) -> Index,
}
impl<M: Major> Sw<M> {
    pub(crate) fn new(
        rows: RowSize,
        cols: ColSize,
        m: fn(Visit, RowSize, ColSize, M) -> Index,
    ) -> Self {
        Self { rows, cols, m }
    }
}
impl<M: Major> Transform for Sw<M> {
    /// Transform for iterating from the SW corner.
    ///
    /// # Panics
    ///
    /// Can only be used on a SqGrid, otherwise a panic will occur.
    ///
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        let row_size: RowSize = ((*grid.capacity() as f32).sqrt() as usize).into();
        assert!(row_size.cap() == grid.capacity()); // must be a square grid
        let t = self.m;
        *(t(id.into(), self.rows, self.cols, M::new()))
    }
}

/// `NestedIter` allows [`Transform`] types to be applied on top of each other.
///
/// This works by taking two generic [`Transform`] params.  The [`Transform`] trait is
/// implemented by calling `A::transform()` then sending the result to `B::transform()`.
/// Since `NestedIter` also implements `Transform`, it will be able to apply any number
/// of `Transform`s.  
///
/// # Example
/// ```
/// use mazes::{CardinalGrid};
/// let grid = mazes::maze::sq::SqGrid::new(4);
/// for i in grid.sw().reverse() {
///     // iterates from ne() corner because of the call to .reverse()
/// }
///
/// ```
///
/// [`Transform`]: crate::trans::Transform
pub struct NestedIter<A: Transform, B: Transform> {
    a: A,
    b: B,
    // _phantom: std::marker::PhantomData<(A, B)>,
}
impl<A: Transform, B: Transform> NestedIter<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<A: Transform, B: Transform> Transform for NestedIter<A, B> {
    fn transform<G: Grid>(&self, id: usize, grid: &G) -> usize {
        self.b.transform(self.a.transform(id, grid), grid)
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
        fn new() -> Self;
        fn op_x() -> fn(Visit, RowSize) -> Index;
        fn op_y() -> fn(Visit, RowSize) -> Index;
    }
    pub struct RowMajor;
    impl Major for RowMajor {
        fn new() -> Self {
            Self
        }
        fn op_x() -> fn(Visit, RowSize) -> Index {
            self::rem
        }
        fn op_y() -> fn(Visit, RowSize) -> Index {
            self::div
        }
    }
    pub struct ColMajor;
    impl Major for ColMajor {
        fn new() -> Self {
            Self
        }
        fn op_x() -> fn(Visit, RowSize) -> Index {
            self::div
        }
        fn op_y() -> fn(Visit, RowSize) -> Index {
            self::rem
        }
    }
}
