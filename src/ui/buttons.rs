use wasm_bindgen::prelude::*;
use web_sys::{window, CssStyleSheet, HtmlElement, MouseEvent};

use crate::{
    board::{get_empty_field_idx, get_shuffle_sequence},
    solver::{divide_and_conquer::DacPuzzleSolver, optimal::find_swap_order},
    ui::{
        lock_ui,
        touch::{get_touch_end_callback, get_touch_move_callback, get_touch_start_callback},
    },
    unlock_ui, BOARD,
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

        let empty_field_idx =
            BOARD.with_borrow(|b| get_empty_field_idx(b.board().indices2ids()).unwrap());

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
        let empty_field_idx =
            BOARD.with_borrow(|b| get_empty_field_idx(b.board().indices2ids()).unwrap());

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

        // TODO: Solver aborting after a certain size?
        let ids = BOARD.with_borrow(|b| b.board().indices2ids().clone());
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

        let ids = BOARD.with_borrow(|b| b.board().indices2ids().clone());
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

const ACTIVE_BUTTON_COLOR: &str = "rgb(233, 233, 237)";
const INACTIVE_BUTTON_COLOR: &str = "rgb(208, 208, 215)";

pub(crate) fn deactivate_buttons() {
    replace_in_button_style(&[
        ("pointer", "wait"),
        (ACTIVE_BUTTON_COLOR, INACTIVE_BUTTON_COLOR),
    ]);
}

pub(crate) fn activate_buttons() {
    replace_in_button_style(&[
        ("wait", "pointer"),
        (INACTIVE_BUTTON_COLOR, ACTIVE_BUTTON_COLOR),
    ]);
}

fn replace_in_button_style(replaces: &[(&str, &str)]) {
    let document = window().unwrap().document().unwrap();
    let style_sheets = document.style_sheets();

    // The global style sheet is the first one.
    if let Some(global_sheet) = style_sheets.get(0) {
        let global_css = global_sheet.dyn_into::<CssStyleSheet>().unwrap();
        let rule_list = global_css.css_rules().unwrap();

        // Iterate rules until we find the rule for the button class.
        for i in 0..rule_list.length() {
            match rule_list.get(i) {
                Some(rule) if rule.css_text().contains("button {") => {
                    let existing_rule = rule.css_text();
                    let updated_rule = replaces
                        .iter()
                        .fold(existing_rule, |rule, (old, new)| rule.replace(old, new));
                    log::debug!("Updating to {updated_rule}");
                    // Replace existing with updated rule in two steps.
                    // `replace` would be better but returns a `Promise` which we do not want to handle in this context.
                    global_css.delete_rule(i).unwrap();
                    global_css.insert_rule(&updated_rule).unwrap();
                    return;
                }
                _ => (),
            }
        }
    }
}
