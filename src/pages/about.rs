use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    use crate::component::settings::panel::Panel as SettingsPanel;
    view! {
      <div class="flex">
        <div class="my-0 mx-auto">
          <p>"Search a directory for images query exif-meta data"</p>
          <p>"Subdirectories are included."</p>
        </div>
        <SettingsPanel />
      </div>
    }
}
