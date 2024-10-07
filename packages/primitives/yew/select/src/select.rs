use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::{Display, Formatter},
    rc::Rc,
};

use radix_number::clamp;
use radix_yew_collection::{
    use_collection, CollectionItemSlot, CollectionProvider, CollectionSlot,
};
use radix_yew_direction::{use_direction, Direction};
use radix_yew_focus_guards::use_focus_guards;
use radix_yew_id::use_id;
use radix_yew_popper::{Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent};
use radix_yew_primitive::{compose_callbacks, Primitive};
use radix_yew_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use web_sys::{wasm_bindgen::JsCast, window};
use yew::{prelude::*, virtual_dom::VNode};
use yew_attrs::{attrs, Attrs};

const OPEN_KEYS: [&str; 4] = [" ", "Enter", "ArrowUp", "ArrowDown"];
const _SELECTION_KEYS: [&str; 2] = [" ", "Enter"];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Position {
    ItemAligned,
    Popper,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::ItemAligned => "item-aligned",
                Position::Popper => "popper",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
struct ItemData {
    value: String,
    disabled: bool,
    text_value: String,
}

#[derive(Clone, PartialEq)]
struct SelectContextValue {
    trigger_ref: NodeRef,
    value_node_ref: NodeRef,
    // TODO: value_node_has_children?
    content_id: String,
    value: Option<String>,
    on_value_change: Callback<String>,
    open: bool,
    required: Option<bool>,
    on_open_change: Callback<bool>,
    dir: Direction,
    trigger_pointer_down_pos_ref: Rc<RefCell<Option<(i32, i32)>>>,
    disabled: Option<bool>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct NativeOption {
    key: String,
    value: String,
    disabled: bool,
    text_content: String,
}

#[derive(Clone, PartialEq)]
struct SelectNativeOptionsContextValue {
    on_native_option_add: Callback<NativeOption>,
    on_native_option_remove: Callback<NativeOption>,
}

#[derive(PartialEq, Properties)]
pub struct SelectProps {
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub default_value: Option<String>,
    #[prop_or_default]
    pub on_value_change: Callback<String>,
    #[prop_or_default]
    pub open: Option<bool>,
    #[prop_or_default]
    pub default_open: Option<bool>,
    #[prop_or_default]
    pub on_open_change: Callback<bool>,
    #[prop_or_default]
    pub dir: Option<Direction>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub autocomplete: Option<String>,
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[prop_or_default]
    pub required: Option<bool>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Select(props: &SelectProps) -> Html {
    let trigger_ref = use_node_ref();
    let value_node_ref = use_node_ref();
    let direction = use_direction(props.dir);

    let on_open_change = use_callback(
        props.on_open_change.clone(),
        |value: Option<bool>, on_open_change| {
            if let Some(value) = value {
                on_open_change.emit(value);
            }
        },
    );
    let (open, set_open) = use_controllable_state(UseControllableStateParams {
        prop: props.open,
        on_change: Some(on_open_change),
        default_prop: props.default_open,
    });
    let open = open.unwrap_or(false);
    let on_open_change = use_callback(set_open, |open: bool, set_open| {
        set_open.emit(Some(open));
    });

    let on_value_change = use_callback(
        props.on_value_change.clone(),
        |value: Option<String>, on_value_change| {
            if let Some(value) = value {
                on_value_change.emit(value);
            }
        },
    );
    let (value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: props.value.clone(),
        on_change: Some(on_value_change),
        default_prop: props.default_value.clone(),
    });
    let on_value_change = use_callback(set_value, |value: String, set_value| {
        set_value.emit(Some(value));
    });

    let trigger_pointer_down_pos_ref = use_mut_ref(|| None);
    let native_options_set = use_state_eq(HashSet::<NativeOption>::new);

    let is_form_control = use_state_eq(|| false);
    use_effect_with(trigger_ref.clone(), {
        let is_form_control = is_form_control.clone();

        move |trigger_ref| {
            is_form_control.set(
                trigger_ref
                    .cast::<web_sys::Element>()
                    .map(|button| button.closest("form").ok().flatten().is_some())
                    .unwrap_or(true),
            );
        }
    });

    let content_id = use_id(None);
    let context_value = use_memo(
        (
            props.disabled,
            props.required,
            direction,
            open,
            on_open_change,
            value,
            on_value_change,
            trigger_pointer_down_pos_ref,
        ),
        |(
            disabled,
            required,
            direction,
            open,
            on_open_change,
            value,
            on_value_change,
            trigger_pointer_down_pos_ref,
        )| {
            SelectContextValue {
                trigger_ref,
                value_node_ref,
                content_id,
                value: value.clone(),
                on_value_change: on_value_change.clone(),
                open: *open,
                required: *required,
                on_open_change: on_open_change.clone(),
                dir: *direction,
                trigger_pointer_down_pos_ref: trigger_pointer_down_pos_ref.clone(),
                disabled: *disabled,
            }
        },
    );

    let native_options_context_value = use_memo((), move |_| SelectNativeOptionsContextValue {
        on_native_option_add: Callback::from({
            let native_options_set = native_options_set.clone();

            move |option| {
                let mut set = (*native_options_set).clone();
                set.insert(option);
                native_options_set.set(set);
            }
        }),
        on_native_option_remove: Callback::from({
            let native_options_set = native_options_set.clone();

            move |option| {
                let mut set = (*native_options_set).clone();
                set.remove(&option);
                native_options_set.set(set);
            }
        }),
    });

    html! {
        <Popper>
            <ContextProvider<SelectContextValue> context={(*context_value).clone()}>
                <CollectionProvider<ItemData>>
                    <ContextProvider<SelectNativeOptionsContextValue> context={(*native_options_context_value).clone()}>
                        {props.children.clone()}
                    </ContextProvider<SelectNativeOptionsContextValue>>
                </CollectionProvider<ItemData>>

                if *is_form_control {
                    // TODO: BubbleSelect
                }
            </ContextProvider<SelectContextValue>>
        </Popper>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectTriggerProps {
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub on_pointer_down: Callback<PointerEvent>,
    #[prop_or_default]
    pub on_key_down: Callback<KeyboardEvent>,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectTrigger(props: &SelectTriggerProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let is_disabled = context.disabled.unwrap_or(props.disabled);
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), context.trigger_ref.clone()]);
    let _get_items = use_collection::<ItemData>();
    let pointer_type_ref = use_mut_ref(|| "touch".to_string());

    let handle_open = use_callback(
        (context.clone(), is_disabled),
        |event_page_coords: Option<(i32, i32)>, (context, is_disabled)| {
            if !is_disabled {
                context.on_open_change.emit(true);
                // Reset typeahead when we open.
                // TODO: reset_typeahead();
            }

            if let Some(event_page_coords) = event_page_coords {
                *context.trigger_pointer_down_pos_ref.borrow_mut() = Some(event_page_coords);
            }
        },
    );

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.on_click.clone(),
            props.on_pointer_down.clone(),
            props.on_key_down.clone(),
        ),
        move |(attrs, on_click, on_pointer_down, on_key_down)| {
            attrs
                .clone()
                .merge(attrs! {
                    type="button"
                    role="combobox"
                    aria-controls={context.content_id}
                    aria-expanded={match context.open {
                        true => "true",
                        false => "false"
                    }}
                    aria-required={context.required.map(|required| match required {
                        true => "true",
                        false => "false"
                    })}
                    aria-autocomplete="none"
                    dir={context.dir.to_string()}
                    data-state={match context.open {
                        true => "open",
                        false => "closed"
                    }}
                    disabled={is_disabled}
                    data-disabled={is_disabled.then_some("")}
                    data-placeholder={should_show_placeholder(context.value).then_some("")}
                    // Enable compatibility with native label or custom `Label` "click" for Safari:
                    onclick={compose_callbacks(Some(on_click.clone()), Some(Callback::from({
                        let pointer_type_ref = pointer_type_ref.clone();
                        let handle_open = handle_open.clone();

                        move |event: MouseEvent| {
                            // Whilst browsers generally have no issue focusing the trigger when clicking
                            // on a label, Safari seems to struggle with the fact that there's no `onclick`.
                            // We force `focus` in this case. Note: this doesn't create any other side-effect
                            // because we are preventing default in `onpointerdown` so effectively
                            // this only runs for a label "click".
                            event
                                .current_target()
                                .expect("Event should have current target.")
                                .unchecked_into::<web_sys::HtmlElement>()
                                .focus()
                                .expect("Element should be focused.");

                            // Open on click when using a touch or pen device.
                            if *pointer_type_ref.borrow() != "mouse" {
                                handle_open.emit(Some((event.page_x(), event.page_y())));
                            }
                    }})), None)}
                    onpointerdown={compose_callbacks(Some(on_pointer_down.clone()), Some(Callback::from({
                        let handle_open = handle_open.clone();

                        move |event: PointerEvent| {
                            *pointer_type_ref.borrow_mut() =event.pointer_type();

                            // Prevent implicit pointer capture.
                            // https://www.w3.org/TR/pointerevents3/#implicit-pointer-capture
                            let target = event.target().expect("Event should have target.").unchecked_into::<web_sys::HtmlElement>();
                            if target.has_pointer_capture(event.pointer_id()) {
                                target.release_pointer_capture(event.pointer_id()).expect("Pointer capture should be released.");
                            }

                            // Only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                            // but not when the control key is pressed (avoiding MacOS right click); also not for touch
                            // devices because that would open the menu on scroll. (pen devices behave as touch on iOS).
                            if event.button() == 0 && !event.ctrl_key() && event.pointer_type() == "mouse" {
                                handle_open.emit(Some((event.page_x(), event.page_y())));

                                // Prevent trigger from stealing focus from the active item after opening.
                                event.prevent_default();
                            }
                        }
                    })), None)}
                    onkeydown={compose_callbacks(Some(on_key_down.clone()), Some(Callback::from(move |event: KeyboardEvent| {
                        // TODO: typeahead

                        if OPEN_KEYS.contains(&event.key().as_str()) {
                            handle_open.emit(None);
                            event.prevent_default();
                        }
                    })), None)}
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <PopperAnchor as_child=true>
            <Primitive
                element="button"
                as_child={props.as_child}
                node_ref={composed_refs}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </PopperAnchor>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectValueProps {
    #[prop_or("".to_string())]
    pub placeholder: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectValue(props: &SelectValueProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), context.value_node_ref]);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // We don't want events from the portalled `SelectValue` children to bubble through the item they came from.
                style="pointer-events: none;"
            })
            .expect("Attributes should be merged.")
    });

    // TODO: value node has children?

    html! {
        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={composed_refs}
            attrs={(*attrs).clone()}
        >
            if should_show_placeholder(context.value) {
                {props.placeholder.clone()}
            } else {
                {props.children.clone()}
            }
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectIconProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectIcon(props: &SelectIconProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {match &props.children {
                VNode::VList(list) if list.is_empty() => html!{"▼"},
                children => children.clone(),
            }}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectPortalProps {
    // TODO: container
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectPortal(props: &SelectPortalProps) -> Html {
    html! {
        // TODO: Portal
        {props.children.clone()}
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectContentProps {
    // TODO
    #[prop_or(Position::ItemAligned)]
    pub position: Position,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectContent(props: &SelectContentProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");

    html! {
        if context.open {
            <SelectContentImpl position={props.position}>
                {props.children.clone()}
            </SelectContentImpl>
        } else {
            // TODO: Portal to DocumentFragment
            // <ContextProvider<SelectContentContextValue>>
            //     <CollectionSlot<ItemData>>
            //         <div>{props.children.clone()}</div>
            //     </CollectionSlot<ItemData>>
            // </ContextProvider<SelectContentContextValue>>
        }
    }
}

const CONTENT_MARGIN: f64 = 10.0;

#[derive(Clone, PartialEq)]
struct SelectContentContextValue {
    content_ref: NodeRef,
    viewport_ref: NodeRef,
    item_ref_callback: Callback<(web_sys::HtmlElement, String, bool)>,
    selected_item: Option<web_sys::HtmlElement>,
    // TODO
    item_text_ref_callback: Callback<(web_sys::HtmlElement, String, bool)>,
    selected_item_text: Option<web_sys::HtmlElement>,
    position: Position,
    is_positioned: bool,
    // search_ref: Rc<RefCell<String>>,
}

#[derive(PartialEq, Properties)]
struct SelectContentImplProps {
    // TODO
    #[prop_or(Position::ItemAligned)]
    pub position: Position,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectContentImpl(props: &SelectContentImplProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select contenxt required.");
    let content_ref = use_node_ref();
    let viewport_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let selected_item = use_state_eq(|| None);
    let selected_item_text = use_state_eq(|| None);
    let _get_items = use_collection::<ItemData>();
    let is_positioned = use_state_eq(|| false);
    let first_valid_item_found_ref = use_mut_ref(|| false);

    // TODO

    // Make sure the whole tree has focus guards as our `Select` may be the last element in the DOM (because of the `Portal`).
    use_focus_guards();

    let item_ref_callback = use_callback(context.value.clone(), {
        let first_valid_item_found_ref = first_valid_item_found_ref.clone();
        let selected_item = selected_item.clone();

        move |(node, value, disabled): (web_sys::HtmlElement, String, bool), context_value| {
            let is_first_valid_item = !*first_valid_item_found_ref.borrow() && !disabled;
            let is_selected_item = context_value
                .as_ref()
                .is_some_and(|context_value| *context_value == value);
            if is_selected_item || is_first_valid_item {
                selected_item.set(Some(node));

                if is_first_valid_item {
                    *first_valid_item_found_ref.borrow_mut() = true;
                }
            }
        }
    });
    // TODO: handle_item_leave
    let item_text_ref_callback = use_callback(context.value, {
        let first_valid_item_found_ref = first_valid_item_found_ref.clone();
        let selected_item_text = selected_item_text.clone();

        move |(node, value, disabled): (web_sys::HtmlElement, String, bool), context_value| {
            let is_first_valid_item = !*first_valid_item_found_ref.borrow() && !disabled;
            let is_selected_item = context_value
                .as_ref()
                .is_some_and(|context_value| *context_value == value);
            if is_selected_item || is_first_valid_item {
                selected_item_text.set(Some(node));
            }
        }
    });

    let content_context_value = use_memo(
        (
            props.position,
            selected_item,
            selected_item_text,
            is_positioned,
        ),
        |(position, selected_item, selected_item_text, is_positioned)| SelectContentContextValue {
            content_ref,
            viewport_ref,
            item_ref_callback,
            selected_item: (**selected_item).clone(),
            item_text_ref_callback,
            selected_item_text: (**selected_item_text).clone(),
            position: *position,
            is_positioned: **is_positioned,
        },
    );

    html! {
        <ContextProvider<SelectContentContextValue> context={(*content_context_value).clone()}>
            // TODO: RemoveScrol, FocusScope, DismissableLayer

            if props.position == Position::Popper {
                <SelectPopperPosition
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={props.attrs.clone()}
                >
                    {props.children.clone()}
                </SelectPopperPosition>
            } else {
                <SelectItemAlignedPosition
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={props.attrs.clone()}
                >
                    {props.children.clone()}
                </SelectItemAlignedPosition>
            }
        </ContextProvider<SelectContentContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectItemAlignedPositionProps {
    // TODO
    #[prop_or_default]
    pub on_placed: Callback<()>,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectItemAlignedPosition(props: &SelectItemAlignedPositionProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let content_wrapper_ref = use_node_ref();
    let content_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_ref.clone()]);
    let get_items = use_collection::<ItemData>();
    // TODO

    let position = use_callback(
        (
            get_items,
            context.trigger_ref,
            context.value_node_ref,
            content_wrapper_ref.clone(),
            content_ref,
            content_context.viewport_ref,
            content_context.selected_item,
            content_context.selected_item_text,
            context.dir,
            props.on_placed.clone(),
        ),
        |_: (),
         (
            get_items,
            trigger_ref,
            value_node_ref,
            content_wrapper_ref,
            content_ref,
            viewport_ref,
            selected_item,
            selected_item_text,
            dir,
            on_placed,
        )| {
            if let Some(trigger) = trigger_ref.cast::<web_sys::Element>() {
                if let Some(value_node) = value_node_ref.cast::<web_sys::Element>() {
                    if let Some(content_wrapper) =
                        content_wrapper_ref.cast::<web_sys::HtmlElement>()
                    {
                        if let Some(content) = content_ref.cast::<web_sys::Element>() {
                            if let Some(viewport) = viewport_ref.cast::<web_sys::HtmlElement>() {
                                if let Some(selected_item) = selected_item {
                                    if let Some(selected_item_text) = selected_item_text {
                                        let window = window().expect("Window should exist.");
                                        let window_inner_width = window
                                            .inner_width()
                                            .expect("Window should have inner width.")
                                            .as_f64()
                                            .expect("Inner width should be a number.");
                                        let window_inner_height = window
                                            .inner_height()
                                            .expect("Window should have inner height.")
                                            .as_f64()
                                            .expect("Inner height should be a number.");

                                        let trigger_rect = trigger.get_bounding_client_rect();

                                        // Horizontal positioning
                                        let content_rect = content.get_bounding_client_rect();
                                        let value_node_rect = value_node.get_bounding_client_rect();
                                        let item_text_rect =
                                            selected_item_text.get_bounding_client_rect();

                                        if *dir != Direction::Rtl {
                                            let item_text_offset =
                                                item_text_rect.left() - content_rect.left();
                                            let left = value_node_rect.left() - item_text_offset;
                                            let left_delta = trigger_rect.left() - left;
                                            let min_content_width =
                                                trigger_rect.width() + left_delta;
                                            let content_width =
                                                min_content_width.max(content_rect.width());
                                            let right_edge = window_inner_width - CONTENT_MARGIN;
                                            let clamped_left = clamp(
                                                left,
                                                [
                                                    CONTENT_MARGIN,
                                                    // Prevents the content from going off the starting edge of the
                                                    // viewport. It may still go off the ending edge, but this can be
                                                    // controlled by the user since they may want to manage overflow in a
                                                    // specific way.
                                                    // https://github.com/radix-ui/primitives/issues/2049
                                                    CONTENT_MARGIN.max(right_edge - content_width),
                                                ],
                                            );

                                            content_wrapper
                                                .style()
                                                .set_property(
                                                    "min-width",
                                                    &format!("{min_content_width}px"),
                                                )
                                                .expect("Min width should be set.");
                                            content_wrapper
                                                .style()
                                                .set_property("left", &format!("{clamped_left}px"))
                                                .expect("Left should be set.");
                                        } else {
                                            let item_text_offset =
                                                content_rect.right() - item_text_rect.right();
                                            let right = window_inner_width
                                                - value_node_rect.right()
                                                - item_text_offset;
                                            let right_delta =
                                                window_inner_width - trigger_rect.right() - right;
                                            let min_content_width =
                                                trigger_rect.width() + right_delta;
                                            let content_width =
                                                min_content_width.max(content_rect.width());
                                            let left_edge = window_inner_width - CONTENT_MARGIN;
                                            let clamped_right = clamp(
                                                right,
                                                [
                                                    CONTENT_MARGIN,
                                                    CONTENT_MARGIN.max(left_edge - content_width),
                                                ],
                                            );

                                            content_wrapper
                                                .style()
                                                .set_property(
                                                    "min-width",
                                                    &format!("{min_content_width}px"),
                                                )
                                                .expect("Min width should be set.");
                                            content_wrapper
                                                .style()
                                                .set_property("left", &format!("{clamped_right}px"))
                                                .expect("Left should be set.");
                                        }

                                        // Vertical positioning
                                        let items = get_items.emit(());
                                        let available_height =
                                            window_inner_height - CONTENT_MARGIN * 2.0;
                                        let items_height = viewport.scroll_height() as f64;

                                        let content_styles = window
                                            .get_computed_style(&content)
                                            .expect("Element is valid.")
                                            .expect("Element should have computed style.");
                                        let content_border_top_width = content_styles
                                            .get_property_value("border-top-width")
                                            .expect("Compyted style should have border top width.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Border top width should be a number.");
                                        let content_padding_top = content_styles
                                            .get_property_value("padding-top")
                                            .expect("Compyted style should have padding top.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding top should be a number.");
                                        let content_border_bottom_width = content_styles
                                            .get_property_value("border-bottom-width")
                                            .expect(
                                                "Compyted style should have border bottom width.",
                                            )
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Border bottom width should be a number.");
                                        let content_padding_bottom = content_styles
                                            .get_property_value("padding-bottom")
                                            .expect("Compyted style should have padding bottom.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding bottom should be a number.");
                                        let full_content_height = content_border_top_width
                                            + content_padding_top
                                            + items_height
                                            + content_padding_bottom
                                            + content_border_bottom_width;
                                        let min_content_height =
                                            (selected_item.offset_height() as f64 * 5.0)
                                                .min(full_content_height);

                                        let viewport_styles = window
                                            .get_computed_style(&viewport)
                                            .expect("Element is valid.")
                                            .expect("Element should have computed style.");
                                        let viewport_padding_top = viewport_styles
                                            .get_property_value("padding-top")
                                            .expect("Compyted style should have padding top.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding top should be a number.");
                                        let viewport_padding_bottom = viewport_styles
                                            .get_property_value("padding-bottom")
                                            .expect("Compyted style should have padding bottom.")
                                            .trim_end_matches("px")
                                            .parse::<f64>()
                                            .expect("Padding bottom should be a number.");

                                        let top_edge_to_trigger_middle = trigger_rect.top()
                                            + trigger_rect.height() / 2.0
                                            - CONTENT_MARGIN;
                                        let trigger_middle_to_bottom_edge =
                                            available_height - top_edge_to_trigger_middle;

                                        let selected_item_half_height =
                                            selected_item.offset_height() as f64 / 2.0;
                                        let item_offset_middle = selected_item.offset_top() as f64
                                            + selected_item_half_height;
                                        let content_top_to_item_middle = content_border_top_width
                                            - content_padding_top
                                            + item_offset_middle;
                                        let item_middle_to_content_bottom =
                                            full_content_height - content_top_to_item_middle;

                                        let will_align_without_top_overflow =
                                            content_top_to_item_middle
                                                <= top_edge_to_trigger_middle;

                                        if will_align_without_top_overflow {
                                            let is_last_item = !items.is_empty()
                                                && items
                                                    .last()
                                                    .expect("Last item should exist.")
                                                    .r#ref
                                                    .cast::<web_sys::HtmlElement>()
                                                    .is_some_and(|element| {
                                                        &element == selected_item
                                                    });

                                            content_wrapper
                                                .style()
                                                .set_property("bottom", "0px")
                                                .expect("Bottom should be set.");

                                            let viewport_offset_bottom = content.client_height()
                                                as f64
                                                - viewport.offset_top() as f64
                                                - viewport.offset_height() as f64;
                                            let clamped_trigger_middle_to_bottom_edge =
                                                trigger_middle_to_bottom_edge.max(
                                                    selected_item_half_height
                                                        // Viewport might have padding bottom,
                                                        // include it to avoid a scrollable viewport.
                                                        + match is_last_item {
                                                            true => viewport_padding_bottom,
                                                            false => 0.0,
                                                        }
                                                        + viewport_offset_bottom
                                                        + content_border_bottom_width,
                                                );
                                            let height = content_top_to_item_middle
                                                + clamped_trigger_middle_to_bottom_edge;

                                            content_wrapper
                                                .style()
                                                .set_property("height", &format!("{height}px"))
                                                .expect("Height should be set.");
                                        } else {
                                            let is_first_item = !items.is_empty()
                                                && items
                                                    .first()
                                                    .expect("First item should exist.")
                                                    .r#ref
                                                    .cast::<web_sys::HtmlElement>()
                                                    .is_some_and(|element| {
                                                        &element == selected_item
                                                    });

                                            content_wrapper
                                                .style()
                                                .set_property("top", "0px")
                                                .expect("Top should be set.");

                                            let clamped_top_edge_to_trigger_middle =
                                                top_edge_to_trigger_middle.max(
                                                    content_border_top_width
                                                        + viewport.offset_top() as f64
                                                        // Viewport might have padding top,
                                                        // include it to avoid a scrollable viewport.
                                                        + match is_first_item {
                                                            true => viewport_padding_top,
                                                            false => 0.0,
                                                        }
                                                        + selected_item_half_height,
                                                );
                                            let height = clamped_top_edge_to_trigger_middle
                                                + item_middle_to_content_bottom;

                                            content_wrapper
                                                .style()
                                                .set_property("height", &format!("{height}px"))
                                                .expect("Height should be set.");
                                            viewport.set_scroll_top(
                                                (content_top_to_item_middle
                                                    - top_edge_to_trigger_middle
                                                    + viewport.offset_top() as f64)
                                                    as i32,
                                            );
                                        }

                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "margin",
                                                &format!("{CONTENT_MARGIN}px 0px"),
                                            )
                                            .expect("Margin should be set.");
                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "min-height",
                                                &format!("{min_content_height}px"),
                                            )
                                            .expect("Min height should be set.");
                                        content_wrapper
                                            .style()
                                            .set_property(
                                                "max-height",
                                                &format!("{available_height}px"),
                                            )
                                            .expect("Min height should be set.");

                                        on_placed.emit(());

                                        // TODO: request animation frame
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    );

    use_effect(move || position.emit(()));

    // Copy z-index from content to wrapper.
    let content_z_index: UseStateHandle<Option<String>> = use_state_eq(|| None);
    // TODO

    let viewport_context_value = use_memo((), |_| SelectViewportContextValue {
        content_wrapper_ref: content_wrapper_ref.clone(),
    });

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // When we get the height of the content, it includes borders. If we were to set
                // the height without having `box-sizing: border-box` it would be too big.

                // We need to ensure the content doesn't get taller than the wrapper.
                style="box-sizing: border-box; max-height: 100%;"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <ContextProvider<SelectViewportContextValue> context={(*viewport_context_value).clone()}>
            <div
                ref={content_wrapper_ref}
                style={format!("display: flex; flex-direction: column; position: fixed;{}", content_z_index.as_ref().map(|content_z_index| format!("z-index: {content_z_index};")).unwrap_or_default())}
            >
                <Primitive
                    element="span"
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={(*attrs).clone()}
                >
                    {props.children.clone()}
                </Primitive>
            </div>
        </ContextProvider<SelectViewportContextValue>>
    }
}

#[derive(PartialEq, Properties)]
struct SelectPopperPositionProps {
    // TODO
    #[prop_or(Align::Start)]
    pub align: Align,
    #[prop_or(Padding::All(CONTENT_MARGIN))]
    pub collision_padding: Padding,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectPopperPosition(props: &SelectPopperPositionProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO: merge with style attr if present

                // Ensure border-box for Floating UI calculations.
                // Re-namespace exposed content custom properties.
                style="\
                    box-sizing: border-box;\
                    --radix-select-content-transform-origin: var(--radix-popper-transform-origin);\
                    --radix-select-content-available-width: var(--radix-popper-available-width);\
                    --radix-select-content-available-height: var(--radix-popper-available-height);\
                    --radix-select-trigger-width: var(--radix-popper-anchor-width);\
                    --radix-select-trigger-height: var(--radix-popper-anchor-height);\
                "
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <PopperContent
            // TODO: other PopperContent props
            align={props.align}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </PopperContent>
    }
}

#[derive(Clone, PartialEq)]
struct SelectViewportContextValue {
    content_wrapper_ref: NodeRef,
    // TODO
    // should_expand_on_scroll: bool,
    // on_scroll_button_change: Callback<>
}

#[derive(PartialEq, Properties)]
pub struct SelectViewportProps {
    #[prop_or_default]
    pub nonce: Option<String>,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectViewport(props: &SelectViewportProps) -> Html {
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), content_context.viewport_ref]);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                data-radix-select-viewport=""
                role="presentation"
                // TODO: merge with style attr if present
                // We use position: 'relative' here on the `viewport` so that when we call `selected_item.offset_top` in calculations,
                // the offset is relative to the viewport (independent of the ScrollUpButton).
                style="position: relative; flex: 1; overflow: auto;"
                // TODO: onscroll
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <>
            // Hide scrollbars cross-browser and enable momentum scroll for touch devices.
            <style nonce={props.nonce.clone()}>
                {"[data-radix-select-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-select-viewport]::-webkit-scrollbar{display:none;}"}
            </style>

            <CollectionSlot<ItemData>>
                <Primitive
                    element="div"
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={(*attrs).clone()}
                >
                    {props.children.clone()}
                </Primitive>
            </CollectionSlot<ItemData>>
        </>
    }
}

#[derive(Clone, PartialEq)]
struct SelectGroupContextValue {
    id: String,
}

#[derive(PartialEq, Properties)]
pub struct SelectGroupProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectGroup(props: &SelectGroupProps) -> Html {
    let group_id = use_id(None);

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                role="group"
                aria-labelledby={group_id.clone()}
            })
            .expect("Attributes should be merged.")
    });

    let context_value = use_memo(group_id, |group_id| SelectGroupContextValue {
        id: group_id.clone(),
    });

    html! {
        <ContextProvider<SelectGroupContextValue> context={(*context_value).clone()}>
            <Primitive
                element="div"
                as_child={props.as_child}
                node_ref={props.node_ref.clone()}
                attrs={(*attrs).clone()}
            >
                {props.children.clone()}
            </Primitive>
        </ContextProvider<SelectGroupContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectLabelProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectLabel(props: &SelectLabelProps) -> Html {
    let group_context =
        use_context::<SelectGroupContextValue>().expect("Select group context required.");

    let attrs = use_memo(
        (props.attrs.clone(), group_context),
        |(attrs, group_context)| {
            attrs
                .clone()
                .merge(attrs! {
                    id={group_context.id.clone()}
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(Clone, PartialEq)]
struct SelectItemContextValue {
    value: String,
    disabled: bool,
    text_id: String,
    is_selected: bool,
    on_item_text_change: Callback<web_sys::HtmlElement>,
}

#[derive(PartialEq, Properties)]
pub struct SelectItemProps {
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub text_value: String,
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItem(props: &SelectItemProps) -> Html {
    let context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let text_value = use_state_eq(|| props.text_value.clone());
    let is_focused = use_state_eq(|| false);
    let item_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), item_ref.clone()]);
    let is_selected = context.value.is_some_and(|value| value == props.value);
    let text_id = use_id(None);

    let item_ref_callback = use_callback(
        (
            content_context.item_ref_callback,
            props.value.clone(),
            props.disabled,
        ),
        |node: web_sys::HtmlElement, (item_ref_callback, value, disabled)| {
            item_ref_callback.emit((node, value.clone(), *disabled))
        },
    );
    use_effect_with(item_ref, move |item_ref| {
        if let Some(node) = item_ref.cast::<web_sys::HtmlElement>() {
            item_ref_callback.emit(node);
        }
    });

    let item_context_value = use_memo(
        (
            props.value.clone(),
            props.disabled,
            text_id.clone(),
            is_selected,
        ),
        |(value, disabled, text_id, is_selected)| SelectItemContextValue {
            value: (*value).clone(),
            disabled: *disabled,
            text_id: (*text_id).clone(),
            is_selected: *is_selected,
            on_item_text_change: Callback::from(|_| {}),
        },
    );

    let item_data = use_memo(
        (props.value.clone(), props.disabled, text_value),
        |(value, disabled, text_value)| ItemData {
            value: (*value).clone(),
            disabled: *disabled,
            text_value: (**text_value).clone(),
        },
    );

    let attrs = use_memo(
        (
            props.attrs.clone(),
            props.disabled,
            text_id,
            is_focused,
            is_selected,
        ),
        |(attrs, disabled, text_id, is_focused, is_selected)| {
            attrs
                .clone()
                .merge(attrs! {
                    role="option"
                    aria-labelledby={text_id.clone()}
                    data-highlighted={is_focused.then_some("")}
                    // `is_focused` caveat fixes stuttering in VoiceOver.
                    aria-selected={(*is_selected && **is_focused).then_some("true")}
                    data-state={if *is_selected { "checked" } else { "unchecked "}}
                    aria-disabled={disabled.then_some("true")}
                    data-disabled={disabled.then_some("")}
                    tab-index={(!disabled).then_some("-1")}
                    // TODO: events
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        <ContextProvider<SelectItemContextValue> context={(*item_context_value).clone()}>
            <CollectionItemSlot<ItemData> item_data={(*item_data).clone()}>
                <Primitive
                    element="div"
                    as_child={props.as_child}
                    node_ref={composed_refs}
                    attrs={(*attrs).clone()}
                >
                    {props.children.clone()}
                </Primitive>
            </CollectionItemSlot<ItemData>>
        </ContextProvider<SelectItemContextValue>>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemTextProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItemText(props: &SelectItemTextProps) -> Html {
    let _context = use_context::<SelectContextValue>().expect("Select context required.");
    let content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let item_context =
        use_context::<SelectItemContextValue>().expect("Select item context required.");
    let _native_options_context = use_context::<SelectNativeOptionsContextValue>()
        .expect("Select native options context required.");
    let item_text_node_ref = use_node_ref();
    let composed_refs = use_composed_ref(&[props.node_ref.clone(), item_text_node_ref.clone()]);

    let item_text_ref_callback = use_callback(
        (content_context.item_text_ref_callback, item_context.clone()),
        |node: web_sys::HtmlElement, (item_text_ref_callback, item_context)| {
            item_context.on_item_text_change.emit(node.clone());
            item_text_ref_callback.emit((node, item_context.value.clone(), item_context.disabled));
        },
    );
    use_effect_with(item_text_node_ref, move |item_text_node_ref| {
        if let Some(node) = item_text_node_ref.cast::<web_sys::HtmlElement>() {
            item_text_ref_callback.emit(node);
        }
    });

    let attrs = use_memo(
        (props.attrs.clone(), item_context),
        |(attrs, item_context)| {
            attrs
                .clone()
                .merge(attrs! {
                    id={item_context.text_id.clone()}
                    // TODO
                })
                .expect("Attributes should be merged.")
        },
    );

    html! {
        // TODO

        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={composed_refs}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectItemIndicatorProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectItemIndicator(props: &SelectItemIndicatorProps) -> Html {
    // let item_context = use_context::<SelectItemContextValue>().expect("Select item context required.");

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        // TODO

        <Primitive
            element="span"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollUpButtonProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollUpButton(props: &SelectScrollUpButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectScrollDownButtonProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectScrollDownButton(props: &SelectScrollDownButtonProps) -> Html {
    // TODO
    html! {
        <SelectScrollButtonImpl
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </SelectScrollButtonImpl>
    }
}

#[derive(PartialEq, Properties)]
struct SelectScrollButtonImplProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
fn SelectScrollButtonImpl(props: &SelectScrollButtonImplProps) -> Html {
    let _content_context =
        use_context::<SelectContentContextValue>().expect("Select content context required.");
    let _get_items = use_collection::<ItemData>();

    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                // TODO
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectSeparatorProps {
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectSeparator(props: &SelectSeparatorProps) -> Html {
    let attrs = use_memo(props.attrs.clone(), |attrs| {
        attrs
            .clone()
            .merge(attrs! {
                aria-hidden="true"
            })
            .expect("Attributes should be merged.")
    });

    html! {
        <Primitive
            element="div"
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={(*attrs).clone()}
        >
            {props.children.clone()}
        </Primitive>
    }
}

#[derive(PartialEq, Properties)]
pub struct SelectArrowProps {
    // TODO
    #[prop_or(false)]
    pub as_child: bool,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attrs: Attrs,
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn SelectArrow(props: &SelectArrowProps) -> Html {
    // TODO
    html! {
        <PopperArrow
            as_child={props.as_child}
            node_ref={props.node_ref.clone()}
            attrs={props.attrs.clone()}
        >
            {props.children.clone()}
        </PopperArrow>
    }
}
fn should_show_placeholder(value: Option<String>) -> bool {
    value.is_none() || value.is_some_and(|value| value.is_empty())
}
