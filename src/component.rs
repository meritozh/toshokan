pub(crate) mod content_viewer;
pub(crate) mod file_list;
pub(crate) mod header;

use std::path::PathBuf;

pub(crate) use content_viewer::ContentViewer;
pub(crate) use file_list::FileList;
pub(crate) use header::Header;

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
