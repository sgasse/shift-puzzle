use crate::board::{
    get_empty_field_idx, get_shuffle_sequence, initialize_fields, trigger_field, PuzzleBoard,
};
use crate::expander::Expander;
use crate::settings::SettingsBlock;
use yew::prelude::*;

#[derive(Debug)]
pub enum ReactiveBoardMsg {
    CompleteFieldsUpdate(Vec<u8>),
    WidthUpdate(usize),
    HeightUpdate(usize),
    Swap((usize, usize)),
    ClickedField(usize),
    BackgroundUrlUpdate(String),
}

pub struct ReactiveBoard {
    fields: Vec<u8>,
    width: usize,
    height: usize,
    background_url: String,
}

#[derive(Properties, PartialEq)]
pub struct ReactiveBoardProps {
    pub width: usize,
    pub height: usize,
    pub background_url: String,
}

impl Component for ReactiveBoard {
    type Message = ReactiveBoardMsg;
    type Properties = ReactiveBoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let fields = initialize_fields(props.width * props.height);
        Self {
            fields,
            width: props.width,
            height: props.height,
            background_url: props.background_url.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Received message {:?}", msg);
        match msg {
            ReactiveBoardMsg::Swap((a, b)) => match a != b {
                true => {
                    self.fields.swap(a, b);
                    true
                }
                false => false,
            },
            ReactiveBoardMsg::ClickedField(clicked_idx) => {
                trigger_field(&mut self.fields, self.width, self.height, clicked_idx)
            }
            ReactiveBoardMsg::WidthUpdate(width) => match width != self.width {
                true => {
                    self.width = width;
                    self.fields = initialize_fields(self.width * self.height);
                    true
                }
                false => false,
            },
            ReactiveBoardMsg::HeightUpdate(height) => match height != self.height {
                true => {
                    self.height = height;
                    self.fields = initialize_fields(self.width * self.height);
                    true
                }
                false => false,
            },
            ReactiveBoardMsg::BackgroundUrlUpdate(bg_url) => match bg_url != self.background_url {
                true => {
                    self.background_url = bg_url;
                    true
                }
                false => false,
            },
            // Do not re-render
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ctx = ctx.clone();
        let timed_callback = self.get_button_callbacks(ctx);

        let field_click_callback = ctx
            .link()
            .callback(move |clicked_idx: usize| ReactiveBoardMsg::ClickedField(clicked_idx));

        let width_change_callback = ctx
            .link()
            .callback(move |width: usize| ReactiveBoardMsg::WidthUpdate(width));
        let height_change_callback = ctx
            .link()
            .callback(move |height: usize| ReactiveBoardMsg::HeightUpdate(height));
        let bg_url_change_callback = ctx
            .link()
            .callback(move |bg_url: String| ReactiveBoardMsg::BackgroundUrlUpdate(bg_url));

        html! {
            <>
                <PuzzleBoard
                    fields={self.fields.clone()}
                    on_click={field_click_callback}
                    width={self.width}
                    height={self.height}
                    field_size={5}
                    field_unit={"rem"}
                    background_url={self.background_url.clone()}
                />

                <button onclick={timed_callback}>{"Shuffle"}</button>

                <Expander title={"Settings"}>
                    <SettingsBlock
                        width={self.width}
                        height={self.height}
                        bg_url={self.background_url.clone()}
                        width_callback={width_change_callback}
                        height_callback={height_change_callback}
                        bg_url_callback={bg_url_change_callback}
                    />
                </Expander>

                <Expander title={"Debug"}>
                    <div>{ format!("Fields: {:?}", &self.fields)}</div>
                    <div>{format!("Width: {}", &self.width)}</div>
                    <div>{format!("Height: {}", &self.height)}</div>
                </Expander>
            </>
        }
    }
}

impl ReactiveBoard {
    fn get_button_callbacks(&self, ctx: &Context<ReactiveBoard>) -> Callback<MouseEvent> {
        // Create a callback to send a swap message that can be passed into
        // closures
        let swap_callback = ctx.link().callback(move |swap_pair: (usize, usize)| {
            ReactiveBoardMsg::Swap((swap_pair.0, swap_pair.1))
        });

        // Locally-bind values of self that we want to pass into the closure
        let empty_field_idx = get_empty_field_idx(&self.fields);
        let width = self.width;
        let height = self.height;

        let timed_callback = Callback::from(move |_| {
            // Calculate a shuffle sequence only when the button is clicked, not
            // on every re-render
            let shuffle_sequence = get_shuffle_sequence(width, height, empty_field_idx, 20);
            log::info!("Shuffle sequence: {:?}", &shuffle_sequence);

            let swap_callback = swap_callback.clone();

            for (i, swap) in shuffle_sequence.into_iter().enumerate() {
                let swap_callback = swap_callback.clone();
                let timeout = gloo_timers::callback::Timeout::new((i * 250) as u32, move || {
                    swap_callback.emit((swap.0, swap.1));
                });
                timeout.forget();
            }
        });
        timed_callback
    }
}
