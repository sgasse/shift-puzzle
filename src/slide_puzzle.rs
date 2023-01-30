use yew::prelude::*;

use crate::{
    board::{
        get_empty_field_idx, get_shuffle_sequence, get_touch_direction, handle_field_click,
        handle_touch_move, initialize_fields, PuzzleBoard,
    },
    expander::Expander,
    settings::SettingsBlock,
    solver::{divide_and_conquer::DacPuzzleSolver, optimal::find_swap_order},
};

#[derive(Debug)]
pub enum SlidePuzzleMsg {
    CompleteFieldsUpdate(Vec<u8>),
    WidthUpdate(usize),
    HeightUpdate(usize),
    Swap((usize, usize)),
    ClickedField(usize),
    BackgroundUrlUpdate(String),
    TouchStartCoords((i32, i32)),
    TouchEndCoords((i32, i32)),
}

pub struct SlidePuzzle {
    fields: Vec<u8>,
    width: usize,
    height: usize,
    background_url: String,
    touch_start_coords: Option<(i32, i32)>,
}

#[derive(Properties, PartialEq)]
pub struct SlidePuzzleProps {
    pub width: usize,
    pub height: usize,
    pub background_url: String,
}

impl Component for SlidePuzzle {
    type Message = SlidePuzzleMsg;
    type Properties = SlidePuzzleProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let fields = initialize_fields(props.width * props.height);
        Self {
            fields,
            width: props.width,
            height: props.height,
            background_url: props.background_url.clone(),
            touch_start_coords: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Received message {:?}", msg);
        match msg {
            SlidePuzzleMsg::Swap((a, b)) => match a != b {
                true => {
                    self.fields.swap(a, b);
                    true
                }
                false => false,
            },
            SlidePuzzleMsg::ClickedField(clicked_idx) => {
                handle_field_click(&mut self.fields, self.width, self.height, clicked_idx)
            }
            SlidePuzzleMsg::WidthUpdate(width) => match width != self.width {
                true => {
                    self.width = width;
                    self.fields = initialize_fields(self.width * self.height);
                    true
                }
                false => false,
            },
            SlidePuzzleMsg::HeightUpdate(height) => match height != self.height {
                true => {
                    self.height = height;
                    self.fields = initialize_fields(self.width * self.height);
                    true
                }
                false => false,
            },
            SlidePuzzleMsg::BackgroundUrlUpdate(bg_url) => match bg_url != self.background_url {
                true => {
                    self.background_url = bg_url;
                    true
                }
                false => false,
            },
            SlidePuzzleMsg::CompleteFieldsUpdate(fields) => match fields != self.fields {
                true => {
                    self.fields = fields;
                    true
                }
                false => false,
            },
            SlidePuzzleMsg::TouchStartCoords((x, y)) => match self.touch_start_coords {
                Some(_) => {
                    log::warn!("Overwriting existing touchStart coordinates");
                    self.touch_start_coords = Some((x, y));
                    false
                }
                None => {
                    self.touch_start_coords = Some((x, y));
                    false
                }
            },
            SlidePuzzleMsg::TouchEndCoords((x_end, y_end)) => match self.touch_start_coords {
                Some((x_start, y_start)) => {
                    if let Some(direction) = get_touch_direction(x_start, y_start, x_end, y_end) {
                        let should_rerender =
                            handle_touch_move(&mut self.fields, self.width, self.height, direction);
                        self.touch_start_coords = None;
                        return should_rerender;
                    }
                    false
                }
                None => {
                    log::warn!("TouchEnd received without previous TouchStart");
                    false
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let quick_swap_callback = self.get_quick_swap_callback(ctx);
        let granular_swap_callback = self.get_granular_swap_callback(ctx);
        let solve_callback = self.get_optimal_solve_callback(ctx);
        let d_and_c_solve_callback = self.get_d_and_c_solve_callback(ctx);

        let field_click_callback = ctx.link().callback(SlidePuzzleMsg::ClickedField);

        let width_change_callback = ctx.link().callback(SlidePuzzleMsg::WidthUpdate);
        let height_change_callback = ctx.link().callback(SlidePuzzleMsg::HeightUpdate);
        let bg_url_change_callback = ctx.link().callback(SlidePuzzleMsg::BackgroundUrlUpdate);
        let touch_start_callback = ctx.link().callback(SlidePuzzleMsg::TouchStartCoords);
        let touch_end_callback = ctx.link().callback(SlidePuzzleMsg::TouchEndCoords);

        html! {
            <>
                <PuzzleBoard
                    fields={self.fields.clone()}
                    on_click={field_click_callback}
                    width={self.width}
                    height={self.height}
                    field_size={4}
                    field_unit={"rem"}
                    background_url={self.background_url.clone()}
                    on_touch_start={touch_start_callback}
                    on_touch_end={touch_end_callback}
                />

                <button onclick={quick_swap_callback}>{"Shuffle Quick"}</button>
                <button onclick={granular_swap_callback}>{"Shuffle Granular"}</button>
                <button onclick={solve_callback}>{"Solve optimally"}</button>
                <button onclick={d_and_c_solve_callback}>{"Solve D and C"}</button>

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
                    <div>{format!("Fields: {:?}", &self.fields)}</div>
                </Expander>
            </>
        }
    }
}

impl SlidePuzzle {
    fn get_quick_swap_callback(&self, ctx: &Context<SlidePuzzle>) -> Callback<MouseEvent> {
        // Create a callback to send a fields message that can be passed into
        // closures
        let swap_callback = ctx.link().callback(SlidePuzzleMsg::CompleteFieldsUpdate);

        // Locally-bind values of self that we want to pass into the closure
        let fields = self.fields.clone();
        let empty_field_idx = get_empty_field_idx(&self.fields);
        let width = self.width;
        let height = self.height;

        Callback::from(move |_| {
            let mut fields = fields.clone();
            // Calculate a shuffle sequence only when the button is clicked, not
            // on every re-render
            let shuffle_sequence = get_shuffle_sequence(width, height, empty_field_idx, 20);
            log::info!("Shuffle sequence: {:?}", &shuffle_sequence);

            for swap in shuffle_sequence {
                fields.swap(swap.0, swap.1);
            }

            swap_callback.emit(fields);
        })
    }

    fn get_granular_swap_callback(&self, ctx: &Context<SlidePuzzle>) -> Callback<MouseEvent> {
        // Create a callback to send a swap message that can be passed into
        // closures
        let swap_callback = ctx.link().callback(move |swap_pair: (usize, usize)| {
            SlidePuzzleMsg::Swap((swap_pair.0, swap_pair.1))
        });

        // Locally-bind values of self that we want to pass into the closure
        let empty_field_idx = get_empty_field_idx(&self.fields);
        let width = self.width;
        let height = self.height;

        Callback::from(move |_| {
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
        })
    }

    fn get_optimal_solve_callback(&self, ctx: &Context<SlidePuzzle>) -> Callback<MouseEvent> {
        // Create a callback to send a swap message that can be passed into
        // closures
        let swap_callback = ctx.link().callback(move |swap_pair: (usize, usize)| {
            SlidePuzzleMsg::Swap((swap_pair.0, swap_pair.1))
        });

        // Locally-bind values of self that we want to pass into the closure
        let fields = self.fields.clone();
        let width = self.width;
        let height = self.height;

        Callback::from(move |_| {
            let fields = fields.clone();
            let swap_callback = swap_callback.clone();

            // Calculate the solving swap sequence only when the button is
            // clicked, not on every re-render
            let solve_sequence = find_swap_order(&fields, width, height);
            log::info!("Solve sequence: {:?}", &solve_sequence);

            for (i, swap) in solve_sequence.into_iter().enumerate() {
                let swap_callback = swap_callback.clone();
                let timeout = gloo_timers::callback::Timeout::new((i * 500) as u32, move || {
                    swap_callback.emit((swap.0, swap.1));
                });
                timeout.forget();
            }
        })
    }

    fn get_d_and_c_solve_callback(&self, ctx: &Context<SlidePuzzle>) -> Callback<MouseEvent> {
        // Create a callback to send a swap message that can be passed into
        // closures
        let swap_callback = ctx.link().callback(move |swap_pair: (usize, usize)| {
            SlidePuzzleMsg::Swap((swap_pair.0, swap_pair.1))
        });

        // Locally-bind values of self that we want to pass into the closure
        let fields = self.fields.clone();
        let width = self.width;
        let height = self.height;

        Callback::from(move |_| {
            let fields = fields.clone();
            let swap_callback = swap_callback.clone();

            // Calculate the solving swap sequence only when the button is
            // clicked, not on every re-render
            let mut solver = DacPuzzleSolver::new(&fields, width as i32, height as i32);
            let solve_sequence = solver.solve_puzzle();
            log::info!("Solve sequence: {:?}", &solve_sequence);

            for (i, swap) in solve_sequence.into_iter().enumerate() {
                let swap_callback = swap_callback.clone();
                let timeout = gloo_timers::callback::Timeout::new((i * 500) as u32, move || {
                    swap_callback.emit((swap.0, swap.1));
                });
                timeout.forget();
            }
        })
    }
}
