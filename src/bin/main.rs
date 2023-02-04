//! Frontend entrypoint.
//!
use slide_puzzle::slide_puzzle::SlidePuzzle;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // Default values
    let background_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/6/61/Blue_Marble_Western_Hemisphere.jpg/600px-Blue_Marble_Western_Hemisphere.jpg?20130305115950".to_owned();
    let width = 3;
    let height = 3;

    html! {
        <div class="content">
            <div class="header">{ "Slide Puzzle" }</div>
            <SlidePuzzle {width} {height} {background_url} />
        </div>
    }
}
