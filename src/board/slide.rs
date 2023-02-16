//! Slide interaction functions.
//!
use crate::{
    utils::{get_idx_from_row_col, get_idx_of_val, get_row_col_from_idx, in_bounds},
    Error,
};

pub enum TouchMoveDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Trigger a field by swapping it with the empty field if it is adjacent.
pub fn handle_field_click(
    fields: &mut [u8],
    width: usize,
    height: usize,
    clicked_idx: usize,
) -> bool {
    if let Some(&u8::MAX) = fields.get(clicked_idx) {
        // Clicked on the empty field - unclear so nothing to do
        return false;
    }

    let (row, col): (usize, usize) = get_row_col_from_idx(clicked_idx, width);
    for (delta_row, delta_col) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbour_row = row as isize + delta_row;
        let neighbour_col = col as isize + delta_col;
        if in_bounds(
            neighbour_row,
            neighbour_col,
            width as isize,
            height as isize,
        ) {
            let idx: isize = get_idx_from_row_col(neighbour_row, neighbour_col, width as isize);
            if let Some(&u8::MAX) = fields.get(idx as usize) {
                fields.swap(clicked_idx, idx as usize);
                // Fields swapped - re-render
                return true;
            }
        }
    }

    // No field swapped - do not re-render
    false
}

pub fn get_touch_direction(
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

pub fn handle_touch_move(
    fields: &mut [u8],
    width: usize,
    height: usize,
    direction: TouchMoveDirection,
) -> Result<bool, Error> {
    let empty_field_idx = get_idx_of_val(fields, u8::MAX)?;
    let (empty_row, empty_col): (usize, usize) = get_row_col_from_idx(empty_field_idx, width);
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

    if in_bounds(swap_row, swap_col, width as i32, height as i32) {
        let swap_idx: i32 = get_idx_from_row_col(swap_row, swap_col, width as i32);
        fields.swap(empty_field_idx, swap_idx as usize);
        return Ok(true);
    }

    // The candidate was not in bounds -> do nothing
    Ok(false)
}
