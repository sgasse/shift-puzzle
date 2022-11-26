mod piece;

use log::info;
use piece::{Piece, PieceBoard};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let pieces: Vec<_> = (0..4)
        .into_iter()
        .map(|num| Piece {
            id: num,
            name: format!("Piece {}", num),
        })
        .collect();

    let pieces = use_state(|| pieces);

    let on_piece_click = {
        // let clicked_piece = clicked_piece.clone();
        Callback::from(move |piece: Piece| {
            info!("Got piece {}", piece.id);
            // clicked_piece.set(Some(piece));
        })
    };

    html! {
        <>
            <h1>{ "Hello Yew App x2!" }</h1>
            <PieceBoard pieces={} on_click={on_piece_click.clone()} />
        </>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
