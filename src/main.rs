mod board;

use board::{initialize_fields, trigger_field, PuzzleBoard};
use log::info;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // Width and height states + callbacks
    let width_txt = use_state(|| "4".to_owned());
    let height_txt = use_state(|| "3".to_owned());

    // let width_val = use_state(|| match width_txt.parse::<usize>() {
    //     Ok(width) => width,
    //     _ => 3,
    // });

    let (width, height) = match (width_txt.parse::<usize>(), height_txt.parse::<usize>()) {
        (Ok(width), Ok(height)) => (width, height),
        _ => (3, 3),
    };
    let fields = use_state(|| initialize_fields(width, height));

    let on_width_change = {
        let width_txt = width_txt.clone();
        let fields = fields.clone();
        Callback::from(move |input_event: InputEvent| {
            if let Some(value) = input_event.data() {
                info!("Received width value {:?}", value);
                info!("Fields {:?}", *fields);
                width_txt.set(value);
            }
        })
    };

    let on_field_click = {
        let fields = fields.clone();
        // let width_val = width_val.clone();
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
            <input type="text" value={(&*width_txt).clone()} oninput={on_width_change.clone()}/>
            <input type="text" value={(&*height_txt).clone()} />
            <PuzzleBoard
                fields={(&*fields).clone()}
                on_click={on_field_click.clone()}
                width={(width) as usize}
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
