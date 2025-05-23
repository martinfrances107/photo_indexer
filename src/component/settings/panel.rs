use leptos::prelude::component;
use leptos::prelude::use_context;
use leptos::prelude::view;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoMaybeErased;
use leptos::prelude::IntoView;
use leptos::prelude::ReadSignal;
use leptos::prelude::WriteSignal;

/// Right handmade side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn Panel() -> impl IntoView {
    use crate::component::file_lister::view::Lister as FileLister;
    use crate::component::settings::SideBarState;

    let (sidebar_state, _sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();
    view! {
      <div
        class:hidden=move || sidebar_state.get().is_hidden()
        class="dark:bg-slate-800 p-2 max-w-80 mr-2 rounded shadow-inner shadow-slate-700"
      >
        <h1 class="mb-2">"Settings Panel"</h1>
        <FileLister />
      </div>
    }
}
