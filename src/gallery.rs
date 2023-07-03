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
      <div class="gallery-item bg-slate-600">

        <img

          class="object-fill h-48 w-96 radius"
          src=doc_link.de.get()
           />


          <h1>{doc_link.filename.get()}</h1>
          <p>
            {doc_link.description.get()}
          </p>

      </div>
    }
}
