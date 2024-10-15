//! Utility functions for interacting with the board.
//!

use rand::prelude::SliceRandom;
use simple_error::simple_error;

use crate::Error;

/// Coordinates consisting of row and column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Coords<T> {
    pub(crate) row: T,
    pub(crate) col: T,
}

/// Get the left/top coordinates based on the index of a board field.
pub(crate) fn get_left_top(idx: usize, width: usize, unit_size: usize) -> (usize, usize) {
    let (row, col): (usize, usize) = get_row_col_from_idx(idx, width);
    let left = col * unit_size;
    let top = row * unit_size;

    (left, top)
}

/// Get the row/column coordinates for a linear array representing a board.
pub(crate) fn get_row_col_from_idx<T>(idx: T, width: T) -> (T, T)
where
    T: std::ops::Div<Output = T>,
    T: std::ops::Rem<Output = T>,
    T: Copy,
{
    let row = idx / width;
    let col = idx % width;

    (row, col)
}

/// Get the index into a linear array based on row/column coordinates.
pub(crate) fn get_idx_from_row_col<T>(row: T, col: T, width: T) -> T
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
{
    row.mul(width).add(col)
}

/// Get the coordinates matching an index.
pub(crate) fn get_coords_from_idx<T>(idx: T, width: T) -> Coords<T>
where
    T: std::ops::Div<Output = T>,
    T: std::ops::Rem<Output = T>,
    T: Copy,
{
    let (row, col) = get_row_col_from_idx(idx, width);
    Coords { row, col }
}

/// Get the index matching a coordinate pair.
pub(crate) fn get_idx_from_coords<T>(coords: Coords<T>, width: T) -> T
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
{
    get_idx_from_row_col(coords.row, coords.col, width)
}

/// Check if row/column coordinates are within a field defined by width/height.
pub(crate) fn in_bounds<T, U>(row: T, col: T, width: U, height: U) -> bool
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
pub(crate) fn get_idx_of_val(slice: &[u8], value: u8) -> Result<usize, Error> {
    slice
        .iter()
        .position(|&v| v == value)
        .ok_or_else(|| simple_error!("value not found").into())
}

/// Initialize fields as vector.
pub(crate) fn initialize_fields(num_elements: usize) -> Vec<u8> {
    let num_elements = usize::min(num_elements, u8::MAX as usize) as u8;
    (0..num_elements).collect()
}

/// Get the indices of neighbours that can be swapped with the empty field.
pub(crate) fn get_swappable_neighbours(
    width: usize,
    height: usize,
    empty_field_idx: usize,
) -> Result<Vec<usize>, Error> {
    let (row, col): (usize, usize) = get_row_col_from_idx(empty_field_idx, width);

    Ok([(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(delta_row, delta_col)| {
            let neighbour_row = row as isize + delta_row;
            let neighbour_col = col as isize + delta_col;
            match in_bounds(
                neighbour_row,
                neighbour_col,
                width as isize,
                height as isize,
            ) {
                true => {
                    let idx: isize =
                        get_idx_from_row_col(neighbour_row, neighbour_col, width as isize);
                    Some(idx as usize)
                }
                false => None,
            }
        })
        .collect())
}

/// Get a sequence of valid semi-random shuffles.
///
/// We prevent fields from being shuffled back and forth, which breaks total
/// randomness.
pub(crate) fn get_shuffle_sequence(
    size: usize,
    mut empty_field_idx: usize,
    num_swaps: usize,
) -> Result<Vec<(usize, usize)>, Error> {
    let mut swaps = Vec::with_capacity(num_swaps);

    // We want to avoid swapping fields back and forth like (2, 1), (1, 2)
    // Our approach is to remove the previous empty field from swappable
    // neighbours
    let mut prev_empty_field_idx = empty_field_idx;

    for _ in 0..num_swaps {
        let swappable_neighbours: Vec<_> = get_swappable_neighbours(size, size, empty_field_idx)?
            .into_iter()
            .filter(|&element| element != prev_empty_field_idx)
            .collect();
        let chosen_neighbour = swappable_neighbours
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| simple_error!("no random neighbour to choose"))?;
        swaps.push((empty_field_idx, *chosen_neighbour));
        prev_empty_field_idx = empty_field_idx;
        empty_field_idx = *chosen_neighbour;
    }

    Ok(swaps)
}
