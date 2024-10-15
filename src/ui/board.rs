use wasm_bindgen::{prelude::Closure, JsCast as _};
use web_sys::{window, CssStyleDeclaration, Document, MouseEvent, Node};

use crate::{
    board::{get_row_col_from_idx, Board},
    ui::{search_params::Parameters, ui_locked},
    BOARD,
};

pub(crate) struct UiBoard {
    inner: Board,
}

impl UiBoard {
    pub(crate) const fn new() -> Self {
        Self {
            inner: Board::new(),
        }
    }

    pub(crate) fn board(&self) -> &Board {
        &self.inner
    }

    pub(crate) fn init(&mut self, params: Parameters) {
        self.inner.init(params.size);
        self.init_board_ui(params);
    }

    fn swap_ui_fields(&mut self, id_a: u8, id_b: u8) {
        // Swap positions in style.
        let document = window().unwrap().document().unwrap();

        let (a_style, a_top, a_left) = get_style_top_left(&document, id_a);
        let (b_style, b_top, b_left) = get_style_top_left(&document, id_b);

        a_style.set_property("top", &b_top).unwrap();
        a_style.set_property("left", &b_left).unwrap();
        b_style.set_property("top", &a_top).unwrap();
        b_style.set_property("left", &a_left).unwrap();
    }

    pub(crate) fn swap_indices(&mut self, idx_a: usize, idx_b: usize) {
        let id_a = self.inner.indices2ids()[idx_a];
        let id_b = self.inner.indices2ids()[idx_b];

        self.swap_ui_fields(id_a, id_b);
        self.inner.swap_ids(id_a, id_b);
    }

    fn swap_ids(&mut self, id_a: u8, id_b: u8) {
        self.swap_ui_fields(id_a, id_b);
        self.inner.swap_ids(id_a, id_b);
    }

    fn init_board_ui(&self, params: Parameters) {
        let document = window().unwrap().document().unwrap();
        let board = document.get_element_by_id("board").unwrap();

        // Adjust field size depending on puzzle size
        let field_size = 12 / params.size;

        for id in self.inner.indices2ids().iter() {
            let div = create_div(*id, params.size, field_size, &params.bg_url);
            board.append_child(&div).unwrap();
        }
    }
}

fn create_div(id: u8, board_size: usize, field_size: usize, background_url: &str) -> Node {
    let div = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlDivElement>()
        .unwrap();

    // Set class for proper general style.
    div.set_class_name("field");
    // Set field ID.
    div.set_id(&format!("{id}"));
    // Set onclick callback.
    let onclick_callback = get_onclick_closure(id as usize, board_size);
    div.set_onclick(Some(onclick_callback.as_ref().unchecked_ref()));
    // Do not drop the onclick callback by leaking its memory.
    onclick_callback.forget();

    // Get position on board. The ID equals the index here.
    let (left, top) = get_left_top_pos(id as usize, board_size, field_size);

    let style = div.style();

    if is_empty_field(id as usize, board_size) {
        // Set positioning with empty background.
        style.set_css_text(&format!(
            "left: {}rem; top: {}rem; width: {}rem; height: {}rem; \
             position: absolute; transition: all 0.2s; z-index: -1",
            left, top, field_size, field_size,
        ));
    } else {
        // Scale and position background to match tile on board.
        let background_x = board_size * field_size - left;
        let background_y = board_size * field_size - top;
        let background_size = board_size * field_size;

        style.set_css_text(&format!(
            "left: {}rem; top: {}rem; width: {}rem; height: {}rem; \
         position: absolute; transition: all 0.2s; \
         background-position: {}rem {}rem; background-size: {}rem {}rem; \
         background-image:url({})",
            left,
            top,
            field_size,
            field_size,
            background_x,
            background_y,
            background_size,
            background_size,
            background_url
        ));
    }

    div.dyn_into::<web_sys::Node>().unwrap()
}

fn get_onclick_closure(clicked_id: usize, size: usize) -> Closure<dyn FnMut(MouseEvent)> {
    if is_empty_field(clicked_id, size) {
        // Unclear which field to swap with the empty field so skip.
        log::debug!("Received a click on ID {clicked_id} (empty field)");
        return Closure::wrap(Box::new(|_| ()));
    }

    Closure::wrap(Box::new(move |_event: MouseEvent| {
        log::debug!("Received a click on ID {clicked_id}");

        if ui_locked() {
            log::debug!("UI is locked");
            return;
        }

        if let Some(empty_id) = is_swappable_with_empty(clicked_id, size) {
            log::info!("Swapping ID {clicked_id} with empty ID {empty_id}");
            BOARD.with_borrow_mut(|b| {
                b.swap_ids(clicked_id as u8, empty_id as u8);
            });
        }
    }))
}

fn get_style_top_left(document: &Document, div_id: u8) -> (CssStyleDeclaration, String, String) {
    let element = document
        .get_element_by_id(&format!("{div_id}"))
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let style = element.style();
    let top = style.get_property_value("top").unwrap();
    let left = style.get_property_value("left").unwrap();
    (style, top, left)
}

/// Get the left/top coordinates based on the index of a board field.
fn get_left_top_pos(idx: usize, width: usize, unit_size: usize) -> (usize, usize) {
    let (row, col): (usize, usize) = get_row_col_from_idx(idx, width);
    let left = col * unit_size;
    let top = row * unit_size;

    (left, top)
}

fn is_empty_field(clicked_id: usize, size: usize) -> bool {
    clicked_id == (size * size - 1)
}

fn is_swappable_with_empty(clicked_id: usize, size: usize) -> Option<usize> {
    BOARD.with_borrow(|b| {
        let empty_id = b.board().ids2indices().len() - 1;
        let clicked_idx = b.board().ids2indices()[clicked_id];
        let empty_idx = b.board().ids2indices()[empty_id];

        if is_swappable_neighbour(clicked_idx, empty_idx, size) {
            Some(empty_id)
        } else {
            None
        }
    })
}

fn is_swappable_neighbour(idx_a: usize, idx_b: usize, size: usize) -> bool {
    let (row_a, col_a): (isize, isize) = get_row_col_from_idx(idx_a as isize, size as isize);
    let (row_b, col_b): (isize, isize) = get_row_col_from_idx(idx_b as isize, size as isize);

    (row_a.abs_diff(row_b) + col_a.abs_diff(col_b)) == 1
}
