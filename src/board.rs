use std::ops::Add;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PuzzleBoardProps {
    pub fields: [u8; 9],
    pub width: usize,
    pub height: usize,
    pub on_click: Callback<usize>,
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board(
    PuzzleBoardProps {
        fields,
        on_click,
        width,
        height,
    }: &PuzzleBoardProps,
) -> Html {
    let on_click = on_click.clone();
    let fields_html: Html = fields
        .into_iter().enumerate()
        .map(|(idx, &field )| {
            let on_field_click = {
                let on_click = on_click.clone();
                Callback::from(move |_| on_click.emit(idx))
            };
            let (left, top) = get_left_top(idx, *width, 4);
            let (class, name) = match field {
                u8::MAX => ("empty-field", "".to_owned()),
                _ => ("field", format!("{}", field)),
            };
            html! {
                <div key={field} class={class} style={format!("left: {}rem; top: {}rem;", left, top)} onclick={on_field_click}>
                    {name}
                </div>
            }
        })
        .collect();

    html! {
        <div class="board">
            { fields_html }
        </div>
    }
}

fn get_left_top(idx: usize, width: usize, unit_size: usize) -> (usize, usize) {
    let (row, col): (usize, usize) = get_row_col_from_idx(idx, width);
    let left = col * unit_size;
    let top = row * unit_size;

    (left, top)
}

fn get_row_col_from_idx<T, U>(idx: T, width: T) -> (U, U)
where
    T: std::ops::Div<Output = T>,
    T: std::ops::Rem<Output = T>,
    T: Copy,
    U: std::convert::From<T>,
{
    let row = idx / width;
    let col = idx % width;

    (row.into(), col.into())
}

fn get_idx_from_row_col<T, U>(row: T, col: T, width: T) -> U
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    U: std::convert::From<T>,
{
    row.mul(width).add(col).into()
}

pub fn trigger_field(fields: &[u8; 9], width: usize, height: usize, clicked_idx: usize) -> [u8; 9] {
    let mut fields = fields.clone();

    if let Some(&u8::MAX) = fields.get(clicked_idx) {
        // Clicked on the empty field - unclear so nothing to do
        return fields;
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
            }
        }
    }
    fields
}

fn in_bounds<T, U>(row: T, col: T, width: U, height: U) -> bool
where
    T: PartialOrd<T>,
    T: PartialOrd<U>,
    T: Default,
{
    let t_zero: T = T::default();
    t_zero <= row && row < width && t_zero <= col && col < height
}

#[derive(Clone, PartialEq)]
pub struct Piece {
    pub id: u8,
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct PieceBoardProps {
    pub fields: Vec<Piece>,
    pub on_click: Callback<Piece>,
}

#[function_component(PieceBoard)]
pub fn field_board(PieceBoardProps { fields, on_click }: &PieceBoardProps) -> Html {
    let on_click = on_click.clone();
    let fields_html: Html = fields
        .iter()
        .map(|field| {
            let on_field_click = {
                let on_click = on_click.clone();
                let field = field.clone();
                Callback::from(move |_| on_click.emit(field.clone()))
            };
            html! {
                <div key={field.id} class="field" onclick={on_field_click}>
                    {field.name.clone()}
                </div>
            }
        })
        .collect();

    html! {
        <div class="field-board">
            { fields_html }
        </div>
    }
}
