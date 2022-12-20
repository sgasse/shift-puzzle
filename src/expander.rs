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
        <div style="display: flex; flex-direction: row;">
            <div style="display: flex; flex-direction: column;">
                <div onclick={toggle_callback}>
                    if *expanded {
                        {"▼"}
                    } else {
                        {"▶"}
                    }
                </div>
            </div>

            <div style="display: flex; flex-direction: column;">
                <div>
                    {props.title.clone()}
                </div>
                if *expanded {
                    <div>
                        {props.children.clone()}
                    </div>
                }
            </div>
        </div>
    }
}
