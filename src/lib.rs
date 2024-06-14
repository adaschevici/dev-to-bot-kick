use dioxus::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    launch(app);
}

fn app() -> Element {
    let mut count = use_signal_sync(|| 0);
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(100));
        println!("{count}");
    });
    rsx!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    )
}
