use gpui::{Context, IntoElement, ParentElement, Render, Styled, div, red};

pub struct Root {}

impl Render for Root {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("hello, world").text_color(red())
    }
}
