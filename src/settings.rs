use leptos::component;
use leptos::use_context;
use leptos::view;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos::SignalUpdate;
use leptos::WriteSignal;

use crate::file_lister::lister::Lister as FileLister;

#[derive(Clone, Debug)]
pub enum SideBarState {
    Open,
    Close,
}

impl SideBarState {
    // Used to dynamically control class the settings bar.
    const fn is_hidden(&self) -> bool {
        match self {
            Self::Open => false,
            Self::Close => true,
        }
    }

    const fn toggle(&self) -> Self {
        match self {
            Self::Open => Self::Close,
            Self::Close => Self::Open,
        }
    }
}

/// App level Button
///
/// The hambuger icon is used to open a settings tray on the right
/// hand side.
#[component]
pub fn SettingsButton() -> impl IntoView {
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
        <img src="/hamburger.svg"/>
      </button>
    }
}

/// Right handside side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn SettingsPannel() -> impl IntoView {
    let (sidebar_state, _sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();
    view! {
      <div class:hidden=move || sidebar_state.get().is_hidden()>
        <h1>"Settings"</h1>
        <FileLister/>
      </div>
    }
}
