use radix_yew_arrow::*;
use yew::prelude::*;

#[function_component]
pub fn Styled() -> Html {
    html! {
        <Arrow width=20.0 height=10.0 style="vertical-align: middle; fill: crimson;" />
    }
}

#[function_component]
pub fn CustomSizes() -> Html {
    html! {
        <>
            <Arrow width=40.0 height=10.0 style="vertical-align: middle; display: inline-block;" />
            <Arrow width=50.0 height=30.0 style="vertical-align: middle; display: inline-block;" />
            <Arrow width=20.0 height=100.0 style="vertical-align: middle; display: inline-block;" />
        </>
    }
}

#[function_component]
pub fn CustomArrow() -> Html {
    html! {
        <Arrow
            as_child={Callback::from(|ArrowChildProps { node_ref, .. }| html! {
                <div
                    ref={node_ref}
                    style="width: 20px; height: 10px; border-bottom-left-radius: 10px; border-bottom-right-radius: 10px; background-color: tomato;"
                />
            })}
        />
    }
}
