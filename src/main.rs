use gpui::{
    App, Application, Bounds, SharedString, WindowBounds, WindowOptions, div, prelude::*, px, red,
    size,
};

struct Root {}

impl Render for Root {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("hello, world").text_color(red())
    }
}

fn main() {
    Application::new().run(move |cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        let window_options = WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some(SharedString::from("图书馆")),
                appears_transparent: false,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        cx.open_window(window_options, |_, cx| cx.new(|_| Root {}))
            .unwrap();
        cx.activate(true);
    });
}
