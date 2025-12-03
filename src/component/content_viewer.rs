use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, rgb, white,
};
use gpui_component::ActiveTheme;

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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(rgb(0x1a1a1a))
            .child(if let Some(file_name) = &self.file_name {
                div()
                    .p_3()
                    .bg(rgb(0x2d2d2d))
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .child(div().text_color(white()).child(format!("📄 {}", file_name)))
            } else {
                div()
                    .p_3()
                    .bg(rgb(0x2d2d2d))
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .child(div().text_color(rgb(0x9ca3af)).child("No file selected"))
            })
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child(if let Some(content) = &self.content {
                        div()
                            .text_color(rgb(0xd4d4d4))
                            .font_family("Consolas, Monaco, 'Courier New', monospace")
                            .text_sm()
                            .whitespace_normal()
                            .child(content.clone())
                    } else {
                        div()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0x9ca3af))
                            .child("Select a file to view its contents")
                    }),
            )
    }
}
