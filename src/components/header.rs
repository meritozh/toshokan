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
    pub fn new(current_path: Option<PathBuf>, cx: &mut Context<Self>) -> Self {
        let path =
            current_path.unwrap_or(std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")));
        Self {
            current_path: path.clone(),
            path_input: SharedString::from(path.to_string_lossy().to_string()),
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .on_key_down(
                        cx.listener(|this, event: &gpui::KeyDownEvent, _window, cx| {
                            if this.is_editing {
                                match event.keystroke.key.as_str() {
                                    "enter" => {
                                        let new_path = PathBuf::from(this.path_input.to_string());
                                        if new_path.exists() && new_path.is_dir() {
                                            this.current_path = new_path;
                                            // this.entries = Self::read_directory(&this.current_path);
                                            // this.header.set_path(this.current_path.clone());
                                            // this.header.is_editing = false;
                                            // this.selected_item = None;
                                            // this.file_content = None;
                                            cx.notify();
                                        }
                                    }
                                    "escape" => {
                                        this.is_editing = false;
                                        this.set_path(this.current_path.clone());
                                        cx.notify();
                                    }
                                    "backspace" => {
                                        let mut text = this.path_input.to_string();
                                        text.pop();
                                        this.path_input = SharedString::from(text);
                                        cx.notify();
                                    }
                                    key => {
                                        if key.len() == 1 {
                                            let mut text = this.path_input.to_string();
                                            text.push_str(key);
                                            this.path_input = SharedString::from(text);
                                            cx.notify();
                                        }
                                    }
                                }
                            }
                        }),
                    )
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
