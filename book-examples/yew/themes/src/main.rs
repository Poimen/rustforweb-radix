mod app;
#[cfg(feature = "avatar")]
mod avatar;
#[cfg(feature = "blockquote")]
mod blockquote;
#[cfg(feature = "box")]
mod r#box;
#[cfg(feature = "button")]
mod button;
#[cfg(feature = "code")]
mod code;
#[cfg(feature = "container")]
mod container;
#[cfg(any(
    feature = "box",
    feature = "container",
    feature = "flex",
    feature = "grid",
    feature = "section"
))]
mod decorative_box;
#[cfg(feature = "em")]
mod em;
#[cfg(feature = "flex")]
mod flex;
#[cfg(feature = "grid")]
mod grid;
#[cfg(feature = "heading")]
mod heading;
#[cfg(feature = "icon-button")]
mod icon_button;
#[cfg(feature = "kbd")]
mod kbd;
#[cfg(feature = "link")]
mod link;
#[cfg(feature = "quote")]
mod quote;
#[cfg(feature = "section")]
mod section;
#[cfg(feature = "select")]
mod select;
#[cfg(feature = "strong")]
mod strong;
#[cfg(feature = "switch")]
mod switch;
#[cfg(feature = "text")]
mod text;
#[cfg(feature = "text-field")]
mod text_field;

use crate::app::App;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    yew::Renderer::<App>::new().render();
}
