use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PuzzleBoardProps {
    pub fields: [u8; 4],
    pub on_click: Callback<u8>,
}

#[function_component(PuzzleBoard)]
pub fn puzzle_board(PuzzleBoardProps { fields, on_click }: &PuzzleBoardProps) -> Html {
    let on_click = on_click.clone();
    let fields_html: Html = fields
        .into_iter().enumerate()
        .map(|(idx, &field )| {
            let on_piece_click = {
                let on_click = on_click.clone();
                Callback::from(move |_| on_click.emit(field))
            };
            html! {
                <div key={field} class="piece" style={format!("left: {}rem", idx*4)} onclick={on_piece_click}>
                    {field}
                </div>
            }
        })
        .collect();

    html! {
        <div class="piece-board">
            { fields_html }
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct Piece {
    pub id: u8,
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct PieceBoardProps {
    pub pieces: Vec<Piece>,
    pub on_click: Callback<Piece>,
}

#[function_component(PieceBoard)]
pub fn piece_board(PieceBoardProps { pieces, on_click }: &PieceBoardProps) -> Html {
    let on_click = on_click.clone();
    let pieces_html: Html = pieces
        .iter()
        .map(|piece| {
            let on_piece_click = {
                let on_click = on_click.clone();
                let piece = piece.clone();
                Callback::from(move |_| on_click.emit(piece.clone()))
            };
            html! {
                <div key={piece.id} class="piece" onclick={on_piece_click}>
                    {piece.name.clone()}
                </div>
            }
        })
        .collect();

    html! {
        <div class="piece-board">
            { pieces_html }
        </div>
    }
}
