use gpui::{
    actions, prelude::*, px, size, App, Application, Bounds, KeyBinding,
    Menu, MenuItem, SharedString, WindowBounds, WindowOptions,
};
use gpui_component::Root;

mod component;
mod shelf;
mod ui;

use shelf::Shelf;

actions!(toshokan, [Quit]);

fn main() {
    Application::new().run(move |cx: &mut App| {
        gpui_component::init(cx);

        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, Some("App"))]);
        cx.set_menus(vec![Menu {
            name: "toshokan".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        }]);
        cx.activate(true);

        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let window_options = WindowOptions {
            titlebar: Some(gpui::TitlebarOptions {
                title: Some(SharedString::from("图书馆")),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        };
        if let Err(err) = cx.open_window(window_options, |window, cx| {
            let shelf = cx.new(|cx| Shelf::new(window, cx));
            cx.new(|cx| Root::new(shelf, window, cx))
        }) {
            eprintln!("Failed to open window: {err}");
        }
    });
}
