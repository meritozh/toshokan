use gpui::prelude::FluentBuilder;
use gpui::{
    Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, SharedString,
    Styled, Window, div, rgb, white,
};
use gpui_component::v_flex;
use std::fs;
use std::path::PathBuf;

pub struct Root {
    current_path: PathBuf,
    entries: Vec<DirEntry>,

    selected_item: Option<DirEntry>,
    file_content: Option<String>,
}

#[derive(Clone, Debug)]
struct DirEntry {
    name: SharedString,
    is_dir: bool,
    path: PathBuf,
}

impl Root {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let entries = Self::read_directory(&current_path);

        Self {
            current_path,
            entries,

            selected_item: None,
            file_content: None,
        }
    }

    fn read_directory(path: &PathBuf) -> Vec<DirEntry> {
        let mut entries = Vec::new();

        if let Ok(read_dir) = fs::read_dir(path) {
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

        // Sort: directories first, then files, both alphabetically
        entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        entries
    }

    fn handle_item_click(&mut self, entry: DirEntry, _window: &mut Window, cx: &mut Context<Self>) {
        if entry.is_dir {
            // Change directory
            self.current_path = entry.path.clone();
            self.entries = Self::read_directory(&self.current_path);
            self.selected_item = None;
            self.file_content = None;
        } else {
            // Read file content
            self.selected_item = Some(entry.clone());
            self.file_content = fs::read_to_string(&entry.path).ok();
        }
        cx.notify();
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_item = self.selected_item.clone();
        let entries = self.entries.clone();

        div()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(white())
            .child(
                div().p_4().border_b_1().border_color(rgb(0x333333)).child(
                    div()
                        .text_color(white())
                        .child(format!("Current: {}", self.current_path.display())),
                ),
            )
            .child(
                div()
                    .flex()
                    .h_full()
                    .child(
                        div()
                            .w_1_2()
                            .border_r_1()
                            .border_color(rgb(0x333333))
                            .child(v_flex().size_full().child(div().children(
                                entries.iter().enumerate().map(|(_ix, entry)| {
                                    let entry_clone = entry.clone();
                                    let is_selected = selected_item
                                        .as_ref()
                                        .map_or(false, |s| s.path == entry.path);

                                    div()
                                        .px_4()
                                        .py_2()
                                        .border_b_1()
                                        .border_color(rgb(0x2a2a2a))
                                        .when(is_selected, |s| s.bg(rgb(0x3d3d3d)))
                                        .hover(|s| s.bg(rgb(0x2d2d2d)))
                                        .on_mouse_down(
                                            gpui::MouseButton::Left,
                                            cx.listener(move |this, _event, window, cx| {
                                                this.handle_item_click(
                                                    entry_clone.clone(),
                                                    window,
                                                    cx,
                                                );
                                            }),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_color(if entry.is_dir {
                                                            rgb(0x60a5fa)
                                                        } else {
                                                            rgb(0x9ca3af)
                                                        })
                                                        .child(if entry.is_dir {
                                                            "📁"
                                                        } else {
                                                            "📄"
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .text_color(white())
                                                        .child(entry.name.clone()),
                                                ),
                                        )
                                }),
                            ))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_4()
                            .child(if let Some(content) = &self.file_content {
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
                            }),
                    ),
            )
    }
}
