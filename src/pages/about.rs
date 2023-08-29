use std::path::Path;
use std::time::Instant;

use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::*;
use leptos_meta::Style;
use tracing::info;

use crate::indexer::Index;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    view! {
      <h1>About</h1>
      <p>"Search a directory for images query exif-meta data"</p>
      <p>"Subdirectories are included."</p>
    }
}
