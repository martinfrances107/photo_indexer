use leptos::component;
use leptos::prelude::*;

use leptos::view;
use leptos::IntoView;

/// Provides routes and HTML template.
#[must_use]
#[component]
pub fn App() -> impl IntoView {
    use crate::component::settings::button::Button as SettingsButton;
    use crate::component::settings::SideBarState;
    use crate::pages::about::About;
    use crate::pages::search::view::Search;
    use leptos_meta::provide_meta_context;
    use leptos_router::components::Route;
    use leptos_router::components::Router;
    use leptos_router::components::Routes;
    use leptos_router::path;
    use leptos_router::SsrMode;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let sidebar_signal = signal::<SideBarState>(SideBarState::Close);
    provide_context(sidebar_signal);

    view! {
      <Router>

        <Routes fallback=move || "Not found.">

          <Route
            path=path!("")
            view=|| {
              view! {
                // TODO work out wrapping for mobile nav is below the header
                // for desktop nav is right justified.
                <header class="flex gap-8 item-center justify-between m-none px-6 ">
                  <h1 class="font-light grow" style="font-size: clamp(1rem, 8vw, 4rem);">
                    "SEARCH"
                  </h1>
                  <nav class="self-center" style="font-size: clamp(.75rem, 4vw, 2rem);">
                    <a href="/about">"ABOUT"</a>
                  </nav>
                  <SettingsButton />
                </header>
                <Search />
              }
            }
          />

          <Route
            path=path!("/about")
            ssr=SsrMode::PartiallyBlocked
            view=|| {
              view! {
                // TODO work out wrapping for mobile nav is below the header
                // for desktop nav is right justified.
                <header class="border-none flex gap-8 item-center justify-between m-none px-6 ">
                  <h1 class="font-light grow" style="font-size: clamp(1rem, 8vw, 4rem);">
                    "ABOUT"
                  </h1>
                  <nav class="self-center" style="font-size: clamp(.75rem, 4vw, 2rem);">
                    <a href="/">"SEARCH"</a>
                  </nav>
                  <SettingsButton />
                </header>
                <About />
              }
            }
          />

        </Routes>

      </Router>
    }
}
