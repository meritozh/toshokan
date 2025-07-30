use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, ScrollHandle, SharedString,
    Styled, Window, div, px, rgb, white,
};
use gpui_component::{scroll::ScrollbarState, v_flex, v_virtual_list};
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Root {
    current_path: PathBuf,
    entries: Vec<DirEntry>,
    scroll_handle: ScrollHandle,
    scroll_state: ScrollbarState,
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
            scroll_handle: ScrollHandle::new(),
            scroll_state: ScrollbarState::default(),
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
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = self.entries.clone();
        let item_heights: Vec<_> = entries
            .iter()
            .map(|_| gpui::size(px(0.0), px(40.0)))
            .collect();
        let item_heights = Rc::new(item_heights);

        v_flex()
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
                v_flex()
                    .flex_1()
                    .relative()
                    .child(
                        v_virtual_list(
                            cx.entity().clone(),
                            "entries",
                            item_heights.clone(),
                            move |_view, visible_range, _window, _cx, _| {
                                visible_range
                                    .map(|ix| {
                                        let entry = &entries[ix];
                                        div()
                                            .px_4()
                                            .py_2()
                                            .border_b_1()
                                            .border_color(rgb(0x2a2a2a))
                                            .hover(|s| s.bg(rgb(0x2d2d2d)))
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
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(&self.scroll_handle)
                        .border_1()
                        .border_color(rgb(0x333333)),
                    )
                    .child(div().absolute().top_0().right_0().bottom_0().child(
                        gpui_component::scroll::Scrollbar::vertical(
                            &self.scroll_state,
                            &self.scroll_handle,
                        ),
                    )),
            )
    }
}
