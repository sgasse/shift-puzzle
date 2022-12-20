use shift_puzzle::reactive_board::ReactiveBoard;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // Default values
    let background_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Sweet_Bread_Mountain.jpg/640px-Sweet_Bread_Mountain.jpg".to_owned();
    let width = 4;
    let height = 3;

    html! {
        <div class="content">
            <div class="header">{ "Shift Puzzle" }</div>
            <ReactiveBoard {width} {height} {background_url} />
        </div>
    }
}
