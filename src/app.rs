use leptos::component;
use leptos::create_signal;
use leptos::provide_context;
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
use crate::settings::SettingsButton;
use crate::settings::SideBarState;

/// Provides routes and HTML template.
#[component]
#[must_use]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let sidebar_signal = create_signal::<SideBarState>(SideBarState::Close);
    provide_context(sidebar_signal);

    view! {
      <Html lang="en"/>
      <Meta name="description" content="Search images metadata."/>
      <Meta name="viewport" content="width=device-width, initial-scale=1"/>
      <Meta name="theme-color" content="#319197"/>
      <Link rel="icon" type_="image/svg+xml" href="/mag.svg"/>
      <Link rel="manifest" href="/manifest.json"/>
      <Title text="Photo Indexer"/>

      <Stylesheet id="leptos" href="/pkg/pi.css"/>
      <Body class="dark:bg-slate-950 dark:text-white font-roboto"/>
      <Router>

        <Routes>

          <Route
            path=""
            view=|| {
                view! {
                  // TODO work out wrapping for mobile nav is below the header
                  // for desktop nav is right justified.
                  <header class="flex gap-8 item-center justify-between m-none px-6 ">
                    <h1 class="font-light grow" style="font-size: calc(min(max(8vw, 1rem), 4rem));">
                      "SEARCH"
                    </h1>
                    <nav class="self-center text-[0.5vw]">
                    <A href="/about">"ABOUT"</A>
                    </nav>
                    <SettingsButton/>
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
                  <header class="border-none flex gap-8 item-center justify-between m-none px-6 ">
                    <h1 class="font-light grow" style="font-size: calc(min(max(8vw, 1rem), 4rem));">
                      "ABOUT"
                    </h1>
                    <nav class="self-center text-[0.5vw]">
                    <A href="/">"SEARCH"</A>
                    </nav>
                    <SettingsButton/>
                  </header>
                  <About/>
                }
            }
          />

        </Routes>

      </Router>
    }
}
