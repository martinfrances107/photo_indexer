pub mod app;
mod doc_links;
mod gallery;
mod indexer;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;
    use leptos::view;
    use crate::app::App;

    #[wasm_bindgen]
    pub fn hydrate() {
      console_error_panic_hook::set_once();

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
