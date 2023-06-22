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
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

use crate::gallery::GalleryItem;
use crate::indexer::DocLink;
use crate::indexer::Index;
use log::info;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

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
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(cx, 0);

    let input_ref = create_node_ref::<Input>(cx);

    let root = Path::new(&"../exif-samples");

    let (index, _set_index) = create_signal::<Index>(cx, Index::new(cx, root));
    info!("indexing complete about to start server");

    // Initially apply no filter
    let filtered = move || index.with(|index| index.doc_links.to_vec());
    // dbg!(filtered());
    // let filtered = move || {
    //     return vec!["Mary", "had", "a", "little", "lamb"];
    // };

    // dbg!(filtered());

    view! { cx,
    <main>
      <section>
        <h1>"Photo Indexer"</h1>
        <input
        placeholder = "Search EXIF data"
        autofocus
        node_ref=input_ref
        />
      </section>
      <section>
         <For
           each=filtered
           key=|doc_link| doc_link.uuid()
           view=move |cx, doc_link| {
             view! {
               cx,
               <p>{"Hello"}</p>
               <GalleryItem doc_link/>
             }
           }
        />
      </section>
    </main>
      }
}
