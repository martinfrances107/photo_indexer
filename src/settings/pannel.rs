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
    use crate::file_lister::lister::Lister as FileLister;
    use crate::settings::SideBarState;

    let (sidebar_state, _sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();
    view! {
      <div class:hidden=move || sidebar_state.get().is_hidden()>
        <h1 class="mb-2">"Settings Pannel"</h1>
        <FileLister/>
      </div>
    }
}
