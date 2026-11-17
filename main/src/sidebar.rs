use gpui_kit::{ParentElement, Render, Styled, component::button::{Button, ButtonVariants}, div, relative};

pub struct Sidebar;

impl Render for Sidebar {
    fn render(&mut self, window: &mut gpui_kit::Window, cx: &mut gpui_kit::prelude::Context<Self>) -> impl gpui_kit::prelude::IntoElement {
        div()
            .flex()
            .flex_col()
            .w(relative(0.1))
            .h(relative(1.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        Button::new("home")
                            .primary()
                            .label("Home")
                    )
            )
    }
}