use leptos::component;
use leptos::view;
use leptos::IntoView;
use leptos_meta::provide_meta_context;
use leptos_meta::Body;
use leptos_meta::Html;
use leptos_meta::Link;
use leptos_meta::Meta;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;
use leptos_router::A;

use crate::pages::about::About;
use crate::pages::search::Search;

/// Provides routes and HTML template.
#[component]
#[must_use]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
      <Html lang="en"/>
      <Meta name="description" content="Search images metadata."/>
      <Meta name="viewport" content="width=device-width, initial-scale=1"/>
      <Meta name="theme-color" content="#319197"/>
      <Link rel="icon" type_="image/svg+xml" href="/mag.svg"/>
      <Link rel="manifest" href="/manifest.json"/>

      <Stylesheet id="leptos" href="/pkg/pi.css"/>
      <Title text="Photo Indexer"/>
      <Body class="dark:bg-slate-950 dark:text-white font-roboto"/>
      <Router>

        <Routes>

          <Route
            path=""
            view=|| {
                view! {
                  // TODO work out wrapping for mobile nav is below the header
                  // for desktop nav is right justified.
                  <header class="flex item-center justify-between px-6 m-none">
                    <h1 class="font-light" style="font-size: calc(min(max(8vw, 1rem), 4rem));">"SEARCH"</h1>
                    <nav class="self-center">
                      <A href="/about">
                        <span class="text-[0.5vw]">"ABOUT"</span>
                      </A>
                    </nav>
                  </header>
                  <Search/>
                }
            }
          />

          <Route
            path="/about"
            view=|| {
                view! {
                  // TODO work out wrapping for mobile nav is below the header
                  // for desktop nav is right justified.
                  <header class="border-none flex item-center justify-between px-6 m-none">
                    <h1 class="font-light" style="font-size: calc(min(max(8vw, 1rem), 4rem));">"ABOUT"</h1>
                    <nav class="self-center">
                      <A href="/">
                        <span class="text-[0.5vw]">"SEARCH"</span>
                      </A>
                    </nav>
                  </header>
                  <About/>
                }
            }
          />

        </Routes>

      </Router>
    }
}
