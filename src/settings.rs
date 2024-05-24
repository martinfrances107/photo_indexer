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

#[derive(Clone, Debug)]
pub enum SideBarState {
    Open,
    Close,
}

// A request by the client to to change the root directory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RootDirResult {
    root_dir: String,
}

#[server]
pub async fn add_root_dir(root_dir: String) -> Result<(), ServerFnError> {
    use std::path::PathBuf;
    use crate::indexer::Index;
    use crate::pages::GLOBAL_STATE;

    leptos::logging::log!("server: entry add_root_dir");

    let pb_root_dir = PathBuf::from(root_dir);
    // SANITIZE: Reject if not a valid directory
    // ALSO check access permissions.
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.index = Index::new(&pb_root_dir);
            state.entries = vec![];
            state.root_dir = pb_root_dir;
            Ok(())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_root_dir() -> Result<RootDirResult, ServerFnError> {
    use crate::pages::GLOBAL_STATE;

    let root_dir = match GLOBAL_STATE.lock() {
        Ok(state) => state.root_dir.clone(),
        Err(e) => {
            panic!("get_root_dir() - could not unlock {e}");
        }
    };

    Ok(RootDirResult {
        root_dir: root_dir.display().to_string(),
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

        title="OPEN METADATA"
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
    let root_dir_action = create_server_action::<AddRootDir>();

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let root_dir = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();

        root_dir_action.dispatch(AddRootDir { root_dir });
    };

    let (sidebar_state, _sidebar_state_setter) =
        use_context::<(ReadSignal<SideBarState>, WriteSignal<SideBarState>)>()
            .unwrap();
    view! {
      <div style:width=move || match sidebar_state.get() {
          SideBarState::Open => "100%",
          SideBarState::Close => "0",
      }>
        <h1>"Settings"</h1>
        <p>"Inside the new sidebar"</p>
        <form on:submit=on_submit class="dark:text-slate-700 px-6 py-2 text-center">
          <label class="hidden" for="root_dir">
            "Set the root directory"
          </label>
          <input
            id="root_dir"
            class="p-2"
            type="text"
            placeholder="root directory"
            node_ref=input_element
          />
          <input
            type="submit"
            title="Set the root directory for the indexer"
            value=" "
            class="bg-sky-700 cursor-grab rounded-r-lg p-2 hover:bg-sky-600 w-[3.5rem]"
          />
        </form>
      </div>
    }
}
