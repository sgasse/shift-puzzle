use yew::prelude::*;

#[derive(Properties, Debug, PartialEq)]
pub struct ExpanderProps {
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
        <div>
            if *expanded {
                <button onclick={toggle_callback}>{"▼"}</button>
                <div>
                    {props.children.clone()}
                </div>
            } else {
                <button onclick={toggle_callback}>{"▶"}</button>
            }
        </div>
    }
}
