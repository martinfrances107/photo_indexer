use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::IntoView;
use leptos::Scope;
use leptos::SignalUpdate;

use leptos_meta::provide_meta_context;
use leptos_meta::Stylesheet;
use leptos_meta::Title;

use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

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
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Photo Indexer"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
