use crate::component::DirEntry;
use gpui::prelude::FluentBuilder;
use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, Styled, Window,
    div, rgb,
};
use gpui_component::v_flex;
use std::fs;
use std::path::PathBuf;

pub struct FileList {
    entries: Vec<DirEntry>,
    pub selected_path: PathBuf,
}

impl FileList {
    pub fn view(_window: &mut Window, _cx: &mut Context<Self>, selected_path: &PathBuf) -> Self {
        let entries = if selected_path.is_dir() {
            fs::read_dir(selected_path)
                .map(|entries| {
                    entries
                        .filter_map(|entry| entry.ok().map(|entry| DirEntry::from(entry)))
                        .collect()
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        Self {
            entries,
            selected_path: selected_path.clone(),
        }
    }
}

impl Render for FileList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_80()
            .h_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(rgb(0x3d3d3d))
            .children(self.entries.iter().map(|entry| {
                let entry_name = entry.name.clone();
                let entry_is_dir = entry.is_dir;
                let is_selected = self.selected_path == entry.path;
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
