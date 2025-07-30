use crate::components::DirEntry;
use crate::root::Root;
use gpui::prelude::FluentBuilder;
use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, div, rgb,
};
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

    pub fn render(&self, cx: &mut Context<Root>) -> impl IntoElement {
        v_flex()
            .w_80()
            .h_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(rgb(0x3d3d3d))
            .children(self.entries.iter().map(|entry| {
                let entry_for_click = entry.clone();
                let entry_name = entry.name.clone();
                let entry_is_dir = entry.is_dir;
                let is_selected = self.selected_path.as_ref() == Some(&entry.path);
                let on_entry_click = cx.listener(
                    move |this: &mut Root, _event: &gpui::MouseDownEvent, window, cx| {
                        this.handle_item_click(entry_for_click.clone(), window, cx);
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
