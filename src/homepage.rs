use std::path::Path;

use leptos::component;
use leptos::create_node_ref;
use leptos::create_signal;
use leptos::html::Input;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::Scope;
use leptos::SignalWith;
use leptos_meta::Style;

use crate::gallery::GalleryItem;
use crate::indexer::Index;
use log::info;

/// Holds main search bar and results.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);

    let root = Path::new(&"../exif-samples");

    let (index, _set_index) = create_signal::<Index>(cx, Index::new(cx, root));
    info!("indexing complete about to start server");

    // Initially apply no filter
    let filtered = move || index.with(|index| index.doc_links.to_vec());

    view! { cx,
    <main >
      <Style>
        "body { font-weight: bold; }"
        ".gallery {
            display: grid;
            grid-template-columns: repeat( auto-fill, minmax(250px, 1fr) );
            background-color: #fff;
          }"

      </Style>

      <section>
        <h1>"Photo Indexer"</h1>
        <input
        placeholder = "Search EXIF data"
        autofocus
        node_ref=input_ref
        />
      </section>
      <section class="gallery">
         <For
           each=filtered
           key=|doc_link| doc_link.uuid()
           view=move |cx, doc_link| {
             view! {
               cx,
               <GalleryItem doc_link/>
             }
           }
        />
      </section>
    </main>
      }
}
