use leptos::component;
use leptos::create_local_resource;
use leptos::create_server_action;
use leptos::server;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::ServerFnError;
use leptos::Signal;
use leptos::SignalGet;

use serde::Deserialize;
use serde::Serialize;

use crate::pages::IMAGE_PREFIX;

// A request by the client to to change the root directory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListUrlResult {
    list_url: String,
    listed_urls: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SelectedUrlResult {
    url: String,
}

#[server]
pub async fn add_list_url(list_url: String) -> Result<(), ServerFnError> {
    use std::path::PathBuf;

    use crate::pages::GLOBAL_STATE;

    leptos::logging::log!("server: entry add_root_url");
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            // SANITIZATION: Convert URL into dirname.
            // Must check the dirname is valid and has permissions.
            let selected_dir = PathBuf::from(list_url.clone());

            let listed_urls =
                vec![String::from("a"), String::from("b"), String::from("c")];

            state.selected_dir = selected_dir;
            state.list_url = list_url;
            state.listed_urls = listed_urls;
            Ok(())
        }
        Err(e) => {
            panic!("add_list_dir - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_list_url() -> Result<ListUrlResult, ServerFnError> {
    use crate::pages::GLOBAL_STATE;
    let (list_url, listed_urls) = match GLOBAL_STATE.lock() {
        Ok(state) => (state.list_url.clone(), state.listed_urls.clone()),
        Err(e) => {
            panic!("get_list_dir() - could not unlock {e}");
        }
    };

    Ok(ListUrlResult {
        list_url,
        listed_urls,
    })
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

/// Right handside side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn FileLister() -> impl IntoView {
    let list_url_action = create_server_action::<AddListUrl>();

    let list_urls_resource = create_local_resource(
        move || list_url_action.version().get(),
        |_| get_list_url(),
    );

    let list_url_result =
        Signal::derive(move || match list_urls_resource.get() {
            Some(Ok(list_urls)) => list_urls,
            _ => ListUrlResult {
                list_url: IMAGE_PREFIX.into(),
                listed_urls: vec!["x".into(), "y".into(), "z".into()],
            },
        });

    // let root_dir_action = create_server_action::<AddSelectedUrl>();

    //   let on_submit = move |ev: SubmitEvent| {
    //     ev.prevent_default();

    //     let url = input_element
    //         .get()
    //         .expect("<input> should be mounted.")
    //         .value();

    //     root_dir_action.dispatch(AddSelectedUrl { url });
    // };

    view! {
      <div>
        <h2>File Lister</h2>
        <p>list_url_results.list_url</p>
        <ol>
          <For
            each=move || {
                list_url_result.get().listed_urls.iter().cloned().enumerate().collect::<Vec<_>>()
            }

            key=|(i, _)| { *i }
            let:data
          >
            <li>{data.1}</li>
          </For>
        </ol>
      </div>
    }
}
