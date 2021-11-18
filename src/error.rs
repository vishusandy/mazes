use crate::util::*;
use thiserror::Error;
#[derive(Error, Debug)]
#[error("Out of bounds error for id={0}")]
pub struct OutOfBoundsError(Index);
impl OutOfBoundsError {
    pub(in crate) fn new(id: Index) -> Self {
        Self(id)
    }
}
#[derive(Error, Debug)]
#[error("Out of bounds error for coord={0}")]
pub struct OutOfBoundsCoordError(Coord);
impl OutOfBoundsCoordError {
    pub(in crate) fn new(coord: Coord) -> Self {
        Self(coord)
    }
}

#[derive(Error, Debug)]
#[error("Could not save image {0}")]
pub struct ImageSaveError<'p>(&'p std::path::PathBuf);

#[derive(Error, Debug)]
#[error("Error linking {a} and {b}: {reason}")]
pub struct CellLinkError<'s> {
    a: Index,
    b: Index,
    reason: &'s str,
}
impl<'s> CellLinkError<'s> {
    pub(in crate) fn new(a: Index, b: Index, reason: &'s str) -> Self {
        Self { a, b, reason }
    }
}

#[derive(Error, Debug)]
#[error("Error linking {a} and {t}: {reason}")]
pub struct NotNeighborsError<'s, T: std::fmt::Display> {
    a: Index,
    t: T,
    reason: &'s str,
}
impl<'s, T: std::fmt::Display> NotNeighborsError<'s, T> {
    pub(in crate) fn new(a: Index, t: T, reason: &'s str) -> Self {
        Self { a, t, reason }
    }
}
