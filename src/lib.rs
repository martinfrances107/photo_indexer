pub mod app;
mod indexer;
mod pages;

extern crate seroost_lib;

use cfg_if::cfg_if;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

cfg_if! {
if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;
    use leptos::view;
    use crate::app::App;

    #[wasm_bindgen]
    pub fn hydrate() {
      console_error_panic_hook::set_once();
    // simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
      leptos::mount_to_body(move || {
          view! { <App/> }
      });
    }
}
}
