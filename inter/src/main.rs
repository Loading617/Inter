use dioxus::prelude::*;
use dioxus_desktop::Config;

fn main() {
    dioxus_desktop::launch(App, Config::new());
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Inter" }
    })
}
