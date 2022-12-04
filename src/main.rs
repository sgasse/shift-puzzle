pub mod board;
pub mod settings;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use rand::seq::SliceRandom;
use rand::thread_rng;
use settings::SettingsBlock;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // Set up state
    let width_state = use_state(|| 4usize);
    let height_state = use_state(|| 3usize);
    let bg_url_state = use_state(|| {
        "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Sweet_Bread_Mountain.jpg/640px-Sweet_Bread_Mountain.jpg".to_owned()
    });
    let fields = use_state(|| initialize_fields(*width_state, *height_state));

    // Set up callbacks
    let on_field_click = get_field_click_callback(&width_state, &height_state, &fields);
    let on_shuffle_click = get_shuffle_callback(&fields);

    html! {
        <div class="content">
            <div class="header">{ "Shift Puzzle" }</div>
            <PuzzleBoard
                fields={(&*fields).clone()}
                on_click={on_field_click.clone()}
                width={*width_state}
                height={*height_state}
                field_size={5}
                field_unit={"rem"}
                background_url={(&*bg_url_state).clone()}
            />
            <button onclick={on_shuffle_click.clone()}>{ "Shuffle" }</button>

            <SettingsBlock
                width_state={width_state}
                height_state={height_state}
                bg_url_state={bg_url_state}
                fields={fields}
            />
        </div>
    }
}

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

fn get_shuffle_callback(fields: &UseStateHandle<Vec<u8>>) -> Callback<MouseEvent> {
    let fields = fields.clone();
    Callback::from(move |_| {
        let fields = fields.clone();
        let timeout = gloo_timers::callback::Timeout::new(1_000, move || {
            log::info!("Shuffling fields");
            let mut updated_fields = (&*fields).clone();
            updated_fields.shuffle(&mut thread_rng());
            fields.set(updated_fields);
        });
        timeout.forget();
    })
}
