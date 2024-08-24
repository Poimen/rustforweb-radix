use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct AlignRightIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component]
pub fn AlignRightIcon(props: &AlignRightIconProps) -> Html {
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
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M14.4999 1C14.2237 1 13.9999 1.22386 13.9999 1.5L13.9999 6L2.99988 6C2.44759 6 1.99988 6.44772 1.99988 7L1.99988 8C1.99988 8.55228 2.44759 9 2.99988 9L13.9999 9L13.9999 13.5C13.9999 13.7761 14.2237 14 14.4999 14C14.776 14 14.9999 13.7761 14.9999 13.5L14.9999 9L14.9999 6L14.9999 1.5C14.9999 1.22386 14.776 1 14.4999 1Z"
                fill={& props.color}
            />
        </svg>
    }
}
