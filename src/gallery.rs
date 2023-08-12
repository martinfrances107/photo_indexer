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
      <div class="gallery-item rounded bg-slate-600">

        /// Default tailwindcss css settings override width and height.
        /// aspect-ratio is required here!!!
        <img
          width="280" height="280"
          class="aspect-square"
          src=doc_link.de.get()
           />


          <h1 class="text-center">{doc_link.filename.get()}</h1>
          <p>
            {doc_link.description.get()}
          </p>

      </div>
    }
}
