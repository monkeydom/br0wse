use std::thread;
use std::time::Duration;

use gpui::*;
use serde::Deserialize;

const APP_NAME: &str = "Br0wse";

struct Services {
    services: Vec<String>,
}

impl Services {
    pub fn new() -> Self {
        Self {
            services: vec!["one".into(), "two".into()],
        }
    }

    pub fn add(&mut self, service: &str) {
        self.services.push(service.into());
    }
}

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
    services: Model<Services>,
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let model = &self.services.read(cx).services;
        let services = self.services.clone();
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x202020))
            .size_full()
            .min_w_64()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .bg(rgb(0x002200))
                    .h_full()
                    .min_w_20()
                    .relative()
                    .child(
                        list(ListState::new(
                            model.len(),
                            ListAlignment::Top,
                            Pixels(40.0),
                            move |i, cx| {
                                div()
                                    .w_full()
                                    .border_1()
                                    .border_color(rgb(0x123456))
                                    .child(format!("Line {}: {}", i, services.read(cx).services[i]))
                                    .px_2()
                                    .py_1()
                                    .into_any()
                            },
                        ))
                        .size_full(),
                    ),
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
    env_logger::init();

    App::new().run(|cx: &mut AppContext| {
        basic_setup(cx);

        cx.activate(true);
        cx.dispatch_action(&NewWindow);

        // let _handle = thread::Builder::new()
        //     .name("zeroconf-browsing".into())
        //     .spawn(move || {
        //         extern crate log;
        //         use std::any::Any;
        //         use std::sync::Arc;
        //         use zeroconf::{prelude::*, MdnsBrowser, ServiceDiscovery, ServiceType};

        //         fn on_service_discovered(
        //             result: zeroconf::Result<ServiceDiscovery>,
        //             _context: Option<Arc<dyn Any>>,
        //         ) {
        //             println!(
        //                 "Found some: {:?}",
        //                 result.expect("Discovery failed instead")
        //             )
        //         }

        //         //                    let services_type = "_services._dns-sd._udp.";
        //         let service_type =
        //             ServiceType::with_sub_types("dns-sd", "udp", ["services"].into()).unwrap();
        //         // let service_type = ServiceType::new("services.dns-sd", "udp").unwrap();
        //         // let service_type = ServiceType::new("ssh", "tcp").unwrap();
        //         //                let service_type = ServiceType::services_type();

        //         let mut browser = MdnsBrowser::new(service_type.clone());
        //         browser.set_service_discovered_callback(Box::new(on_service_discovered));
        //         let event_loop = browser
        //             .browse_services()
        //             .expect(&format!("Could not start Browsing for {:?}", &service_type));
        //         println!("Entering our event loop, browsing for {:?}", &service_type);
        //         loop {
        //             _ = event_loop.poll(Duration::from_millis(500));
        //         }
        //     })
        //     .unwrap();
        // println!("spawned browser");
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
    let services = cx.new_model(|_cx| Services::new());
    let view_services = services.clone();
    let ssh_services = services.clone();
    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(format!("{APP_NAME} - Main Window").into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        |cx| {
            cx.new_view(move |_cx| HelloWorld {
                text: "World".into(),
                services: view_services,
            })
        },
    );
    cx.spawn(|mut cx| async move {
        loop {
            cx.background_executor().timer(Duration::from_secs(2)).await;
            _ = cx.update_model(&services, |s, _cx| {
                s.add("one more");
            });
            _ = cx.refresh();
        }
    })
    .detach();

    let (tx, rx) = smol::channel::unbounded();
    thread::Builder::new()
        .name("ssh-browse".into())
        .spawn(move || {
            extern crate log;
            use smol::channel::Sender;
            use std::any::Any;
            use std::sync::Arc;
            use zeroconf::{prelude::*, MdnsBrowser, ServiceDiscovery, ServiceType};

            fn on_service_discovered(
                result: zeroconf::Result<ServiceDiscovery>,
                context: Option<Arc<dyn Any>>,
            ) {
                println!(
                    "Found some: {:?}",
                    result.as_ref().expect("Discovery failed instead")
                );
                if let Some(tx) = context {
                    let tx = tx.downcast_ref::<Sender<String>>().unwrap();

                    smol::block_on(tx.send(format!("Found Some {:?}", result.unwrap()))).unwrap();
                }
            }

            smol::block_on(tx.send("Started Browsing for _ssh".to_string())).unwrap();

            //                    let services_type = "_services._dns-sd._udp.";
            // let service_type =
            // ServiceType::with_sub_types("dns-sd", "udp", ["services"].into()).unwrap();
            // let service_type = ServiceType::new("services.dns-sd", "udp").unwrap();
            let service_type = ServiceType::new("ssh", "tcp").unwrap();
            //                let service_type = ServiceType::services_type();

            let mut browser = MdnsBrowser::new(service_type.clone());
            browser.set_context(Box::new(tx));

            browser.set_service_discovered_callback(Box::new(on_service_discovered));
            let event_loop = browser
                .browse_services()
                .expect(&format!("Could not start Browsing for {:?}", &service_type));
            println!("Entering our event loop, browsing for {:?}", &service_type);
            loop {
                _ = event_loop.poll(Duration::from_millis(500));
            }
        })
        .unwrap();

    cx.spawn(|mut cx| async move {
        loop {
            let s = rx.recv().await.unwrap();
            println!("Received: {}", s);
            _ = cx.update_model(&ssh_services, |services, _cx| services.add(&s));
            _ = cx.refresh();
        }
    })
    .detach();
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
            window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                Point::new(10.into(), 10.into()),
                Size {
                    width: 300.into(),
                    height: 40.into(),
                },
            ))),
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
