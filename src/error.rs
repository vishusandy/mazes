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
