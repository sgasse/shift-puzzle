use crate::board::{trigger_field, PuzzleBoard, PuzzleBoardProps};
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
    pub fields: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub background_url: String,
}

impl Component for ReactiveBoard {
    type Message = ReactiveBoardMsg;
    type Properties = ReactiveBoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            fields: props.fields.clone(),
            width: props.width,
            height: props.height,
            background_url: props.background_url.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Received message {:?}", msg);
        match msg {
            ReactiveBoardMsg::Swap((a, b)) => {
                self.fields.swap(a, b);
                true
            }
            ReactiveBoardMsg::ClickedField(clicked_idx) => {
                trigger_field(&mut self.fields, self.width, self.height, clicked_idx)
            }
            // Do not re-render
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ctx = ctx.clone();
        let inner_callbacks: Vec<_> = [(1, 2), (2, 3), (3, 4)]
            .into_iter()
            .map(|(a, b)| {
                ctx.link()
                    .callback(move |_: ()| ReactiveBoardMsg::Swap((a as usize, b as usize)))
            })
            .collect();
        let timed_callback = Callback::from(move |_| {
            let inner_callbacks = inner_callbacks.clone();
            for (i, inner_callback) in inner_callbacks.into_iter().enumerate() {
                let timeout = gloo_timers::callback::Timeout::new((i * 1000) as u32, move || {
                    inner_callback.emit(());
                });
                timeout.forget();
            }
        });

        let inner_callback = ctx
            .link()
            .callback(move |clicked_idx: usize| ReactiveBoardMsg::ClickedField(clicked_idx));
        let field_click_callback = Callback::from(move |clicked_idx: usize| {
            log::info!("Clicked on field with index {}", clicked_idx);
            inner_callback.emit(clicked_idx);
        });

        html! {
            <>
                <div>{ format!("Fields: {:?}", &self.fields)}</div>
                <div>{format!("Width: {}", &self.width)}</div>
                <div>{format!("Height: {}", &self.height)}</div>

                <button onclick={timed_callback}>{"Timed swaps"}</button>

                <PuzzleBoard
                    fields={self.fields.clone()}
                    on_click={field_click_callback}
                    width={self.width}
                    height={self.height}
                    field_size={5}
                    field_unit={"rem"}
                    background_url={self.background_url.clone()}
                />
            </>
        }
    }
}
