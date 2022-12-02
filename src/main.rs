mod board;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use log::info;
use yew::prelude::*;

fn get_dimension_callbacks(
    width_state: &UseStateHandle<usize>,
    height_state: &UseStateHandle<usize>,
    fields: &UseStateHandle<Vec<u8>>,
) -> (Callback<InputEvent>, Callback<InputEvent>) {
    let on_width_change = {
        let width_state = width_state.clone();
        let height_state = height_state.clone();
        let fields = fields.clone();
        Callback::from(move |input_event: InputEvent| {
            if let Some(value) = input_event.data() {
                info!("Received width value {:?}", value);
                if let Ok(width) = value.parse::<usize>() {
                    info!("Parsed width value {:?}", width);
                    width_state.set(width);
                    fields.set(initialize_fields(width, *height_state));
                }
            }
        })
    };

    let on_height_change = {
        let width_state = width_state.clone();
        let height_state = height_state.clone();
        let fields = fields.clone();
        Callback::from(move |input_event: InputEvent| {
            if let Some(value) = input_event.data() {
                info!("Received height value {:?}", value);
                if let Ok(height) = value.parse::<usize>() {
                    info!("Parsed height value {:?}", height);
                    height_state.set(height);
                    fields.set(initialize_fields(*width_state, height));
                }
            }
        })
    };

    (on_width_change, on_height_change)
}

fn get_bg_callback(bg_url_state: &UseStateHandle<String>) -> Callback<InputEvent> {
    let bg_url_state = bg_url_state.clone();
    Callback::from(move |input_event: InputEvent| {
        if let Some(value) = input_event.data() {
            log::info!("Updating background URL to {}", &value);
            bg_url_state.set(value);
        }
    })
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
        info!("Clicked on field with index {}", clicked_idx);
        let updated_fields = trigger_field(&fields, *width_state, *height_state, clicked_idx);
        fields.set(updated_fields);
    })
}

#[derive(Properties, PartialEq)]
struct SettingsBlockProps {
    pub width_state: UseStateHandle<usize>,
    pub height_state: UseStateHandle<usize>,
    pub bg_url_state: UseStateHandle<String>,
    pub fields: UseStateHandle<Vec<u8>>,
}

#[function_component(SettingsBlock)]
fn settings_block(
    SettingsBlockProps {
        width_state,
        height_state,
        bg_url_state,
        fields,
    }: &SettingsBlockProps,
) -> Html {
    // Set up callback
    let (on_width_change, on_height_change) =
        get_dimension_callbacks(&width_state, &height_state, &fields);
    let on_bg_url_change = get_bg_callback(&bg_url_state);

    html! {
        <div class="settings">
            <div class="">
                <div class="settings-text">{ "Image URL" }</div>
                <input type="text" value={(&**bg_url_state).clone()} oninput={on_bg_url_change.clone()} />
            </div>
            <div class="dimensions">
                <div class="dimenions-block">
                    <div class="settings-text">{ "Width" }</div>
                    <input type="text" value={format!("{}", **width_state)} oninput={on_width_change.clone()} />
                </div>
                <div class="dimenions-block">
                    <div class="settings-text">{ "Height" }</div>
                    <input type="text" value={format!("{}", **height_state)} oninput={on_height_change.clone()} />
                </div>
            </div>
        </div>
    }
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
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
