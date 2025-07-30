use crate::components::DirEntry;
use gpui::prelude::FluentBuilder;
use gpui::{InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, div, rgb};
use gpui_component::v_flex;
use std::path::PathBuf;

pub struct FileList {
    entries: Vec<DirEntry>,
    selected_path: Option<PathBuf>,
}

impl FileList {
    pub fn new(entries: Vec<DirEntry>, selected_path: Option<PathBuf>) -> Self {
        Self {
            entries,
            selected_path,
        }
    }
}

impl Render for FileList {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<'_, Self>,
    ) -> impl IntoElement {
        v_flex()
            .w_80()
            .h_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(rgb(0x3d3d3d))
            .children(self.entries.iter().map(|entry| {
                let entry_name = entry.name.clone();
                let entry_is_dir = entry.is_dir;
                let is_selected = self.selected_path.as_ref() == Some(&entry.path);
                let on_entry_click = cx.listener(
                    move |_this: &mut FileList, _event: &gpui::MouseDownEvent, _window, _cx| {
                        // Note: This will need to be updated to call the appropriate method on FileList
                        // or delegate to Root somehow, as FileList doesn't have handle_item_click
                    },
                );

                div()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(rgb(0x2a2a2a))
                    .when(is_selected, |s| s.bg(rgb(0x3d3d3d)))
                    .hover(|s| s.bg(rgb(0x2d2d2d)))
                    .on_mouse_down(MouseButton::Left, on_entry_click)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .child(
                                div()
                                    .mr_2()
                                    .text_color(rgb(0x9ca3af))
                                    .child(if entry_is_dir { "📁" } else { "📄" }),
                            )
                            .child(div().text_color(rgb(0xd1d5db)).child(entry_name)),
                    )
            }))
    }
}
