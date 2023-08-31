use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos::*;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    view! {
      <div class="dark:bg-slate-950 dark:text-white my-0 mx-auto font-roboto">
        <h1>ABOUT</h1>
        <p>"Search a directory for images query exif-meta data"</p>
        <p>"Subdirectories are included."</p>
      </div>
    }
}
