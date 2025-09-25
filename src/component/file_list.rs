use crate::components::DirEntry;
use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, SharedString,
    Styled, Window, div, prelude::*, rgb,
};
use gpui_component::{StyledExt, scroll::ScrollbarAxis};
use std::fs;
use std::path::PathBuf;

pub struct FileList {
    entries: Vec<DirEntry>,
    dir: PathBuf,
    pub selected: Option<PathBuf>,
    pub selected_entry: Option<DirEntry>,
}

impl FileList {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, dir: &PathBuf) -> Self {
        let entries = Self::read_directory(dir);
        Self {
            entries,
            dir: dir.clone(),
            selected: None,
            selected_entry: None,
        }
    }

    pub fn select_entry(&mut self, entry: DirEntry, cx: &mut Context<Self>) {
        self.selected = Some(entry.path.clone());
        self.selected_entry = Some(entry);
        cx.notify();
    }

    pub fn update_directory(&mut self, dir: &PathBuf, cx: &mut Context<Self>) {
        self.entries = Self::read_directory(dir);
        self.dir = dir.clone();
        self.selected = None;
        self.selected_entry = None;
        cx.notify();
    }

    pub fn get_selected_entry(&self) -> Option<&DirEntry> {
        self.selected_entry.as_ref()
    }

    fn read_directory(dir: &PathBuf) -> Vec<DirEntry> {
        let mut entries = Vec::new();

        if let Ok(read_dir) = fs::read_dir(dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = path.is_dir();

                entries.push(DirEntry {
                    name: SharedString::from(name),
                    is_dir,
                    path,
                });
            }
        }

        entries
    }
}

impl Render for FileList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x252525))
            .border_r_1()
            .border_color(rgb(0x3d3d3d))
            .flex_col()
            .scrollable(ScrollbarAxis::Vertical)
            .child(div().flex_col().children(self.entries.iter().map(|entry| {
                let entry_name = entry.name.clone();
                let entry_is_dir = entry.is_dir;
                let is_selected = self.selected.as_ref() == Some(&entry.path);
                let entry_clone = entry.clone();
                let on_entry_click = cx.listener(
                    move |this: &mut FileList, _event: &gpui::MouseDownEvent, _window, cx| {
                        this.select_entry(entry_clone.clone(), cx);
                    },
                );

                div()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(rgb(0x2a2a3a))
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
            })))
    }
}
