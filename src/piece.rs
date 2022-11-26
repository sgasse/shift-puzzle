use yew::prelude::*;

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
