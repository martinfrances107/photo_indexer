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
      <div class="gallery-item bg-slate-900">

        <img
          width="175ps" height="175px"
          class="radius"
          src=doc_link.de.get()
           />


          <h1>{doc_link.filename}</h1>
          <p>
            {doc_link.description}
          </p>

      </div>
    }
}
