use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct SectionIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component]
pub fn SectionIcon(props: &SectionIconProps) -> Html {
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
                d="M2 1.5C2 1.77614 1.77614 2 1.5 2C1.22386 2 1 1.77614 1 1.5C1 1.22386 1.22386 1 1.5 1C1.77614 1 2 1.22386 2 1.5ZM2 5L2 10H13V5H2ZM2 4C1.44772 4 1 4.44772 1 5V10C1 10.5523 1.44772 11 2 11H13C13.5523 11 14 10.5523 14 10V5C14 4.44772 13.5523 4 13 4H2ZM1.5 14C1.77614 14 2 13.7761 2 13.5C2 13.2239 1.77614 13 1.5 13C1.22386 13 1 13.2239 1 13.5C1 13.7761 1.22386 14 1.5 14ZM4 1.5C4 1.77614 3.77614 2 3.5 2C3.22386 2 3 1.77614 3 1.5C3 1.22386 3.22386 1 3.5 1C3.77614 1 4 1.22386 4 1.5ZM3.5 14C3.77614 14 4 13.7761 4 13.5C4 13.2239 3.77614 13 3.5 13C3.22386 13 3 13.2239 3 13.5C3 13.7761 3.22386 14 3.5 14ZM6 1.5C6 1.77614 5.77614 2 5.5 2C5.22386 2 5 1.77614 5 1.5C5 1.22386 5.22386 1 5.5 1C5.77614 1 6 1.22386 6 1.5ZM5.5 14C5.77614 14 6 13.7761 6 13.5C6 13.2239 5.77614 13 5.5 13C5.22386 13 5 13.2239 5 13.5C5 13.7761 5.22386 14 5.5 14ZM8 1.5C8 1.77614 7.77614 2 7.5 2C7.22386 2 7 1.77614 7 1.5C7 1.22386 7.22386 1 7.5 1C7.77614 1 8 1.22386 8 1.5ZM7.5 14C7.77614 14 8 13.7761 8 13.5C8 13.2239 7.77614 13 7.5 13C7.22386 13 7 13.2239 7 13.5C7 13.7761 7.22386 14 7.5 14ZM10 1.5C10 1.77614 9.77614 2 9.5 2C9.22386 2 9 1.77614 9 1.5C9 1.22386 9.22386 1 9.5 1C9.77614 1 10 1.22386 10 1.5ZM9.5 14C9.77614 14 10 13.7761 10 13.5C10 13.2239 9.77614 13 9.5 13C9.22386 13 9 13.2239 9 13.5C9 13.7761 9.22386 14 9.5 14ZM12 1.5C12 1.77614 11.7761 2 11.5 2C11.2239 2 11 1.77614 11 1.5C11 1.22386 11.2239 1 11.5 1C11.7761 1 12 1.22386 12 1.5ZM11.5 14C11.7761 14 12 13.7761 12 13.5C12 13.2239 11.7761 13 11.5 13C11.2239 13 11 13.2239 11 13.5C11 13.7761 11.2239 14 11.5 14ZM14 1.5C14 1.77614 13.7761 2 13.5 2C13.2239 2 13 1.77614 13 1.5C13 1.22386 13.2239 1 13.5 1C13.7761 1 14 1.22386 14 1.5ZM13.5 14C13.7761 14 14 13.7761 14 13.5C14 13.2239 13.7761 13 13.5 13C13.2239 13 13 13.2239 13 13.5C13 13.7761 13.2239 14 13.5 14Z"
                fill={& props.color}
            />
        </svg>
    }
}
