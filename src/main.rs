pub mod board;
pub mod reactive_board;
pub mod settings;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use reactive_board::ReactiveBoard;
use settings::SettingsBlock;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let background_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Sweet_Bread_Mountain.jpg/640px-Sweet_Bread_Mountain.jpg".to_owned();

    // Set up state
    let width_state = use_state(|| 4usize);
    let height_state = use_state(|| 3usize);
    let bg_url_state = use_state(|| {
        "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Sweet_Bread_Mountain.jpg/640px-Sweet_Bread_Mountain.jpg".to_owned()
    });
    let fields = use_state(|| initialize_fields(*width_state, *height_state));

    let width = 4;
    let height = 3;
    let fields_vec = initialize_fields(width, height);

    html! {
        <div class="content">
            <div class="header">{ "Shift Puzzle" }</div>
            <ReactiveBoard fields={fields_vec} {width} {height} {background_url} />

            <SettingsBlock
                width_state={width_state}
                height_state={height_state}
                bg_url_state={bg_url_state}
                fields={fields}
            />
        </div>
    }
}
