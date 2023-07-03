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
use crate::homepage::HomePage;
use crate::indexer::Index;
use log::info;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/pi.css"/>

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
