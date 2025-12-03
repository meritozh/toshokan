use gpui::{
    Context, CursorStyle, EventEmitter, FocusHandle, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, SharedString, Styled, Window, div, prelude::*, px,
};
use gpui_component::ActiveTheme;
use std::path::PathBuf;

#[derive(Clone)]
pub enum HeaderEvent {
    NavigateTo(PathBuf),
}

pub struct Header {
    pub current_path: PathBuf,
    pub path_input: SharedString,
    pub focus_handle: FocusHandle,
    pub is_editing: bool,
}

impl EventEmitter<HeaderEvent> for Header {}

impl Header {
    pub fn view(
        _window: &mut Window,
        cx: &mut Context<Self>,
        current_path: Option<PathBuf>,
    ) -> Self {
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

    pub fn go_back(&mut self, cx: &mut Context<Self>) -> Option<PathBuf> {
        if let Some(parent) = self.current_path.parent() {
            let parent_path = parent.to_path_buf();
            self.set_path(parent_path.clone());
            cx.emit(HeaderEvent::NavigateTo(parent_path.clone()));
            cx.notify();
            Some(parent_path)
        } else {
            None
        }
    }
}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let is_editing = self.is_editing;
        let path_input = self.path_input.clone();
        let can_go_back = self.current_path.parent().is_some();

        div()
            .flex()
            .items_center()
            .px_4()
            .py_3()
            .min_h(px(48.0))
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .mr_3()
                    .px_2()
                    .py_1()
                    .rounded(cx.theme().radius)
                    .child("← Back")
                    .when(can_go_back, |s| {
                            s.on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.go_back(cx);
                            }))
                        }),
            )
            .child(div().mr_2().child("Path:"))
            .child(
                div()
                    .flex_1()
                    .p_2()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
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
                                            this.set_path(new_path.clone());
                                            this.is_editing = false;
                                            cx.emit(HeaderEvent::NavigateTo(new_path));
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
