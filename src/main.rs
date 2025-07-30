use gpui::{
    App, Application, Bounds, KeyBinding, Menu, MenuItem, SharedString, WindowBounds,
    WindowOptions, actions, prelude::*, px, size,
};

use toshokan::root::*;

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
    });
}
