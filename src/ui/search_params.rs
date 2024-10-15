use std::collections::BTreeMap;

use web_sys::window;

const DEFAULT_SIZE: usize = 3;
const DEFAULT_BACKGROUND: &str = "https://upload.wikimedia.org/wikipedia/commons/thumb/6/61/Blue_Marble_Western_Hemisphere.jpg/600px-Blue_Marble_Western_Hemisphere.jpg?20130305115950";

pub(crate) fn extract_parameters() -> Parameters {
    let params = search_params()
        .map(|s| {
            BTreeMap::from_iter(
                s.split('&')
                    .filter_map(|s| s.split_once('=').map(|(k, v)| (k.to_owned(), v.to_owned()))),
            )
        })
        .unwrap_or_default();

    let size: usize = params
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SIZE);

    let bg_url = params
        .get("bg_url")
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| DEFAULT_BACKGROUND.to_owned());

    Parameters { size, bg_url }
}

#[derive(Debug)]
pub(crate) struct Parameters {
    pub(crate) size: usize,
    pub(crate) bg_url: String,
}

pub(crate) fn search_params() -> Option<String> {
    window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.trim_start_matches('?').replace("%22", "").to_owned())
}
