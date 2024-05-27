use leptos::component;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::logging::log;
use leptos::server;
use leptos::use_context;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::ReadSignal;
use leptos::ServerFnError;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::WriteSignal;

use serde::Deserialize;
use serde::Serialize;

use crate::file_lister::FileLister;

#[derive(Clone, Debug)]
pub enum SideBarState {
    Open,
    Close,
}

#[derive(Deserialize, Serialize)]
pub struct SelectedUrlResult {
    url: String,
}

impl SideBarState {
    // Used to dynamically control class the settings bar.
    const fn is_hidden(&self) -> bool {
        match self {
            Self::Open => false,
            Self::Close => true,
        }
    }
}

#[server]
pub async fn add_selected_url(url: String) -> Result<(), ServerFnError> {
    use std::path::PathBuf;

    use crate::indexer::Index;
    use crate::pages::GLOBAL_STATE;

    leptos::logging::log!("server: entry add_root_dir");
    // SANITIZE: Reject if not a valid directory
    // ALSO check access permissions.
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            // SANITIZATION
            // Reject urls without a prefix "/images"
            // Reject invalid directory names ( within the container directory ).
            let selected_dir = match PathBuf::from(url).strip_prefix("images") {
                Ok(filename_suffix) => {
                    state.container_dir.join(filename_suffix)
                }
                Err(_) => {
                    // malformed input.
                    return Err(ServerFnError::Args(String::from(
                        "URL must be prefixed with 'images/'",
                    )));
                }
            };

            if selected_dir.is_dir() {
                // reject suspicious input.
                return Err(ServerFnError::Args(String::from(
                    "rejecting selected url",
                )));
            }

            state.index =
                Index::new(selected_dir.clone(), state.container_dir.clone());
            state.entries = vec![];
            state.selected_dir = selected_dir;
            Ok(())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_selected_dir() -> Result<SelectedUrlResult, ServerFnError> {
    use crate::pages::GLOBAL_STATE;

    let selected_dir = match GLOBAL_STATE.lock() {
        Ok(state) => state.selected_dir.clone(),
        Err(e) => {
            panic!("get_root_dir() - could not unlock {e}");
        }
    };

    Ok(SelectedUrlResult {
        url: selected_dir.display().to_string(),
    })
}

/// App level Button
///
/// The hambuger icon is used to open a settings tray on the right
/// hand side.
#[component]
pub fn SettingsButton() -> impl IntoView {
    let (sidebar_state, sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();

    view! {
      <button
        class="text-white"
        on:click=move |_| {
            let new_state = match sidebar_state.get() {
                SideBarState::Open => {
                    log!("clicked: closing");
                    SideBarState::Close
                }
                SideBarState::Close => {
                    log!("clicked: opening");
                    SideBarState::Open
                }
            };
            sidebar_state_setter.set(new_state);
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
    let root_dir_action = create_server_action::<AddSelectedUrl>();

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let url = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();

        root_dir_action.dispatch(AddSelectedUrl { url });
    };

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
