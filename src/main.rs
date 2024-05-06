use gpui::*;
use serde::Deserialize;

const APP_NAME: &str = "Br0wse";

#[derive(Clone, PartialEq, Deserialize)]
pub struct OpenBrowser {
    pub url: String,
}

impl_actions!(br0wse, [OpenBrowser]);

pub(crate) mod editor {
    pub(crate) mod actions {
        gpui::actions!(editor, [Copy, Cut, Paste, SelectAll, Redo, Undo,]);
    }
}

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x202020))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .bg(rgb(0x002200))
                    .size_full()
                    .relative()
                    .child(uniform_list(cx.view().clone(), "entries", 8, {
                        |_this, range, _cx| {
                            let mut items = Vec::new();
                            for idx in range {
                                items.push(div().child(format!("line {idx}")))
                            }
                            items
                        }
                    })),
            )
    }
}

fn basic_setup(cx: &mut AppContext) {
    cx.on_action(quit);
    cx.bind_keys([
        KeyBinding::new("cmd-m", Minimize, None),
        KeyBinding::new("cmd-w", CloseActiveItem, None),
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-n", NewWindow, None),
    ]);
    cx.on_action(new_window);
    cx.on_action(close_active_item);
    cx.on_action(about);
    cx.on_action(|action: &OpenBrowser, cx| cx.open_url(&action.url));
    cx.on_action(|_: &Minimize, cx| {
        cx.defer(|cx| {
            if let Some(window) = cx.active_window() {
                _ = window.update(cx, |_, cx| {
                    cx.minimize_window();
                    true
                });
            }
        });
    });
    cx.on_action(|_: &Zoom, cx| {
        cx.defer(|cx| {
            if let Some(window) = cx.active_window() {
                _ = window.update(cx, |_, cx| {
                    cx.zoom_window();
                    true
                });
            }
        });
    });

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
        Menu {
            name: "Edit",
            items: vec![
                MenuItem::os_action("Undo", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::actions::Copy, OsAction::Copy),
                MenuItem::os_action("Paste", editor::actions::Paste, OsAction::Paste),
                MenuItem::os_action(
                    "Select All",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
            ],
        },
        Menu {
            name: "Window",
            items: vec![
                MenuItem::action("Minimize", Minimize),
                MenuItem::action("Zoom", Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help",
            items: vec![MenuItem::action(
                "Documentation",
                OpenBrowser {
                    url: "https://github.com/monkeydom/br0wse".into(),
                },
            )],
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
actions!(
    br0wse,
    [About, NewWindow, CloseActiveItem, Quit, Zoom, Minimize]
);

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

struct AboutView {
    text: SharedString,
}

impl Render for AboutView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgba(0x12345678))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("{}", &self.text))
    }
}
fn about(_: &About, cx: &mut AppContext) {
    let version = env!("CARGO_PKG_VERSION");
    let message = format!("{APP_NAME} {version}");

    cx.open_window(
        WindowOptions {
            bounds: Some(Bounds::new(
                Point::new(10.into(), 10.into()),
                Size {
                    width: 300.into(),
                    height: 40.into(),
                },
            )),
            window_background: WindowBackgroundAppearance::Blurred,
            ..Default::default()
        },
        |cx| {
            cx.set_window_title(&message);
            cx.new_view(|_cx| AboutView {
                text: message.clone().into(),
            })
        },
    );
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
