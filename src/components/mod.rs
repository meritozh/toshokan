pub mod content_viewer;
pub mod file_list;
pub mod header;

use std::path::PathBuf;

pub use content_viewer::ContentViewer;
pub use file_list::FileList;
pub use header::Header;

#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: gpui::SharedString,
    pub is_dir: bool,
    pub path: PathBuf,
}

impl From<std::fs::DirEntry> for DirEntry {
    fn from(value: std::fs::DirEntry) -> Self {
        DirEntry {
            name: value.file_name().into_string().unwrap().into(),
            is_dir: value.path().is_dir(),
            path: value.path(),
        }
    }
}
