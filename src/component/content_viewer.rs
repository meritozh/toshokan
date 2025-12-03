use gpui::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, Window, div, px,
};
use gpui_component::ActiveTheme;
use std::path::PathBuf;
use gpui::AppContext;

pub struct ContentViewer {
    file_name: Option<SharedString>,
    file_path: Option<PathBuf>,
    content: Option<SharedString>,
    loading: bool,
    error: Option<SharedString>,
    image_cols: usize,
    image_rows: usize,
    image_grid: Option<Vec<Vec<(u8, u8, u8)>>>,
}

impl ContentViewer {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self { file_name: None, file_path: None, content: None, loading: false, error: None, image_cols: 0, image_rows: 0, image_grid: None }
    }

    pub fn set_text(&mut self, file_name: Option<SharedString>, content: Option<String>, cx: &mut Context<Self>) {
        self.file_name = file_name;
        self.file_path = None;
        self.content = content.map(SharedString::from);
        self.loading = false;
        self.error = None;
        cx.notify();
    }

    pub fn set_image_path(&mut self, file_name: Option<SharedString>, path: PathBuf, cx: &mut Context<Self>) {
        self.file_name = file_name;
        self.file_path = Some(path);
        self.content = None;
        self.loading = true;
        self.error = None;
        self.image_grid = None;
        self.image_cols = 0;
        self.image_rows = 0;
        cx.notify();

        let handle = cx.entity().downgrade();
        let path2 = self.file_path.clone().unwrap();
        let task = cx.background_spawn(async move {
            image::open(&path2)
                .map_err(|e| e.to_string())
                .and_then(|img| {
                    let img = img.to_rgb8();
                    let (w, h) = (img.width() as usize, img.height() as usize);
                    let target_cols = w.min(256);
                    let scale = (w as f32 / target_cols as f32).max(1.0);
                    let target_rows = ((h as f32) / scale).round() as usize;
                    let mut grid: Vec<Vec<(u8, u8, u8)>> = Vec::with_capacity(target_rows);
                    for row in 0..target_rows {
                        let mut rvec = Vec::with_capacity(target_cols);
                        let src_y = ((row as f32) * scale) as usize;
                        for col in 0..target_cols {
                            let src_x = ((col as f32) * scale) as usize;
                            let idx = (src_y.min(h - 1) * w + src_x.min(w - 1)) * 3;
                            let p = &img.as_raw()[idx..idx + 3];
                            rvec.push((p[0], p[1], p[2]));
                        }
                        grid.push(rvec);
                    }
                    Ok((target_cols, target_rows, grid))
                })
        });
        cx.spawn(async move |_, cx| {
            let res = task.await;
            if let Some(cv) = handle.upgrade() {
                let _ = cv.update(cx, |this, cx| {
                    match res {
                        Ok((c, r, g)) => {
                            this.loading = false;
                            this.error = None;
                            this.image_cols = c;
                            this.image_rows = r;
                            this.image_grid = Some(g);
                        }
                        Err(err) => {
                            this.loading = false;
                            this.error = Some(SharedString::from(err));
                            this.image_cols = 0;
                            this.image_rows = 0;
                            this.image_grid = None;
                        }
                    }
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub fn is_image_name(name: &SharedString) -> bool {
        let s = name.to_string().to_lowercase();
        s.ends_with(".png") || s.ends_with(".jpg") || s.ends_with(".jpeg") || s.ends_with(".gif") || s.ends_with(".bmp") || s.ends_with(".webp")
    }
}

impl Render for ContentViewer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = if let Some(file_name) = &self.file_name {
            div()
                .p_3()
                .border_b_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius)
                .child(div().child(format!("📄 {}", file_name)))
        } else {
            div()
                .p_3()
                .border_b_1()
                .border_color(cx.theme().border)
                .rounded(cx.theme().radius)
                .child(div().child("No file selected"))
        };

        let body = if self.loading {
            div().size_full().flex().items_center().justify_center().child("Loading...")
        } else if let Some(err) = &self.error {
            div().size_full().flex().items_center().justify_center().child(format!("Error: {}", err))
        } else if let Some(content) = &self.content {
            div()
                .flex_1()
                .p_4()
                .child(
                    div()
                        .font_family("Consolas, Monaco, 'Courier New', monospace")
                        .text_sm()
                        .whitespace_normal()
                        .child(content.clone()),
                )
        } else if let Some(grid) = &self.image_grid {
            let tile = {
                let t = (1024.0 / (self.image_cols.max(1) as f32)).clamp(2.0, 8.0);
                px(t)
            };
            let mut column_divs = Vec::new();
            for row in grid.iter() {
                let mut row_div = div().flex().gap(px(0.0));
                for (r, g, b) in row.iter() {
                    let color = ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32);
                    row_div = row_div.child(div().w(tile).h(tile).bg(gpui::rgb(color)));
                }
                column_divs.push(row_div);
            }
            let mut container = div().flex_col().p_4().gap(px(0.0));
            for rd in column_divs {
                container = container.child(rd);
            }
            container
        } else {
            div().size_full().flex().items_center().justify_center().child("Select a file to view its contents")
        };

        div().flex_1().flex_col().child(header).child(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn image_ext_recognition() {
        for name in ["a.PNG", "b.jpg", "c.JPEG", "d.gif", "e.bmp", "f.webp"] {
            assert!(ContentViewer::is_image_name(&SharedString::from(name.to_string())));
        }
        for name in ["a.txt", "b.md", "c.rs"] {
            assert!(!ContentViewer::is_image_name(&SharedString::from(name.to_string())));
        }
    }
}
