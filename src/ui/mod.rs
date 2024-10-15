use crate::UI_LOCKED;

pub(crate) mod board;
pub(crate) mod buttons;
pub(crate) mod search_params;
pub(crate) mod touch;

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

pub(crate) fn lock_ui() -> bool {
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

pub(crate) fn unlock_ui() {
    UI_LOCKED.with_borrow_mut(|locked| {
        if !*locked {
            log::warn!("Should unlock UI which was not locked");
        } else {
            *locked = false;
            log::debug!("Unlocked UI");
        }
    })
}

pub(crate) fn ui_locked() -> bool {
    UI_LOCKED.with(|locked| *locked.borrow())
}

// TODO: Change button colors when locking UI.
