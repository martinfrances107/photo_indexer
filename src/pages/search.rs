use std::path::PathBuf;

use leptos::component;
use leptos::create_effect;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::create_signal;
use leptos::create_local_resource;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::logging::log;
use leptos::server;
use leptos::view;
use leptos::IntoView;
use leptos::Signal;
use leptos::NodeRef;
use leptos::ServerFnError;
use leptos::SignalGet;
use leptos::Transition;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;
use crate::image_gallery::ImageGallery;
use crate::sidebar::Sidebar;

pub type SRType = (usize, (PathBuf, f32));

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SearchResult {
    pub entries: Vec<SRType>,
}

#[server]
pub async fn add_query(query: String) -> Result<SearchResult, ServerFnError> {
    log!("serve: entry search_query");
    let sq = query.chars().collect::<Vec<char>>();
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.query = sq;
            let entries_raw = state.index.model.search_query(&state.query);
            state.entries = entries_raw.into_iter().enumerate().collect();
            Ok(SearchResult {
                entries: state.entries.clone(),
            })
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

// TODO wiered leptos default naming convention
// get_query get the result of the last query
// ie get a list of images.
#[server]
pub async fn get_query() -> Result<SearchResult, ServerFnError> {
    let entries = match GLOBAL_STATE.lock() {
        Ok(state) => {
                state.entries.clone()
        }
        Err(e) => {
            panic!("get_query - could not unlock {e}");
        }
    };

    Ok(SearchResult { entries })
}

/// A settings form calls root_path_set ( Todo: it hard coded for now ).
///
/// This triggers Index to update,
/// Index is a async process - which is from here onwards is
/// is considered semi static.
///
/// When the user enters a search terms
/// The Index is queried and a set of images produced.
///
#[component]
pub fn Search() -> impl IntoView {
    let search_query_action = create_server_action::<AddQuery>();

    let pb = move || {
        Some(PathBuf::from(
            "../exif-samples/jpg/orientation/landscape_6.jpg",
        ))
    };

    let (md_key, md_key_set) = create_signal::<Option<PathBuf>>(pb());

    // TODO this should be under the control of a setting forms.
    // let (root_path, _root_path_set) =
    //     create_signal(String::from("../exif-samples"));

    let (search_query, _) =
        create_signal(String::from("orient"));

    let images = create_local_resource(
        move || search_query_action.version().get(),
        |_| get_query(),
    );

    let entries = Signal::derive(move || {
      match images.get() {
        Some(Ok(SearchResult{entries})) => {
           let paths: Vec<_>  = entries.iter().map(|(_, (path, _rank))|  {
            path.display().to_string()
          }).collect();
          paths

        },
        _ => {
          vec![]
        }
      }
    });

    // create_effect(move |_| {
    //     log!("monitor: sq/images pair {:#?}", &search_query.get());
    //     log!("monitor: images {:#?}", &images.get());
    // });

    let count_string = Signal::derive(move || {
        let len = entries.get().len();
        match len {
            0 => String::from("No results found"),
            1 => String::from("1 image found"),
            l => {
                format!("{l} images found")
            }
        }
    });

    // create_effect(move |_| {
    //     log!("monitor count_string() {:#?}", &count_string.get());
    // });

    // Use key to extract metadata from the md_store.
    // let md = Signal::derive(move || {
    //     md_key
    //         .get()
    //         .and_then(|key| index.get().md_store.get(&key).cloned())
    // });

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let query = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();

        search_query_action.dispatch(AddQuery { query });

    };

    view! {
        <div class="my-0 mx-auto">

          <form on:submit=on_submit class="dark:text-slate-950 px-6 py-2 text-center">
            <label class="hidden" for="search">Search</label>
            <input
              id="search"
              class="p-2"
              type="text"
              placeholder="Search EXIF data"
              node_ref = input_element
            />
            <input type="submit" value="submit"/>
          </form>

          <Transition
            fallback =move || view!{ <p>"Loading count"</p> }
          >
            <p>{ move || count_string.get()}</p>
          </Transition>

          <div class="flex">
            // <Sidebar md/>
            <ImageGallery entries md_key_set />
          </div>

      </div>
    }
}
