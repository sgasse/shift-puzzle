use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct ExpanderProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Expander(props: &ExpanderProps) -> Html {
    let expanded = use_state(|| false);

    let toggle_callback = {
        let expanded = expanded.clone();
        Callback::from(move |_| expanded.set(!(*expanded)))
    };

    html! {
        <div class="expander-wrapper">
            <div class="column-flex">
                <div class="clickable" onclick={toggle_callback.clone()}>
                    if *expanded {
                        {"▼"}
                    } else {
                        {"▶"}
                    }
                </div>
            </div>

            <div class="column-flex">
                <div class="clickable" onclick={toggle_callback}>
                    {props.title.clone()}
                </div>
                if *expanded {
                    <div class="expander-context">
                        {props.children.clone()}
                    </div>
                }
            </div>
        </div>
    }
}
