use dioxus::prelude::*;
use dioxus_web::use_eval;

fn App(cx: Scope) -> Element {
    let eval = use_eval(cx);

    let get_device_info = move || {
        eval(
            r#"
            navigator.userAgent
        "#,
        )
        .unwrap()
        .then(|result| {
            if let Ok(info) = result {
                log::info!("Device Info: {:?}", info);
            }
        });
    };

    cx.render(rsx! {
        div {
            button { onclick: move |_| get_device_info(), "Get Device Info" }
        }
    })
}
