use gpui::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, div, rgb, white,
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
            .child(Header::new(self.current_path.clone()).render())
            .child(
                div()
                    .flex()
                    .h_full()
                    .child(FileList::new(entries, selected_path).render(cx))
                    .child(ContentViewer::new(file_name, file_content).render()),
            )
    }
}
