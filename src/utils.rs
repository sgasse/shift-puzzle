//! Utility functions for interacting with the board.
//!

use std::collections::BTreeMap;

use rand::prelude::SliceRandom;
use web_sys::window;

use crate::{Error, BOARD};

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
pub(crate) fn get_row_col_from_idx<U>(idx: U, width: U) -> (U, U)
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
pub(crate) fn get_idx_from_row_col<T>(row: T, col: T, width: T) -> T
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
{
    row.mul(width).add(col)
}

/// Get the coordinates matching an index.
pub(crate) fn get_coords_from_idx<U>(idx: U, width: U) -> Coords<U>
where
    U: std::ops::Div<Output = U>,
    U: std::ops::Rem<Output = U>,
    U: Copy,
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
        .ok_or_else(|| -> Error { simple_error::simple_error!("Value not found").into() })
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

pub(crate) fn search_params() -> Option<String> {
    window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.trim_start_matches('?').replace("%22", "").to_owned())
}

const DEFAULT_SIZE: usize = 3;
const DEFAULT_BACKGROUND: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/6/61/Blue_Marble_Western_Hemisphere.jpg/600px-Blue_Marble_Western_Hemisphere.jpg?20130305115950";

pub(crate) fn extract_parameters() -> Parameters {
    let params = search_params()
        .map(|s| {
            BTreeMap::from_iter(
                s.split('&')
                    .filter_map(|s| s.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned()))),
            )
        })
        .unwrap_or_default();

    let size: usize = params
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SIZE);

    let bg_url = params
        .get("bg_url")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| DEFAULT_BACKGROUND.to_owned());

    Parameters { size, bg_url }
}

#[derive(Debug)]
pub(crate) struct Parameters {
    pub(crate) size: usize,
    pub(crate) bg_url: String,
}

pub(crate) fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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
            .ok_or_else(|| -> Error {
                simple_error::simple_error!("no random neighbour to choose").into()
            })?;
        swaps.push((empty_field_idx, *chosen_neighbour));
        prev_empty_field_idx = empty_field_idx;
        empty_field_idx = *chosen_neighbour;
    }

    Ok(swaps)
}

pub(crate) fn get_touch_direction(
    x_start: i32,
    y_start: i32,
    x_end: i32,
    y_end: i32,
) -> Option<TouchMoveDirection> {
    let d_x = x_end - x_start;
    let d_y = y_end - y_start;

    if d_x.abs() + d_y.abs() < 40 {
        // Overall displacement is too small, ignore
        return None;
    }

    match d_x.abs() > d_y.abs() {
        true => {
            // Horizontal
            if d_x > 0 {
                Some(TouchMoveDirection::Right)
            } else {
                Some(TouchMoveDirection::Left)
            }
        }
        false => {
            // Vertical
            if d_y > 0 {
                Some(TouchMoveDirection::Down)
            } else {
                Some(TouchMoveDirection::Up)
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum TouchMoveDirection {
    Left,
    Right,
    Up,
    Down,
}

pub(crate) fn handle_touch_move(size: usize, direction: TouchMoveDirection) -> Result<bool, Error> {
    let empty_field_id = size * size - 1;
    let empty_field_idx = BOARD.with_borrow(|b| b.ids2indices()[empty_field_id]);
    let (empty_row, empty_col): (usize, usize) = get_row_col_from_idx(empty_field_idx, size);
    let (empty_row, empty_col) = (empty_row as i32, empty_col as i32);

    let (d_row, d_col) = match direction {
        // If the slide was to the left, the user wants to move the neighbour
        // on the *right* towards the left.
        TouchMoveDirection::Left => (0, 1),
        TouchMoveDirection::Right => (0, -1),
        TouchMoveDirection::Up => (1, 0),
        TouchMoveDirection::Down => (-1, 0),
    };

    let (swap_row, swap_col) = (empty_row + d_row, empty_col + d_col);

    if in_bounds(swap_row, swap_col, size as i32, size as i32) {
        let swap_idx: i32 = get_idx_from_row_col(swap_row, swap_col, size as i32);
        BOARD.with_borrow_mut(|b| b.swap_indices(empty_field_idx, swap_idx as usize));
        return Ok(true);
    }

    // The candidate was not in bounds -> do nothing
    Ok(false)
}

#[derive(Debug)]
pub(crate) struct TouchCoords {
    pub(crate) start: Option<(i32, i32)>,
    pub(crate) end: Option<(i32, i32)>,
}

impl TouchCoords {
    pub(crate) const fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}
