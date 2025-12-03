use autocorrect::ignorer::Ignorer;
use gpui::{actions, px, App, AppContext, Context, Entity, InteractiveElement, IntoElement, KeyBinding, ParentElement, Render, Styled, Window};
use gpui_component::label::Label;
use gpui_component::{h_flex, v_flex, ActiveTheme, IconName};
use gpui_component::list::ListItem;
use gpui_component::tree::{TreeItem, TreeState, tree};
use std::fs::read_dir;
use std::path::PathBuf;

const CONTEXT: &str = "FileTree";

actions!(toshokan, [Rename, SelectItem]);

fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", Rename, Some(CONTEXT)),
        KeyBinding::new("space", SelectItem, Some(CONTEXT)),
    ])
}

pub struct FileTree {
    tree_state: Entity<TreeState>,
    selected_item: Option<TreeItem>,
}

impl FileTree {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let tree_state = cx.new(|cx| TreeState::new(cx));

        Self::load_files(tree_state.clone(), PathBuf::from("./"), cx);

        Self {
            tree_state,
            selected_item: None,
        }
    }

    fn load_files(state: Entity<TreeState>, path: PathBuf, cx: &mut App) {
        cx.spawn(async move |cx| {
            let ignorer = Ignorer::new(&path.to_string_lossy());
            let items = build_file_items(&ignorer, &path, &path);
            if let Err(err) = state.update(cx, |state, cx| {
                state.set_items(items, cx);
            }) {
                eprintln!("FileTree set_items failed: {err}");
            }
        }).detach();
    }

    fn on_action_rename(&mut self, _: &Rename, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.tree_state.read(cx).selected_entry() {
            let _ = entry.item();
            todo!()
        }
    }

    fn on_action_select_item(&mut self, _: &SelectItem, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(entry) = self.tree_state.read(cx).selected_entry() {
            self.selected_item = Some(entry.item().clone());
            cx.notify();
        }
    }

    pub fn selected_path(&self) -> Option<PathBuf> {
        self.selected_item
            .as_ref()
            .map(|item| PathBuf::from(item.id.to_string()))
    }

    pub fn set_root_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let state = self.tree_state.clone();
        // Context<T> 可用作 &mut App 传递到 load_files
        Self::load_files(state, path, cx);
    }
}

impl Render for FileTree {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        v_flex()
            .id("file-tree-view")
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::on_action_rename))
            .on_action(cx.listener(Self::on_action_select_item))
            .gap_5()
            .size_full()
            .child(
                tree(
                    &self.tree_state,
                    move |idx, entry, _, _, cx| {
                        view.update(cx, |_, cx| {
                            let item = entry.item();
                            let icon = if !entry.is_folder() {
                                IconName::File
                            } else if entry.is_expanded() {
                                IconName::FolderOpen
                            } else {
                                IconName::Folder
                            };

                            ListItem::new(idx)
                                .w_full()
                                .rounded(cx.theme().radius)
                                .px_3()
                                .pl(px(16.) * entry.depth() + px(12.))
                                .child(
                                    h_flex().gap_2().child(icon).child(item.label.clone())
                                )
                                .on_click(cx.listener({
                                    let item = item.clone();
                                    move |this, _, _, cx| {
                                        this.selected_item = Some(item.clone());
                                        cx.notify()
                                    }
                                }))
                        })
                    })
                    .p_1()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .h_full()
            )
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_3()
                    .children(
                        self.tree_state
                            .read(cx)
                            .selected_index()
                            .map(|idx| format!("Selected Index: {}", idx))
                    )
                    .children(
                        self.selected_item
                            .as_ref()
                            .map(|item| Label::new("Selected:").secondary(item.id.clone()))
                    )
            )
    }
}

fn build_file_items(ignorer: &Ignorer, root: &PathBuf, path: &PathBuf) -> Vec<TreeItem> {
    let mut items = Vec::new();
    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path.strip_prefix(root).unwrap_or(&path);
            if ignorer.is_ignored(&relative_path.to_string_lossy()) || relative_path.ends_with(".git") {
                continue;
            }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown").to_string();
            let id = path.to_string_lossy().to_string();
            if path.is_dir() {
                let children = build_file_items(ignorer, &root, &path);
                items.push(TreeItem::new(id, file_name).children(children));
            } else {
                items.push(TreeItem::new(id, file_name));
            }
        }
    }
    items.sort_by(|a, b| {
        b.is_folder()
            .cmp(&a.is_folder())
            .then(a.label.cmp(&b.label))
    });
    items
}
