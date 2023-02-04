//! Utility functions for interacting with the board.
//!
use crate::Error;

/// Coordinates consisting of row and column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coords<T> {
    pub row: T,
    pub col: T,
}

/// Get the left/top coordinates based on the index of a board field.
pub fn get_left_top(idx: usize, width: usize, unit_size: usize) -> (usize, usize) {
    let (row, col): (usize, usize) = get_row_col_from_idx(idx, width);
    let left = col * unit_size;
    let top = row * unit_size;

    (left, top)
}

/// Get the row/column coordinates for a linear array representing a board.
pub fn get_row_col_from_idx<U>(idx: U, width: U) -> (U, U)
where
    U: std::ops::Div<Output = U>,
    U: std::ops::Rem<Output = U>,
    U: Copy,
{
    let row = idx / width;
    let col = idx % width;

    (row, col)
}

/// Get the index into a linear array based on row/column coordinates.
pub fn get_idx_from_row_col<T>(row: T, col: T, width: T) -> T
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
{
    row.mul(width).add(col)
}

/// Get the coordinates matching an index.
pub fn get_coords_from_idx<U>(idx: U, width: U) -> Coords<U>
where
    U: std::ops::Div<Output = U>,
    U: std::ops::Rem<Output = U>,
    U: Copy,
{
    let (row, col) = get_row_col_from_idx(idx, width);
    Coords { row, col }
}

/// Get the index matching a coordinate pair.
pub fn get_idx_from_coords<T>(coords: Coords<T>, width: T) -> T
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
{
    get_idx_from_row_col(coords.row, coords.col, width)
}

/// Check if row/column coordinates are within a field defined by width/height.
pub fn in_bounds<T, U>(row: T, col: T, width: U, height: U) -> bool
where
    T: PartialOrd<T>,
    T: PartialOrd<U>,
    T: Default,
{
    let t_zero: T = T::default();
    t_zero <= row && row < height && t_zero <= col && col < width
}

/// Get the index of a value in a slice.
///
/// This is a convenience wrapper and panics if the value cannot be found.
pub fn get_idx_of_val(slice: &[u8], value: u8) -> Result<usize, Error> {
    slice
        .iter()
        .position(|&v| v == value)
        .ok_or_else(|| -> Error { simple_error::simple_error!("Value not found").into() })
}
