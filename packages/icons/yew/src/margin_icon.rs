use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct MarginIconProps {
    #[prop_or(15)]
    pub width: usize,
    #[prop_or(15)]
    pub height: usize,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
}
#[function_component]
pub fn MarginIcon(props: &MarginIconProps) -> Html {
    html! {
        <svg
            ref={props.node_ref.clone()}
            class={props.class.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1.49988 2.00012C1.77602 2.00012 1.99988 1.77626 1.99988 1.50012C1.99988 1.22398 1.77602 1.00012 1.49988 1.00012C1.22374 1.00012 0.999878 1.22398 0.999878 1.50012C0.999878 1.77626 1.22374 2.00012 1.49988 2.00012ZM4.49988 2.00012C4.77602 2.00012 4.99988 1.77626 4.99988 1.50012C4.99988 1.22398 4.77602 1.00012 4.49988 1.00012C4.22374 1.00012 3.99988 1.22398 3.99988 1.50012C3.99988 1.77626 4.22374 2.00012 4.49988 2.00012ZM7.99988 1.50012C7.99988 1.77626 7.77602 2.00012 7.49988 2.00012C7.22374 2.00012 6.99988 1.77626 6.99988 1.50012C6.99988 1.22398 7.22374 1.00012 7.49988 1.00012C7.77602 1.00012 7.99988 1.22398 7.99988 1.50012ZM10.4999 2.00012C10.776 2.00012 10.9999 1.77626 10.9999 1.50012C10.9999 1.22398 10.776 1.00012 10.4999 1.00012C10.2237 1.00012 9.99988 1.22398 9.99988 1.50012C9.99988 1.77626 10.2237 2.00012 10.4999 2.00012ZM13.9999 1.50012C13.9999 1.77626 13.776 2.00012 13.4999 2.00012C13.2237 2.00012 12.9999 1.77626 12.9999 1.50012C12.9999 1.22398 13.2237 1.00012 13.4999 1.00012C13.776 1.00012 13.9999 1.22398 13.9999 1.50012ZM1.49988 14.0001C1.77602 14.0001 1.99988 13.7763 1.99988 13.5001C1.99988 13.224 1.77602 13.0001 1.49988 13.0001C1.22374 13.0001 0.999878 13.224 0.999878 13.5001C0.999878 13.7763 1.22374 14.0001 1.49988 14.0001ZM1.99988 10.5001C1.99988 10.7763 1.77602 11.0001 1.49988 11.0001C1.22374 11.0001 0.999878 10.7763 0.999878 10.5001C0.999878 10.224 1.22374 10.0001 1.49988 10.0001C1.77602 10.0001 1.99988 10.224 1.99988 10.5001ZM1.49988 8.00012C1.77602 8.00012 1.99988 7.77626 1.99988 7.50012C1.99988 7.22398 1.77602 7.00012 1.49988 7.00012C1.22374 7.00012 0.999878 7.22398 0.999878 7.50012C0.999878 7.77626 1.22374 8.00012 1.49988 8.00012ZM1.99988 4.50012C1.99988 4.77626 1.77602 5.00012 1.49988 5.00012C1.22374 5.00012 0.999878 4.77626 0.999878 4.50012C0.999878 4.22398 1.22374 4.00012 1.49988 4.00012C1.77602 4.00012 1.99988 4.22398 1.99988 4.50012ZM13.4999 11.0001C13.776 11.0001 13.9999 10.7763 13.9999 10.5001C13.9999 10.224 13.776 10.0001 13.4999 10.0001C13.2237 10.0001 12.9999 10.224 12.9999 10.5001C12.9999 10.7763 13.2237 11.0001 13.4999 11.0001ZM13.9999 7.50012C13.9999 7.77626 13.776 8.00012 13.4999 8.00012C13.2237 8.00012 12.9999 7.77626 12.9999 7.50012C12.9999 7.22398 13.2237 7.00012 13.4999 7.00012C13.776 7.00012 13.9999 7.22398 13.9999 7.50012ZM13.4999 5.00012C13.776 5.00012 13.9999 4.77626 13.9999 4.50012C13.9999 4.22398 13.776 4.00012 13.4999 4.00012C13.2237 4.00012 12.9999 4.22398 12.9999 4.50012C12.9999 4.77626 13.2237 5.00012 13.4999 5.00012ZM4.99988 13.5001C4.99988 13.7763 4.77602 14.0001 4.49988 14.0001C4.22374 14.0001 3.99988 13.7763 3.99988 13.5001C3.99988 13.224 4.22374 13.0001 4.49988 13.0001C4.77602 13.0001 4.99988 13.224 4.99988 13.5001ZM7.49988 14.0001C7.77602 14.0001 7.99988 13.7763 7.99988 13.5001C7.99988 13.224 7.77602 13.0001 7.49988 13.0001C7.22374 13.0001 6.99988 13.224 6.99988 13.5001C6.99988 13.7763 7.22374 14.0001 7.49988 14.0001ZM10.9999 13.5001C10.9999 13.7763 10.776 14.0001 10.4999 14.0001C10.2237 14.0001 9.99988 13.7763 9.99988 13.5001C9.99988 13.224 10.2237 13.0001 10.4999 13.0001C10.776 13.0001 10.9999 13.224 10.9999 13.5001ZM13.4999 14.0001C13.776 14.0001 13.9999 13.7763 13.9999 13.5001C13.9999 13.224 13.776 13.0001 13.4999 13.0001C13.2237 13.0001 12.9999 13.224 12.9999 13.5001C12.9999 13.7763 13.2237 14.0001 13.4999 14.0001ZM3.99988 5.00012C3.99988 4.44784 4.44759 4.00012 4.99988 4.00012H9.99988C10.5522 4.00012 10.9999 4.44784 10.9999 5.00012V10.0001C10.9999 10.5524 10.5522 11.0001 9.99988 11.0001H4.99988C4.44759 11.0001 3.99988 10.5524 3.99988 10.0001V5.00012ZM4.99988 5.00012H9.99988V10.0001H4.99988V5.00012Z"
                fill={& props.color}
            />
        </svg>
    }
}
