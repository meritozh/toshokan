use gpui::{
    Context, CursorStyle, FocusHandle, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, SharedString, Styled, Window, div, rgb, white,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Header {
    pub current_path: PathBuf,
    pub path_input: SharedString,
    pub focus_handle: FocusHandle,
    pub is_editing: bool,
}

impl Header {
    pub fn new(current_path: PathBuf, cx: &mut Context<'_, Window>) -> Self {
        Self {
            current_path: current_path.clone(),
            path_input: SharedString::from(current_path.to_string_lossy().to_string()),
            focus_handle: cx.focus_handle(),
            is_editing: false,
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.current_path = path.clone();
        self.path_input = SharedString::from(path.to_string_lossy().to_string());
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let is_editing = self.is_editing;
        let path_input = self.path_input.clone();

        div()
            .flex()
            .items_center()
            .p_4()
            .bg(rgb(0x2d2d2d))
            .border_b_1()
            .border_color(rgb(0x3e3e3e))
            .child(div().text_color(white()).mr_2().child("Path:"))
            .child(
                div()
                    .flex_1()
                    .p_2()
                    .bg(rgb(0x1e1e1e))
                    .border_1()
                    .border_color(if is_editing {
                        rgb(0x0078d4)
                    } else {
                        rgb(0x3e3e3e)
                    })
                    .rounded_md()
                    .text_color(white())
                    .child(path_input)
                    .cursor(if is_editing {
                        CursorStyle::IBeam
                    } else {
                        CursorStyle::Arrow
                    })
                    .track_focus(&focus_handle)
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, window, cx| {
                            this.is_editing = true;
                            window.focus(&focus_handle);
                            cx.notify();
                        }),
                    ),
            )
    }
}
