use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_meta::Link;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::component::settings::{
    button::Button as SettingsButton, SideBarState,
};
use crate::pages::about::About;
use crate::pages::search::view::Search;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    let sidebar_signal = signal::<SideBarState>(SideBarState::Close);
    provide_context(sidebar_signal);

    view! {
      <Link rel="icon" type_="image/svg+xml" href="/assets/mag.svg" />
      <Link rel="manifest" href="/assets/manifest.json" />
      <Title text="Search Image MetaTags"/>
      <Stylesheet id="leptos" href="/pkg/pi.css" />

      <Router>
        <main>
          <Routes fallback>
            <Route path=StaticSegment("") view=HomePage />
            <Route path=StaticSegment("/about") view=AboutPage />
          </Routes>
        </main>
      </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let sidebar_signal = signal::<SideBarState>(SideBarState::Close);
    provide_context(sidebar_signal);

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

#[component]
fn AboutPage() -> impl IntoView {
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
