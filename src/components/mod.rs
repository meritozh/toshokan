pub mod content_viewer;
pub mod file_list;
pub mod header;

pub use content_viewer::ContentViewer;
pub use file_list::FileList;
pub use header::Header;

#[derive(Clone, Debug)]
pub struct DirEntry {
    pub name: gpui::SharedString,
    pub is_dir: bool,
    pub path: std::path::PathBuf,
}
