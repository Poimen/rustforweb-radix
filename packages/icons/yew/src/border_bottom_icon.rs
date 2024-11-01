use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BorderBottomIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn BorderBottomIcon(props: &BorderBottomIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            width={& props.width}
            height={& props.height}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 13.25L14 13.25V14.75L1 14.75V13.25Z"
                fill={& props.color}
            />
            <rect x="7" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="7" y="3" width="1" height="1" rx=".5" fill={& props.color} />
            <rect
                x="13"
                y="3"
                width="1"
                height="1"
                rx=".5"
                fill={& props
        .color}
            />
            <rect
                x="7"
                y="7"
                width="1"
                height="1"
                rx=".5"
                fill={&
        props.color}
            />
            <rect x="7" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="5" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="5" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="3" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="3" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="9" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="9" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="11" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="11" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect
                x="7"
                y="9"
                width="1"
                height="1"
                rx=".5"
                fill={& props
        .color}
            />
            <rect
                x="13"
                y="9"
                width="1"
                height="1"
                rx=".5"
                fill={&
        props.color}
            />
            <rect x="7" y="11" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="13" y="11" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="5" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="3" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="7" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="1" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="9" width="1" height="1" rx=".5" fill={& props.color} />
            <rect x="1" y="11" width="1" height="1" rx=".5" fill={& props.color} />
        </svg>
    }
}
