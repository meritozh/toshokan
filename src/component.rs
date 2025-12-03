pub(crate) mod content_viewer;
pub(crate) mod header;
pub(crate) mod file_tree;
mod left_dock;

use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: gpui::SharedString,
    pub is_dir: bool,
    pub path: PathBuf,
}

impl From<std::fs::DirEntry> for DirEntry {
    fn from(value: std::fs::DirEntry) -> Self {
        DirEntry {
            name: value
                .file_name()
                .into_string()
                .unwrap_or_else(|os| os.to_string_lossy().to_string())
                .into(),
            is_dir: value.path().is_dir(),
            path: value.path(),
        }
    }
}
