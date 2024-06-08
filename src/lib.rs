#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::module_name_repetitions)]
//! A web app the search a set of images.

/// TODO public to that main can see this
/// is this correct?
pub mod app;
mod component;
mod indexer;
mod pages;
mod util;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    use wasm_bindgen::prelude::wasm_bindgen;
    use leptos::view;
    use crate::app::App;

    /// Hydrate entry function logging initialisation
    /// and mount point for App.
    #[wasm_bindgen]
    pub fn hydrate() {
      console_error_panic_hook::set_once();
    FmtSubscriber::builder()
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
