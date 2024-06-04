use leptos::component;
use leptos::create_local_resource;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::server;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::ServerFnError;
use leptos::Signal;
use leptos::SignalGet;
use leptos::Transition;
use serde::Deserialize;
use serde::Serialize;

use crate::image_gallery::ImageGallery;
use crate::settings::pannel::Pannel as SettingsPannel;

#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;

// Search Result Element
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SRElem {
    pub description: String,
    // key: Ensures images in the gallery have a
    // unique id, otherwise images are not correctly refreshed.
    pub key: usize,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchResult {
    pub entries: Vec<SRElem>,
    // counter that increments every for query.
    pub version: usize,
}

// Cantor Pairing.
//
// A hash function for two integers
//
// <https://en.wikipedia.org/wiki/Pairing_function>
#[cfg(feature = "ssr")]
#[inline]
fn cantor_pair(k1: usize, k2: usize) -> usize {
    (k1 + k2) * (k1 + k2 + 1) / 2 + k2
}

#[server]
pub async fn add_query(query: String) -> Result<(), ServerFnError> {
    leptos::logging::log!("server: entry search_query");

    let sq = query.chars().collect::<Vec<char>>();

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.query = sq;
            Ok(())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

// TODO wierd leptos default naming convention
// get_query get the result of the last query
// ie get a list of images.
#[server]
pub async fn get_query(version: usize) -> Result<SearchResult, ServerFnError> {

    use crate::pages::IMAGE_PREFIX;

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            let entries_raw = state.index.model.search_query(&state.query);
            state.entries = entries_raw
                .iter()
                .enumerate()
                .map(|(i, path_rank)| {
                    let key = cantor_pair(version, i);
                    let description = match state
                        .index
                        .description_store
                        .get(&path_rank.0.display().to_string())
                    {
                        Some(description) => description.to_string(),
                        None => String::default(),
                    };

                    // Construct url from filename
                    let url = match path_rank
                        .0
                        .strip_prefix(state.selected_dir.clone())
                    {
                        Ok(filename) => {
                            format!(
                                "{IMAGE_PREFIX}{}",
                                filename.display().to_string()
                            )
                        }
                        Err(_) => String::default(),
                    };

                    SRElem {
                        description,
                        key,
                        url,
                    }
                })
                .collect();
            Ok(SearchResult {
                entries: state.entries.clone(),
                version,
            })
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
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

    let images = create_local_resource(
        move || search_query_action.version().get(),
        get_query,
    );

    let entries = Signal::derive(move || match images.get() {
        Some(Ok(SearchResult { entries, .. })) => entries,
        _ => {
            vec![]
        }
    });

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

        <form on:submit=on_submit class="dark:text-slate-700 px-6 py-2 text-center">
          <label class="hidden" for="search">
            Search
          </label>
          <input
            id="search"
            class="p-2"
            type="text"
            placeholder="Search EXIF data"
            node_ref=input_element
          />
          <input
            type="submit"
            title="Search"
            value=" "
            class="bg-sky-700 cursor-grab rounded-r-lg p-2 hover:bg-sky-600 w-[3.5rem]"
          />
        </form>

        <Transition fallback=|| view! { <p>"Loading count"</p> }>
          <p class="m-6 text-right">{move || count_string.get()}</p>
        </Transition>

        <div class="flex">
          <ImageGallery entries/>
          <SettingsPannel/>
        </div>

      </div>
    }
}
