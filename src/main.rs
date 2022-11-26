mod piece;

use log::info;
use piece::PuzzleBoard;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let fields = [0, 1, 2, 3];
    let fields = use_state(|| fields);

    let on_field_click = {
        let fields = fields.clone();
        Callback::from(move |field: u8| {
            info!("Got field {}", field);
            if field == 0 {
                fields.set([0, 1, 2, 3]);
            } else {
                fields.set([3, 2, 1, 0]);
            }
        })
    };

    html! {
        <>
            <h1>{ "Hello Yew App x2!" }</h1>
            <PuzzleBoard fields={*fields} on_click={on_field_click.clone()} />
        </>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
