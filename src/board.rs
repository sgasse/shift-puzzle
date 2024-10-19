use rand::prelude::SliceRandom;
use simple_error::simple_error;

use crate::Error;

#[derive(Debug)]
pub(crate) struct Board {
    /// Vector mapping indices to IDs.
    /// e.g. ids[5] = 6 -> index 5 has tile 6 on the board
    indices2ids: Vec<u8>,
    /// Vector mapping IDs to indices.
    /// e.g. indices[2] = 4 -> tile 2 is at index 4 on the board
    ids2indices: Vec<usize>,
}

impl Board {
    pub(crate) const fn new() -> Self {
        Self {
            indices2ids: Vec::new(),
            ids2indices: Vec::new(),
        }
    }

    pub(crate) fn init(&mut self, size: usize) {
        let num_elements = size * size;
        self.indices2ids = (0..(num_elements as u8)).collect();
        self.ids2indices = (0..num_elements).collect();
    }

    pub(crate) fn indices2ids(&self) -> &Vec<u8> {
        &self.indices2ids
    }

    pub(crate) fn ids2indices(&self) -> &Vec<usize> {
        &self.ids2indices
    }

    pub(crate) fn swap_ids(&mut self, id_a: u8, id_b: u8) {
        debug_assert!((id_a as usize) < self.indices2ids.len());
        debug_assert!((id_b as usize) < self.indices2ids.len());

        // Swap IDs / indices in maps.
        // Look up at which index which ID is.
        // Swap the IDs in both maps.
        let idx_a = self.ids2indices[id_a as usize];
        let idx_b = self.ids2indices[id_b as usize];
        self.indices2ids.swap(idx_a, idx_b);

        self.ids2indices[id_a as usize] = idx_b;
        self.ids2indices[id_b as usize] = idx_a;
    }
}

/// Coordinates consisting of row and column.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Coords<T> {
    pub(crate) row: T,
    pub(crate) col: T,
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
/// This is a convenience wrapper which should not be used in a hot path.
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

pub(crate) fn get_empty_field_idx(fields: &[u8]) -> Result<usize, Error> {
    get_idx_of_val(fields, fields.len() as u8 - 1)
}

/// Get the indices of neighbours that can be swapped with the empty field.
pub(crate) fn get_swappable_neighbours(
    width: usize,
    height: usize,
    empty_field_idx: usize,
) -> impl Iterator<Item = usize> {
    let (row, col): (usize, usize) = get_row_col_from_idx(empty_field_idx, width);

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(move |(delta_row, delta_col)| {
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
        let swappable_neighbours: Vec<_> = get_swappable_neighbours(size, size, empty_field_idx)
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
