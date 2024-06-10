use leptos::component;
use leptos::use_context;
use leptos::view;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::WriteSignal;

/// Right handside side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn Pannel() -> impl IntoView {
    use crate::component::file_lister::view::Lister as FileLister;
    use crate::component::settings::SideBarState;

    let (sidebar_state, _sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();
    view! {
      <div
        class:hidden=move || sidebar_state.get().is_hidden()
        class="dark:bg-slate-800 p-2 max-w-80 mr-2 rounded"
      >
        <h1 class="mb-2">"Settings Pannel"</h1>
        <FileLister/>
      </div>
    }
}
