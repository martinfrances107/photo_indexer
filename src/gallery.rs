use leptos::component;
use leptos::view;
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::Scope;

use crate::indexer::DocLink;

/// Renders Image and its associated computed document.
#[component]
// pub(crate) fn GalleryItem(cx: Scope) -> impl IntoView {
pub(crate) fn GalleryItem(cx: Scope, doclink: DocLink) -> impl IntoView {
    view! { cx,
      <div class="grid-cols-2">

        <img src=format!("{:?}", doclink.de.path()) />

        <div>
          <h1>"EXIF Data"</h1>
          <p>
            {doclink.doc}
          </p>
        </div>

      </div>
    }
}
