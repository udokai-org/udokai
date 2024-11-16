#[allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_logger::tracing::{info, Level};
use dioxus::desktop::{
    tao::dpi::PhysicalPosition, LogicalSize, WindowBuilder,
};

pub fn show() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    LaunchBuilder::desktop()
        .with_cfg(make_config())
        .launch(App);
}

fn make_config() -> dioxus::desktop::Config {
    dioxus::desktop::Config::default()
        .with_menu(None)
        .with_window(make_window())
}

fn make_window() -> WindowBuilder {
    WindowBuilder::new()
        .with_title("udokai")
        .with_transparent(false)
        .with_decorations(false)
        .with_resizable(false)
        .with_always_on_top(true)
        .with_position(PhysicalPosition::new(0, 0))
        .with_max_inner_size(LogicalSize::new(100000, 50))
}

#[component]
fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        input {
            placeholder: "Type something...",
            onchange: |e| {
                info!("input event: {:?}", e);
            }
        }
    }
}

