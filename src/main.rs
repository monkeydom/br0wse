use gpui::*;

const APP_NAME: &str = "Br0wse";

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x202020))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn basic_setup(cx: &mut AppContext) {
    // Register the `quit` function so it can be referenced by the `MenuItem::action` in the menu bar
    cx.on_action(quit);
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    cx.on_action(new_window);
    cx.bind_keys([KeyBinding::new("cmd-n", NewWindow, None)]);
    cx.on_action(close_active_item);
    cx.bind_keys([KeyBinding::new("cmd-w", CloseActiveItem, None)]);
    cx.on_action(about);

    // Add menu items
    cx.set_menus(vec![
        Menu {
            name: "<AppMenu>",
            items: vec![
                MenuItem::action(&format!("About {APP_NAME}"), About),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "File",
            items: vec![
                MenuItem::action("New Window", NewWindow),
                MenuItem::separator(),
                MenuItem::action("Close Window", CloseActiveItem),
            ],
        },
    ]);
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        basic_setup(cx);

        cx.activate(true);
        cx.dispatch_action(&NewWindow);
    });
}

// Associate actions using the `actions!` macro (or `impl_actions!` macro)
actions!(br0wse, [About, NewWindow, CloseActiveItem, Quit]);

// Define the quit function that is registered with the AppContext
fn quit(_: &Quit, cx: &mut AppContext) {
    cx.quit();
}

fn new_window(_: &NewWindow, cx: &mut AppContext) {
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(format!("{APP_NAME} - Main Window").into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        },
    );
}

fn about(_: &About, cx: &mut AppContext) {
    let version = env!("CARGO_PKG_VERSION");
    let message = format!("{APP_NAME} {version}");

    cx.open_window(Default::default(), |cx| {
        cx.set_window_title(&message);
        cx.new_view(|_cx| EmptyView {})
    });
}

fn close_active_item(_: &CloseActiveItem, cx: &mut AppContext) {
    cx.defer(|cx| {
        if let Some(handle) = cx.active_window() {
            handle
                .update(cx, |_, window| {
                    window.remove_window();
                    true
                })
                .unwrap();
        }
    });
}
