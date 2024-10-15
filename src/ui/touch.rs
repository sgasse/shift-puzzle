use wasm_bindgen::prelude::*;
use web_sys::TouchEvent;

use crate::{
    ui_locked,
    utils::{get_idx_from_row_col, get_row_col_from_idx, in_bounds},
    Error, BOARD, TOUCH_COORDS,
};

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

pub(crate) fn get_touch_start_callback() -> Closure<dyn FnMut(TouchEvent)> {
    Closure::wrap(Box::new(move |event: TouchEvent| {
        if ui_locked() {
            log::debug!("UI locked");
        } else {
            TOUCH_COORDS.with_borrow_mut(|t| {
                if let Some(t) = t.start {
                    log::warn!("Overwriting existing touch start coords: {:?}", t);
                }

                let first_touch = event.target_touches().get(0).unwrap();
                let coords = (first_touch.screen_x(), first_touch.screen_y());
                t.start = Some(coords);
            })
        }
    }))
}

pub(crate) fn get_touch_move_callback() -> Closure<dyn FnMut(TouchEvent)> {
    Closure::wrap(Box::new(move |event: TouchEvent| {
        if ui_locked() {
            log::debug!("UI locked");
        } else {
            TOUCH_COORDS.with_borrow_mut(|t| {
                let first_touch = event.target_touches().get(0).unwrap();
                let coords = (first_touch.screen_x(), first_touch.screen_y());
                t.end = Some(coords);
            })
        }
    }))
}

pub(crate) fn get_touch_end_callback(size: usize) -> Closure<dyn FnMut(TouchEvent)> {
    Closure::wrap(Box::new(move |_| {
        if ui_locked() {
            log::debug!("UI locked");
        } else {
            TOUCH_COORDS.with_borrow_mut(|t| {
                if let (Some((x_start, y_start)), Some((x_end, y_end))) = (t.start, t.end) {
                    if let Some(direction) = get_touch_direction(x_start, y_start, x_end, y_end) {
                        log::debug!("Handling touch direction {direction:?}");
                        handle_touch_move(size, direction).unwrap();
                    }

                    t.start = None;
                    t.end = None;
                } else {
                    log::warn!("Incomplete touch coordinates on touch end");
                }
            })
        }
    }))
}

#[derive(Debug)]
enum TouchMoveDirection {
    Left,
    Right,
    Up,
    Down,
}

fn get_touch_direction(
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

fn handle_touch_move(size: usize, direction: TouchMoveDirection) -> Result<bool, Error> {
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
