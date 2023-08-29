use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::*;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    view! {
      <h1>About</h1>
      <p>"Search a directory for images query exif-meta data"</p>
      <p>"Subdirectories are included."</p>
    }
}
