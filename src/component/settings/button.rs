use leptos::component;
use leptos::use_context;
use leptos::view;
use leptos::IntoView;

/// App level Button
///
/// The hambuger icon is used to open a settings tray on the right
/// hand side.
#[component]
pub fn Button() -> impl IntoView {
    use leptos::ReadSignal;
    use leptos::SignalUpdate;
    use leptos::WriteSignal;

    use crate::component::settings::SideBarState;

    let (_, sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();

    view! {
      <button
        class="text-white"
        on:click=move |_| {
            sidebar_state_setter.update(|state| *state = state.toggle());
        }

        title="Open settings"
      >
        <svg
          class="stroke-neutral-400 hover:stroke-neutral-300"
          width="24px"
          height="24px"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M3 6.00092H21M3 12.0009H21M3 18.0009H21"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
        </svg>
      </button>
    }
}
