use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MouseEvent, TouchEvent};

use crate::{
    lock_ui,
    solver::{divide_and_conquer::DacPuzzleSolver, optimal::find_swap_order},
    ui_locked, unlock_ui,
    utils::{get_shuffle_sequence, get_touch_direction, handle_touch_move},
    BOARD, TOUCH_COORDS,
};

const NUM_SHUFFLES: usize = 10;
const SWAP_TIMEOUT_FAST: i32 = 250;
const SWAP_TIMEOUT_SLOW: i32 = 500;

pub(crate) fn setup_button_callbacks(size: usize) {
    let document = window().unwrap().document().unwrap();

    #[allow(clippy::type_complexity)]
    let ids_get_callbacks: [(_, &dyn Fn(usize) -> Closure<dyn FnMut(MouseEvent)>); 4] = [
        ("quick_swap", &get_quick_swap_callback),
        ("granular_swap", &get_granular_swap_callback),
        ("optimal_solve", &get_optimal_solve_callback),
        ("d_and_c_solve", &get_dac_solve_callback),
    ];

    for (id, get_callback) in ids_get_callbacks {
        let callback = get_callback(size);
        let button = document
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        button.set_onclick(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }

    let board = document
        .get_element_by_id("board")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let touch_start_callback = get_touch_start_callback();
    let touch_move_callback = get_touch_move_callback();
    let touch_end_callback = get_touch_end_callback(size);
    board.set_ontouchstart(Some(touch_start_callback.as_ref().unchecked_ref()));
    board.set_ontouchmove(Some(touch_move_callback.as_ref().unchecked_ref()));
    board.set_ontouchend(Some(touch_end_callback.as_ref().unchecked_ref()));
    touch_start_callback.forget();
    touch_move_callback.forget();
    touch_end_callback.forget();
}

fn get_quick_swap_callback(size: usize) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_| {
        if !lock_ui() {
            return;
        }

        let empty_field_idx = size - 1;

        match get_shuffle_sequence(size, empty_field_idx, 20) {
            Ok(shuffle_sequence) => {
                log::info!("Shuffle sequence: {:?}", &shuffle_sequence);

                BOARD.with_borrow_mut(|b| {
                    for swap in shuffle_sequence {
                        b.swap_indices(swap.0, swap.1);
                    }
                });
            }
            Err(err) => {
                log::error!("failed in quick swapping: {err}");
            }
        }

        unlock_ui();
    }))
}

fn get_granular_swap_callback(size: usize) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_| {
        if !lock_ui() {
            return;
        }

        let num_shuffles = NUM_SHUFFLES;
        let empty_field_idx = size - 1;

        match get_shuffle_sequence(size, empty_field_idx, num_shuffles) {
            Ok(shuffle_sequence) => {
                log::info!("Shuffle sequence: {:?}", &shuffle_sequence);

                let window = window().unwrap();
                let mut callbacks = Vec::with_capacity(num_shuffles);

                // Send every shuffle with a separate timeout.
                for (i, swap) in shuffle_sequence.into_iter().enumerate() {
                    let callback = get_swap_callback(swap);
                    let millis = SWAP_TIMEOUT_FAST * (i as i32 + 1);

                    window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            callback.as_ref().unchecked_ref(),
                            millis,
                        )
                        .unwrap();

                    // Keep callback handles to drop at the end.
                    callbacks.push(callback);
                }

                let mut _callbacks = Some(callbacks);
                let finish_callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    // Drop callbacks by overwriting with None.
                    _callbacks = None;
                    log::debug!("Finished granular swap sequence");
                    unlock_ui();
                }));

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        finish_callback.as_ref().unchecked_ref(),
                        (num_shuffles as i32 + 1) * SWAP_TIMEOUT_FAST,
                    )
                    .unwrap();
                finish_callback.forget();
            }
            Err(err) => {
                log::error!("failed in granular swapping: {err}");
                unlock_ui();
            }
        }
    }))
}

fn get_optimal_solve_callback(size: usize) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_| {
        if !lock_ui() {
            return;
        }

        let ids = BOARD.with_borrow(|b| b.indices2ids().clone());
        match find_swap_order(&ids, size, size) {
            Ok(solve_sequence) => {
                apply_solve_sequence(solve_sequence, SWAP_TIMEOUT_SLOW);
            }
            Err(err) => {
                log::error!("failed to find optimal solve sequence: {err}");
                unlock_ui();
            }
        }
    }))
}

fn get_dac_solve_callback(size: usize) -> Closure<dyn FnMut(MouseEvent)> {
    Closure::wrap(Box::new(move |_| {
        if !lock_ui() {
            return;
        }

        let ids = BOARD.with_borrow(|b| b.indices2ids().clone());
        match DacPuzzleSolver::new(&ids, size as i32, size as i32) {
            Ok(mut solver) => match solver.solve_puzzle() {
                Ok(solve_sequence) => {
                    apply_solve_sequence(solve_sequence, SWAP_TIMEOUT_SLOW);
                }
                Err(err) => {
                    log::error!("failed to solve puzzle: {err}");
                    unlock_ui();
                }
            },
            Err(err) => {
                log::error!("failed to create divide&conquer solver: {err}");
                unlock_ui();
            }
        }
    }))
}

fn get_swap_callback(swap: (usize, usize)) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        BOARD.with_borrow_mut(|b| b.swap_indices(swap.0, swap.1));
    }))
}

fn apply_solve_sequence(solve_sequence: Vec<(usize, usize)>, interval: i32) {
    log::info!("Solve sequence: {:?}", &solve_sequence);
    let num_swaps = solve_sequence.len();

    let window = window().unwrap();
    let mut callbacks = Vec::with_capacity(num_swaps);

    // Send every shuffle with a separate timeout.
    for (i, swap) in solve_sequence.into_iter().enumerate() {
        let callback = get_swap_callback(swap);
        let millis = (i as i32 + 1) * interval;

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                millis,
            )
            .unwrap();

        // Keep callback handles to drop at the end.
        callbacks.push(callback);
    }

    let mut _callbacks = Some(callbacks);
    let finish_callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        // Drop callbacks by overwriting with None.
        _callbacks = None;
        log::debug!("Finished swap sequence");
        unlock_ui();
    }));

    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            finish_callback.as_ref().unchecked_ref(),
            (num_swaps as i32 + 1) * interval,
        )
        .unwrap();
    finish_callback.forget();
}

fn get_touch_start_callback() -> Closure<dyn FnMut(TouchEvent)> {
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

fn get_touch_move_callback() -> Closure<dyn FnMut(TouchEvent)> {
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

fn get_touch_end_callback(size: usize) -> Closure<dyn FnMut(TouchEvent)> {
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
