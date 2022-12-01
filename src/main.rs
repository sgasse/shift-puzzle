mod board;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let width_state = use_state(|| 4usize);
    let height_state = use_state(|| 3usize);
    let bg_url_state = use_state(|| {
        "https://scr.wfcdn.de/21565/Imgur-Memes-des-Jahrzehnts-1579171161-0-0.jpg".to_owned()
    });

    let fields = use_state(|| initialize_fields(*width_state, *height_state));

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

    let on_bg_url_change = {
        let bg_url_state = bg_url_state.clone();
        Callback::from(move |input_event: InputEvent| {
            if let Some(value) = input_event.data() {
                log::info!("Updating background URL to {}", &value);
                bg_url_state.set(value);
            }
        })
    };

    let on_field_click = {
        let width_state = width_state.clone();
        let height_state = height_state.clone();
        let fields = fields.clone();
        Callback::from(move |clicked_idx: usize| {
            info!("Clicked on field with index {}", clicked_idx);
            let updated_fields = trigger_field(&fields, *width_state, *height_state, clicked_idx);
            fields.set(updated_fields);
        })
    };

    html! {
        <>
            <h1>{ "Shift Puzzle" }</h1>
            <input type="text" value={(&*bg_url_state).clone()} oninput={on_bg_url_change.clone()} />
            <input type="text" value={format!("{}", *width_state)} oninput={on_width_change.clone()} />
            <input type="text" value={format!("{}", *height_state)} oninput={on_height_change.clone()} />
            <PuzzleBoard
                fields={(&*fields).clone()}
                on_click={on_field_click.clone()}
                width={*width_state}
                height={*height_state}
                field_size={5}
                field_unit={"rem"}
                background_url={(&*bg_url_state).clone()}
            />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Logger initialized");
    yew::Renderer::<App>::new().render();
}
