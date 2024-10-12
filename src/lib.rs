//! Slide puzzle frontend and solvers.
//!

use std::cell::RefCell;

use board::Board;
use buttons::setup_button_callbacks;
use utils::{extract_parameters, set_panic_hook, TouchCoords};
use wasm_bindgen::prelude::*;

pub mod board;
pub mod buttons;
pub mod solver;
pub mod utils;

pub type Error = Box<dyn std::error::Error>;

thread_local! {
    static UI_LOCKED: RefCell<bool> = const { RefCell::new(true) };
    static BOARD: RefCell<Board> = const { RefCell::new(Board::new()) };
    static TOUCH_COORDS: RefCell<TouchCoords> = const { RefCell::new(TouchCoords::new()) };
}

#[wasm_bindgen]
pub fn wasm_main() {
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");

    let params = extract_parameters();
    log::debug!("Params: {:?}", params);

    setup_button_callbacks(params.size);

    BOARD.with_borrow_mut(|b| {
        b.init(params);
    });

    unlock_ui();
}

fn lock_ui() -> bool {
    UI_LOCKED.with_borrow_mut(|locked| {
        if *locked {
            log::debug!("UI is locked");
            false
        } else {
            *locked = true;
            log::debug!("Locked UI");
            true
        }
    })
}

fn unlock_ui() {
    UI_LOCKED.with_borrow_mut(|locked| {
        if !*locked {
            log::warn!("Should unlock UI which was not locked");
        } else {
            *locked = false;
            log::debug!("Unlocked UI");
        }
    })
}

fn ui_locked() -> bool {
    UI_LOCKED.with(|locked| *locked.borrow())
}

// TODO: Solver not attempting / button greyed out at a certain size
