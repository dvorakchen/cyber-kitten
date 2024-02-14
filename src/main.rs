use leptos::*;

mod app;
mod cats;

use app::App;

fn main() {
    if cfg!(debug_assertions) {
        _ = console_log::init_with_level(log::Level::Debug);
        logging::log!("csr mode - mounting to body");
    } else {
        _ = console_log::init_with_level(log::Level::Warn);
    }
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    });
}
