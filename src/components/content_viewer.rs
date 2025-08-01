use gpui::{
    AppContext, Context, IntoElement, ParentElement, Render, RenderOnce, SharedString, Styled,
    Window, div, rgb, white,
};

#[derive(IntoElement)]
pub struct ContentViewer {
    file_name: Option<SharedString>,
    content: Option<SharedString>,
}

impl ContentViewer {
    pub fn new(file_name: Option<SharedString>, content: Option<String>) -> Self {
        Self {
            file_name,
            content: content.map(SharedString::from),
        }
    }
}

impl RenderOnce for ContentViewer {
    fn render(self, _window: &mut Window, _cx: &mut AppContext<Self>) -> impl IntoElement {
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
