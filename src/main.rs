mod board;

use board::{trigger_field, PuzzleBoard};
use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let fields = [0, 1, 2, 3, 4, 5, 6, 7, u8::MAX];
    let fields = use_state(|| fields);

    let on_field_click = {
        let fields = fields.clone();
        Callback::from(move |clicked_idx: usize| {
            info!("Clicked on field with index {}", clicked_idx);
            let updated_fields = trigger_field(&fields, 3, 3, clicked_idx);
            fields.set(updated_fields);
        })
    };

    html! {
        <>
            <h1>{ "Shift Puzzle" }</h1>
            <PuzzleBoard fields={*fields} on_click={on_field_click.clone()} width={3} height={3} />
        </>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
