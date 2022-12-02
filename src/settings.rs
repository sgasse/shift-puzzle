use crate::board::initialize_fields;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingsBlockProps {
    pub width_state: UseStateHandle<usize>,
    pub height_state: UseStateHandle<usize>,
    pub bg_url_state: UseStateHandle<String>,
    pub fields: UseStateHandle<Vec<u8>>,
}

#[function_component(SettingsBlock)]
pub fn settings_block(
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
                log::info!("Received width value {:?}", value);
                if let Ok(width) = value.parse::<usize>() {
                    log::info!("Parsed width value {:?}", width);
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
                log::info!("Received height value {:?}", value);
                if let Ok(height) = value.parse::<usize>() {
                    log::info!("Parsed height value {:?}", height);
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
