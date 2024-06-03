use leptos::component;
use leptos::view;
use leptos::IntoView;

use crate::settings::SettingsPannel;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    view! {
      <div class="flex">
        <div class="my-0 mx-auto">
          <p>"Search a directory for images query exif-meta data"</p>
          <p>"Subdirectories are included."</p>
        </div>
        <SettingsPannel/>
      </div>
    }
}
