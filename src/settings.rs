//! Settings module.
//!
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingsBlockProps {
    pub width: usize,
    pub height: usize,
    pub bg_url: String,
    pub width_callback: Callback<usize>,
    pub height_callback: Callback<usize>,
    pub bg_url_callback: Callback<String>,
}

#[function_component(SettingsBlock)]
pub fn settings_block(
    SettingsBlockProps {
        width,
        height,
        bg_url,
        width_callback,
        height_callback,
        bg_url_callback,
    }: &SettingsBlockProps,
) -> Html {
    html! {
        <div class="settings">
            <div class="image-settings">
                <div class="left-align-wrapper">
                    <div class="settings-text">{ "Image URL" }</div>
                    <input
                        type="text"
                        value={bg_url.clone()}
                        oninput={get_bg_callback(bg_url_callback.clone())}
                    />
                </div>
            </div>
            <div class="dimensions">
                <div class="left-align-wrapper">
                        <div class="settings-text">{ "Width" }</div>
                        <input
                            type="text"
                            value={format!("{width}")}
                            oninput={get_dimension_callback(width_callback.clone())}
                        />
                </div>
                <div class="left-align-wrapper">
                        <div class="settings-text">{ "Height" }</div>
                        <input
                            type="text"
                            value={format!("{height}")}
                            oninput={get_dimension_callback(height_callback.clone())}
                        />
                </div>
            </div>
        </div>
    }
}

fn get_dimension_callback(parent_callback: Callback<usize>) -> Callback<InputEvent> {
    Callback::from(move |input_event: InputEvent| {
        if let Some(value) = input_event.data() {
            log::info!("Received value {:?}", value);
            if let Ok(value) = value.parse::<usize>() {
                log::info!("Parsed value {:?}", value);
                parent_callback.emit(value);
            }
        }
    })
}

fn get_bg_callback(parent_callback: Callback<String>) -> Callback<InputEvent> {
    Callback::from(move |input_event: InputEvent| {
        if let Some(value) = input_event.data() {
            log::info!("Updating background URL to {}", &value);
            parent_callback.emit(value);
        }
    })
}
