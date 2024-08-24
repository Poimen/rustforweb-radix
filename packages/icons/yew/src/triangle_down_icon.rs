use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct TriangleDownIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component]
pub fn TriangleDownIcon(props: &TriangleDownIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path d="M4 6H11L7.5 10.5L4 6Z" fill={& props.color} />
        </svg>
    }
}
