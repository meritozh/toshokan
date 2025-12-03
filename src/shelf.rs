use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, px, rgb, white,
};
use std::fs;
use std::path::PathBuf;

use crate::component::header::HeaderEvent;
use crate::component::content_viewer::ContentViewer;
use crate::component::header::Header;
use crate::component::DirEntry;
use crate::component::file_tree::FileTree;

pub struct Shelf {
    current_path: PathBuf,
    entries: Vec<DirEntry>,
    header: Entity<Header>,
    file_tree: Entity<FileTree>,
    selected_item: Option<DirEntry>,
    file_content: Option<String>,
}

impl Shelf {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let entries = Vec::new();

        let header = cx.new(|cx| Header::view(window, cx, None));
        let file_tree = FileTree::view(window, cx);

        // Set up observer for file selection changes
        let file_tree_weak = file_tree.downgrade();
        cx.observe(&file_tree, move |this: &mut Shelf, _file_tree, cx| {
            if let Some(file_tree) = file_tree_weak.upgrade() {
                if let Some(path) = file_tree.read(cx).selected_path() {
                    let is_dir = path.is_dir();
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let entry = DirEntry {
                        name: SharedString::from(name),
                        is_dir,
                        path,
                    };
                    this.handle_item_click(entry, cx);
                }
            }
        })
        .detach();

        // Set up observer for header navigation events
        cx.subscribe(
            &header,
            move |this: &mut Shelf, _header, event: &HeaderEvent, cx| match event {
                HeaderEvent::NavigateTo(path) => {
                    this.navigate_to_directory(path.clone(), cx);
                }
            },
        )
        .detach();

        let mut this = Self {
            current_path,
            entries,
            header,
            file_tree,
            selected_item: None,
            file_content: None,
        };

        this.load_directory_async(this.current_path.clone(), cx);
        this
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
            // Read file content in background
            let path = entry.path.clone();
            self.selected_item = Some(entry);
            let view = cx.entity();
            let task = cx.background_spawn(async move {
                fs::read_to_string(&path)
            });
            cx.spawn(async move |_, cx| {
                let content = task.await.ok();
                if let Err(err) = view.update(cx, |this, cx| {
                    this.file_content = content;
                    cx.notify();
                }) {
                    eprintln!("Update file content failed: {err}");
                }
            })
            .detach();
        }
    }

    pub fn navigate_to_directory(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.current_path = path.clone();
        self.load_directory_async(self.current_path.clone(), cx);
        self.header.update(cx, |header, cx| {
            header.set_path(self.current_path.clone());
            cx.notify();
        });
        self.file_tree.update(cx, |tree, cx| {
            tree.set_root_path(self.current_path.clone(), cx);
        });
        self.selected_item = None;
        self.file_content = None;
        cx.notify();
    }

    fn load_directory_async(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let view = cx.entity();
        let task = cx.background_spawn(async move {
            let mut entries = Vec::new();
            if let Ok(read_dir) = fs::read_dir(&path) {
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
            entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            });
            entries
        });
        cx.spawn(async move |_, cx| {
            let entries = task.await;
            if let Err(err) = view.update(cx, |this, cx| {
                this.entries = entries;
                cx.notify();
            }) {
                eprintln!("Update entries failed: {err}");
            }
        })
        .detach();
    }
}

impl Render for Shelf {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let file_content = self.file_content.clone();
        let file_name = self
            .selected_item
            .as_ref()
            .map(|e| e.name.clone());

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
                                    .child(self.file_tree.clone()),
                            )
                            .child(ContentViewer::new(file_name, file_content)),
                    ),
            )
    }
}
