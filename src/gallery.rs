use leptos::component;
use leptos::view;
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::Scope;
use leptos::SignalGet;

use crate::indexer::DocLink;

/// Renders Image and its associated computed document.
#[component]
pub(crate) fn GalleryItem(cx: Scope, doc_link: DocLink) -> impl IntoView {
    view! { cx,
      <div class="grid-cols-2">

        <img src=doc_link.de.get() />

        <div>
          <h1>"EXIF Data"</h1>
          <p>
            {doc_link.doc}
          </p>
        </div>

      </div>
    }
}
