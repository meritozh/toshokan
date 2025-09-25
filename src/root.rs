use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, px, rgb, white,
};
use std::fs;
use std::path::PathBuf;

use crate::components::header::HeaderEvent;
use crate::components::{ContentViewer, DirEntry, FileList, Header};

pub struct Root {
    current_path: PathBuf,
    entries: Vec<DirEntry>,
    header: Entity<Header>,
    file_list: Entity<FileList>,
    selected_item: Option<DirEntry>,
    file_content: Option<String>,
}

impl Root {
    pub fn view(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let entries = Self::read_directory(&current_path);

        let header = cx.new(|cx| Header::new(None, cx));
        let file_list = cx.new(|cx| FileList::new(_window, cx, &current_path));

        // Set up observer for file selection changes
        let file_list_weak = file_list.downgrade();
        cx.observe(&file_list, move |this: &mut Root, _file_list, cx| {
            if let Some(file_list) = file_list_weak.upgrade() {
                if let Some(selected_entry) = file_list.read(cx).get_selected_entry() {
                    let entry = selected_entry.clone();
                    this.handle_item_click(entry, cx);
                }
            }
        })
        .detach();

        // Set up observer for header navigation events
        cx.subscribe(
            &header,
            move |this: &mut Root, _header, event: &HeaderEvent, cx| match event {
                HeaderEvent::NavigateTo(path) => {
                    this.navigate_to_directory(path.clone(), cx);
                }
            },
        )
        .detach();

        Self {
            current_path,
            entries,
            header,
            file_list,
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

    pub fn handle_item_click(&mut self, entry: DirEntry, cx: &mut Context<Self>) {
        if entry.is_dir {
            // Change directory
            self.navigate_to_directory(entry.path.clone(), cx);
        } else {
            // Read file content
            self.selected_item = Some(entry.clone());
            self.file_content = fs::read_to_string(&entry.path).ok();
            cx.notify();
        }
    }

    pub fn navigate_to_directory(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.current_path = path.clone();
        self.entries = Self::read_directory(&self.current_path);
        self.header.update(cx, |header, cx| {
            header.set_path(self.current_path.clone());
            cx.notify();
        });
        self.file_list.update(cx, |file_list, cx| {
            file_list.update_directory(&self.current_path, cx);
        });
        self.selected_item = None;
        self.file_content = None;
        cx.notify();
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let file_content = self.file_content.clone();

        div()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(white())
            .child(
                div()
                    .flex_col()
                    .size_full()
                    .child(self.header.clone())
                    .child(
                        div()
                            .flex()
                            .size_full()
                            .child(
                                div()
                                    .flex_shrink()
                                    .flex_basis(px(300.0))
                                    .min_w(px(200.0))
                                    .max_w(px(500.0))
                                    .child(self.file_list.clone()),
                            )
                            .child(ContentViewer::new(file_name, file_content)),
                    ),
            )
    }
}
