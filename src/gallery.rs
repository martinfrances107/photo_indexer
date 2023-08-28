use leptos::component;
use leptos::view;
use leptos::IntoAttribute;
use leptos::IntoView;

/// Renders Image and its associated computed document.
#[component]
pub(crate) fn GalleryItem() -> impl IntoView {
    view! {
      <div class="gallery-item rounded">
        <figure>
          <img
            width="280" height="280"
            class="aspect-square"
            // src=doc_link.de.get()
          />
          <figcaption class="text-center">
            // {doc_link.filename.get()}
          </figcaption>
        </figure>
        <p>
          // {doc_link.description.get()}
        </p>

      </div>
    }
}
