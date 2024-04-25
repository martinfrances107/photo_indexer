use std::path::PathBuf;

use leptos::component;
use leptos::create_effect;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::create_signal;
// use leptos::create_slice;
use leptos::create_local_resource;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::logging::log;
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
#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;
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
            state.query = sq.clone();
            let entries_raw = state.index.model.search_query(&sq);
            // log!("add_query: {:#?}", entries_raw);
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
            // log!("get_query: {:#?}", state.query);
            // log!("get_query entries_raw {:#?}", state.entries);
            let entries = if state.query == vec!['h', 'd', 'r'] {
                // state.entries = entries_raw.into_iter().enumerate().collect();
                state.entries.clone()
                // vec![
                //     (1usize, (PathBuf::from("a".to_string()), 1_f32)),
                //     (2usize, (PathBuf::from("b".to_string()), 2_f32)),
                //     (3usize, (PathBuf::from("c".to_string()), 3_f32)),
                // ]
            } else {
                vec![
                    (1usize, (PathBuf::from("x".to_string()), 1_f32)),
                    (2usize, (PathBuf::from("y".to_string()), 2_f32)),
                    (3usize, (PathBuf::from("y".to_string()), 3_f32)),
                ]
            };
            entries
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

    let (search_query, _) = create_signal(String::from("orient"));

    // // Index is a derrived signal that depends on a semi static root_path.
    // let index = create_memo(move |_| {
    //     let path_str = root_path.get();
    //     let path = Path::new(&path_str);
    //     Index::new(path)
    // });

    // // `create_slice` lets us create a "lens" into the data
    // let (index, _set_index) = create_slice(
    //     // we take a slice *from* `state`
    //     state,
    //     // our getter returns a "slice" of the data
    //     |state| state.index.clone(),
    //     // our setter describes how to mutate that slice, given a new value
    //     |state, index| state.index = index,
    // );

    // Images depends on both signals index and search_query.
    // was using a single iterator here, but rust is lazily evaluates
    // iterators, and care must be taken within closures.
    // Dropped back to for loop here.
    // let images = Signal::derive(move || {
    //     // let sq = search_query.get().chars().collect::<Vec<char>>();
    //     let mut sq = vec![];
    //     for c in search_query.get().chars() {
    //         sq.push(c);
    //     }
    //     let index_g = index.get();
    //     log!("deriving images with sq ");
    //     // log!("{:#?}", &sq);
    //     // log!("{:#?}", &index_g.model);
    //     let partial_results = index_g.model.search_query(&sq);
    //     // log!("partial results");
    //     // log!("{:#?}", &partial_results);
    //     let mut results = vec![];
    //     for (i, partial) in partial_results.into_iter().enumerate() {
    //         results.push((i, partial));
    //     }
    //     results
    // });
    // let (images, images_set) = create_signal::<Vec<SRType>>(vec![]);
    let images = create_local_resource(
        move || search_query_action.version().get(),
        |_| get_query(),
    );

    let entries = Signal::derive(move || {
        match images.get() {
            Some(Ok(SearchResult { entries })) => {
                // log!("SD {:#?}", entries);
                let paths: Vec<_> = entries
                    .iter()
                    .map(|(_, (path, _rank))| path.display().to_string())
                    .collect();
                paths
            }
            _ => {
                vec![]
            }
        }
    });

    // create_effect(move |_| {
    //     log!("monitor: sq/images pair {:#?}", &search_query.get());
    //     log!("monitor: images {:#?}", &images.get());
    // });

    // let count_string = Signal::derive(move || {
    //     let len = images.get().len();
    //     match len {
    //         0 => String::from("No results found"),
    //         1 => String::from("1 image found"),
    //         l => {
    //             format!("{l} images found")
    //         }
    //     }
    // });

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
        // log!("pressed enter {:#?}", &query);

        search_query_action.dispatch(AddQuery { query });

        // if let Some(Ok(val)) = search_query_action.value().get() {
        //     log!("{:#?}", &val);
        // } else {
        //     log!("Failed to get return value");
        // }

        // images_set.set(return_value);
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

          // <Transition
          //   fallback =move || view!{ <p>"Loading count"</p> }
          // >
          //   <p>{ move || count_string.get()}</p>
          // </Transition>

          <p id="key">{ move || {
              let pb: PathBuf = md_key.get().unwrap_or_default();
              let key = pb.as_path().display().to_string();
              format!("key: {key}")
             }}
          </p>

          <Transition
            fallback =move || view!{ <p>"Loading search query"</p> }
          >
            <p >{ move || {
              let s = search_query.get();
              format!("search query: {s}")
                }}
            </p>
          </Transition>


            <div class="flex">
            // <p>Hello{
            //   entries.get()
            // }
            //   World
            // </p>
              // <Sidebar md/>
              <ImageGallery entries md_key_set />
            </div>

      </div>
    }
}
