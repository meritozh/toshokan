use gpui::{
    Context, CursorStyle, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    Styled, Window, div, rgb, white,
};
use std::fs;
use std::path::PathBuf;

use crate::components::{ContentViewer, DirEntry, FileList, Header};

pub struct Root {
    current_path: PathBuf,
    entries: Vec<DirEntry>,
    selected_item: Option<DirEntry>,
    file_content: Option<String>,
}

impl Root {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
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

    pub fn handle_item_click(
        &mut self,
        entry: DirEntry,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if entry.is_dir {
            // Change directory
            self.current_path = entry.path.clone();
            self.entries = Self::read_directory(&self.current_path);
            self.header.set_path(self.current_path.clone());
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
        let selected_path = self.selected_item.as_ref().map(|item| item.path.clone());
        let entries = self.entries.clone();
        let file_name = self.selected_item.as_ref().map(|item| item.name.clone());
        let file_content = self.file_content.clone();

        div()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(white())
            .on_key_down(
                cx.listener(|this, event: &gpui::KeyDownEvent, _window, cx| {
                    if this.header.is_editing {
                        match event.keystroke.key.as_str() {
                            "enter" => {
                                let new_path = PathBuf::from(this.header.path_input.to_string());
                                if new_path.exists() && new_path.is_dir() {
                                    this.current_path = new_path;
                                    this.entries = Self::read_directory(&this.current_path);
                                    this.header.set_path(this.current_path.clone());
                                    this.header.is_editing = false;
                                    this.selected_item = None;
                                    this.file_content = None;
                                    cx.notify();
                                }
                            }
                            "escape" => {
                                this.header.is_editing = false;
                                this.header.set_path(this.current_path.clone());
                                cx.notify();
                            }
                            "backspace" => {
                                let mut text = this.header.path_input.to_string();
                                text.pop();
                                this.header.path_input = SharedString::from(text);
                                cx.notify();
                            }
                            key => {
                                if key.len() == 1 {
                                    let mut text = this.header.path_input.to_string();
                                    text.push_str(key);
                                    this.header.path_input = SharedString::from(text);
                                    cx.notify();
                                }
                            }
                        }
                    }
                }),
            )
            .child(
                div()
                    .flex_col()
                    .size_full()
                    .child(Header {
                        current_path: current_path.clone(),
                        path_input: SharedString::from(current_path.to_string_lossy().to_string()),
                        focus_handle: cx.focus_handle(),
                        is_editing: false,
                    })
                    .child(
                        div()
                            .flex()
                            .size_full()
                            .child(FileList::new(entries, selected_path))
                            .child(ContentViewer::new(file_name, file_content)),
                    ),
            )
    }
}
