use gpui::{Context, IntoElement, SharedString, Window, div, prelude::*};
use gpui_component::{
    ActiveTheme, StyledExt,
    button::{Button, ButtonRounded},
};

struct ToolBar {
    left_tools: Vec<SharedString>,
    right_tools: Vec<SharedString>,
}

impl ToolBar {
    fn new(tools: &[SharedString]) -> Self {
        Self {
            left_tools: tools.into(),
            right_tools: Vec::new(),
        }
    }
}

impl Render for ToolBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().h_flex().children([
            div().h_flex().children(
                self.left_tools
                    .iter()
                    .map(|name| ToolItem { name: name.clone() }),
            ),
            div().flex().flex_grow(),
            div().h_flex().children(
                self.right_tools
                    .iter()
                    .map(|name| ToolItem { name: name.clone() }),
            ),
        ])
    }
}

#[derive(IntoElement)]
struct ToolItem {
    name: SharedString,
}

impl RenderOnce for ToolItem {
    fn render(self, _window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        Button::new("tool-item")
            .label(self.name)
            .rounded(ButtonRounded::Small)
            .text_color(cx.theme().info_foreground)
    }
}
