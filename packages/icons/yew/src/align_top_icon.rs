use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct AlignTopIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub width: AttrValue,
    #[prop_or(AttrValue::from("15"))]
    pub height: AttrValue,
}
#[function_component]
pub fn AlignTopIcon(props: &AlignTopIconProps) -> Html {
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
                d="M1.5 0C1.22386 0 1 0.223858 1 0.5C1 0.776142 1.22386 1 1.5 1H6V12C6 12.5523 6.44772 13 7 13H8C8.55228 13 9 12.5523 9 12V1H13.5C13.7761 1 14 0.776142 14 0.5C14 0.223858 13.7761 0 13.5 0H9H6H1.5Z"
                fill={& props.color}
            />
        </svg>
    }
}
