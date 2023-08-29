use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos_meta::provide_meta_context;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;
use tracing::info;

use crate::pages::about::About;
use crate::pages::search::Search;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/pi.css"/>
        <Title text="Photo Indexer"/>

        <Router>

          <Routes>
              <Route path="" view=|| view! { <Search /> }/>
              <Route path="/about" view=|| view! { <About /> }/>
          </Routes>

        </Router>
    }
}
