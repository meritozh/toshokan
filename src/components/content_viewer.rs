use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, rgb, white,
};

#[derive(IntoElement)]
pub struct ContentViewer {
    content: Option<SharedString>,
}

impl ContentViewer {
    pub fn new(content: Option<String>) -> Self {
        Self {
            content: content.map(SharedString::from),
        }
    }
}

impl RenderOnce for ContentViewer {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .p_4()
            .child(if let Some(content) = &self.content {
                div().size_full().child(
                    div()
                        .text_color(white())
                        .font_family("Consolas, Monaco, monospace")
                        .text_sm()
                        .whitespace_normal()
                        .child(content.clone()),
                )
            } else {
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0x9ca3af))
                    .child("Select a file to view its contents")
            })
    }
}
