mod utils;
pub use utils::*;

mod slide;
pub use slide::*;

use rand::seq::SliceRandom;
use web_sys::TouchEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PuzzleBoardProps {
    pub fields: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub field_size: usize,
    pub field_unit: &'static str,
    pub background_url: String,
    pub on_click: Callback<usize>,
    pub on_touch_start: Callback<(i32, i32)>,
    pub on_touch_end: Callback<(i32, i32)>,
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board(
    PuzzleBoardProps {
        fields,
        width,
        height,
        field_size,
        field_unit,
        background_url,
        on_click,
        on_touch_start,
        on_touch_end,
    }: &PuzzleBoardProps,
) -> Html {
    let on_click = on_click.clone();

    log::info!(
        "Rendering puzzle board with width {}, height {}, fields {:?}",
        width,
        height,
        &fields
    );

    // Callback to concatenate a size value with the given unit
    let as_unit = |value: usize| format!("{}{}", value, field_unit);

    // Enumerate values and sort by fields. This is required so that every
    // field shows up at the same list index in the DOM regardless of its left/
    // right value. Otherwise, elements would be recreated and the animation
    // state lost.
    let mut indexes_fields: Vec<_> = fields.into_iter().enumerate().collect();
    indexes_fields.sort_by(|a, b| b.1.cmp(&a.1));

    let fields_html: Html = indexes_fields
        .into_iter()
        .map(|(idx, &field)| {
            let on_field_click = {
                let on_click = on_click.clone();
                Callback::from(move |_| {
                    log::info!("Clicked on field with index {}", idx);
                    on_click.emit(idx);
                })
            };
            let bg_string = format!(
                "background-size: {} {}; \
                 background-image: url({});",
                as_unit(width * field_size),
                as_unit(height * field_size),
                background_url
            );
            let (left_pos, top_pos) = get_left_top(idx, *width, *field_size);
            let (left_img, top_img) = get_left_top(field as usize, *width, *field_size);

            let field_props = match field {
                u8::MAX => FieldProps {
                    left_pos,
                    top_pos,
                    img_pos_x: 0,
                    img_pos_y: 0,
                    class: "empty-field",
                    name: "".to_owned(),
                    bg_str: "".to_owned(),
                },
                _ => FieldProps {
                    left_pos,
                    top_pos,
                    img_pos_x: width * field_size - left_img,
                    img_pos_y: height * field_size - top_img,
                    class: "field",
                    name: format!("{}", field),
                    bg_str: bg_string,
                },
            };

            html! {
                <div
                    key={field}
                    class={field_props.class}
                    style={format!("left: {}; \
                                    top: {}; \
                                    width: {}; \
                                    height: {}; \
                                    position: absolute; \
                                    transition: all 0.2s; \
                                    background-position: {} {}; \
                                    {}",
                                    as_unit(field_props.left_pos),
                                    as_unit(field_props.top_pos),
                                    as_unit(*field_size),
                                    as_unit(*field_size),
                                    as_unit(field_props.img_pos_x),
                                    as_unit(field_props.img_pos_y),
                                    field_props.bg_str)}
                    onclick={on_field_click}
                >
                    // Maybe optionally display field index?
                    // {field_props.name}
                </div>
            }
        })
        .collect();

    let on_touch_start = on_touch_start.clone();
    let on_touch_start = Callback::from(move |event: TouchEvent| {
        if let Some(touch) = event.changed_touches().get(0) {
            let coords = (touch.client_x(), touch.client_y());
            on_touch_start.emit(coords);
        }
    });

    let on_touch_end = on_touch_end.clone();
    let on_touch_end = Callback::from(move |event: TouchEvent| {
        if let Some(touch) = event.changed_touches().get(0) {
            let coords = (touch.client_x(), touch.client_y());
            on_touch_end.emit(coords);
        }
    });

    html! {
        <div
            class="board"
            style={format!("width: {}; \
                            height: {}; \
                            position: relative;",
                            as_unit(width*field_size),
                            as_unit(height*field_size))}
            ontouchstart={on_touch_start}
            ontouchend={on_touch_end}
        >
            { fields_html }
        </div>
    }
}

pub fn initialize_fields(num_elements: usize) -> Vec<u8> {
    let num_elements = usize::min(num_elements, u8::MAX as usize) as u8;
    let mut fields: Vec<_> = (0..(num_elements - 1)).into_iter().collect();
    fields.push(u8::MAX);
    fields
}

struct FieldProps {
    left_pos: usize,
    top_pos: usize,
    img_pos_x: usize,
    img_pos_y: usize,
    class: &'static str,
    name: String,
    bg_str: String,
}

/// Get a sequence of valid semi-random shuffles.
///
/// We prevent fields from being shuffled back and forth, which breaks total
/// randomness.
pub fn get_shuffle_sequence(
    width: usize,
    height: usize,
    mut empty_field_idx: usize,
    num_swaps: usize,
) -> Vec<(usize, usize)> {
    let mut swaps = Vec::with_capacity(num_swaps);

    // We want to avoid swapping fields back and forth like (2, 1), (1, 2)
    // Our approach is to remove the previous empty field from swappable
    // neighbours
    let mut prev_empty_field_idx = empty_field_idx;

    for _ in 0..num_swaps {
        let swappable_neighbours: Vec<_> = get_swappable_neighbours(width, height, empty_field_idx)
            .into_iter()
            .filter(|&element| element != prev_empty_field_idx)
            .collect();
        let chosen_neighbour = swappable_neighbours
            .choose(&mut rand::thread_rng())
            .expect("Neighbour");
        swaps.push((empty_field_idx, *chosen_neighbour));
        prev_empty_field_idx = empty_field_idx;
        empty_field_idx = *chosen_neighbour;
    }

    swaps
}

/// Get the indices of neighbours that can be swapped with the empty field.
pub fn get_swappable_neighbours(width: usize, height: usize, empty_field_idx: usize) -> Vec<usize> {
    let (row, col): (usize, usize) = get_row_col_from_idx(empty_field_idx, width);

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
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
        .collect()
}
