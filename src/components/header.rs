use gpui::{IntoElement, ParentElement, Styled, div, rgb, white};
use std::path::PathBuf;

pub struct Header {
    current_path: PathBuf,
}

impl Header {
    pub fn new(current_path: PathBuf) -> Self {
        Self { current_path }
    }

    pub fn render(&self) -> impl IntoElement {
        div().p_4().border_b_1().border_color(rgb(0x333333)).child(
            div()
                .text_color(white())
                .child(format!("Current: {}", self.current_path.display())),
        )
    }
}
