//! Slide puzzle frontend and solvers.
//!

use std::cell::RefCell;

use ui::{
    board::UiBoard, buttons::setup_button_callbacks, search_params::extract_parameters,
    set_panic_hook, touch::TouchCoords, unlock_ui,
};
use wasm_bindgen::prelude::*;

pub mod board;
pub mod error;
pub mod solver;
pub mod ui;

pub type Error = Box<dyn std::error::Error>;

thread_local! {
    static UI_LOCKED: RefCell<bool> = const { RefCell::new(true) };
    static BOARD: RefCell<UiBoard> = const { RefCell::new(UiBoard::new()) };
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
