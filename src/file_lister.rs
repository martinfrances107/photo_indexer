use leptos::component;
use leptos::create_effect;
use leptos::create_local_resource;
use leptos::create_server_action;
use leptos::logging::log;
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

    create_effect(move |_| {
        log!("{:#?}", list_url_result.get());
    });

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
