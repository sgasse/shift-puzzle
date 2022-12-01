mod board;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let width = "4";
    let height = "3";
    let (width, height) = match (width.parse::<usize>(), height.parse::<usize>()) {
        (Ok(width), Ok(height)) => (width, height),
        _ => (3, 3),
    };
    let fields = initialize_fields(width, height);
    // let fields = [0, 1, 2, 3, 4, 5, 6, 7, u8::MAX];
    let fields = use_state(|| fields);

    let on_field_click = {
        let fields = fields.clone();
        Callback::from(move |clicked_idx: usize| {
            info!("Clicked on field with index {}", clicked_idx);
            let updated_fields = trigger_field(&fields, width, height, clicked_idx);
            fields.set(updated_fields);
        })
    };

    html! {
        <>
            <h1>{ "Shift Puzzle" }</h1>
            <input type="text" value="bla" />
            <input type="text" value="3" />
            <input type="text" value="3" />
            <PuzzleBoard
                fields={(&*fields).clone()}
                on_click={on_field_click.clone()}
                width={width as usize}
                height={height as usize}
                field_size={5}
                field_unit={"rem"}
                background_url={"https://scr.wfcdn.de/21565/Imgur-Memes-des-Jahrzehnts-1579171161-0-0.jpg"}
            />
        </>
    }
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
