use leptos::component;
use leptos::view;
use leptos::IntoView;

/// Holds main search bar and results.
#[component]
pub fn About() -> impl IntoView {
    use crate::component::settings::pannel::Pannel as SettingsPannel;
    view! {
      <div class="flex">
        <div class="my-0 mx-auto">
          <p>"Search a directory for images query exif-meta data"</p>
          <p>"Subdirectories are included."</p>
        </div>
        <SettingsPannel />
      </div>
    }
}
