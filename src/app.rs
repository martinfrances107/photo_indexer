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
use leptos_meta::provide_meta_context;
use leptos_meta::Style;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

use crate::gallery::GalleryItem;
use crate::indexer::Index;
use log::info;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Photo Indexer"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage /> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);

    let root = Path::new(&"../exif-samples");

    let (index, _set_index) = create_signal::<Index>(cx, Index::new(cx, root));
    info!("indexing complete about to start server");

    // Initially apply no filter
    let filtered = move || index.with(|index| index.doc_links.to_vec());

    view! { cx,
    <main class="bg-slate-900" >
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
