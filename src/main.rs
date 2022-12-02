pub mod board;
pub mod settings;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use settings::SettingsBlock;
use yew::prelude::*;

fn get_field_click_callback(
    width_state: &UseStateHandle<usize>,
    height_state: &UseStateHandle<usize>,
    fields: &UseStateHandle<Vec<u8>>,
) -> Callback<usize> {
    let width_state = width_state.clone();
    let height_state = height_state.clone();
    let fields = fields.clone();
    Callback::from(move |clicked_idx: usize| {
        log::info!("Clicked on field with index {}", clicked_idx);
        let updated_fields = trigger_field(&fields, *width_state, *height_state, clicked_idx);
        fields.set(updated_fields);
    })
}

#[function_component(App)]
fn app() -> Html {
    // Set up state
    let width_state = use_state(|| 4usize);
    let height_state = use_state(|| 3usize);
    let bg_url_state = use_state(|| {
        "https://scr.wfcdn.de/21565/Imgur-Memes-des-Jahrzehnts-1579171161-0-0.jpg".to_owned()
    });
    let fields = use_state(|| initialize_fields(*width_state, *height_state));

    // Set up callback
    let on_field_click = get_field_click_callback(&width_state, &height_state, &fields);

    html! {
        <div class="content">
            <h1>{ "Shift Puzzle" }</h1>
            <button>{ "Shuffle" }</button>
            <PuzzleBoard
                fields={(&*fields).clone()}
                on_click={on_field_click.clone()}
                width={*width_state}
                height={*height_state}
                field_size={5}
                field_unit={"rem"}
                background_url={(&*bg_url_state).clone()}
            />

            <SettingsBlock
                width_state={width_state}
                height_state={height_state}
                bg_url_state={bg_url_state}
                fields={fields}
            />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
