use std::path::Path;
use std::path::PathBuf;

use leptos::component;
use leptos::create_node_ref;
use leptos::create_signal;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::logging::log;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::Transition;

use crate::image_gallery::ImageGallery;
use crate::indexer::Index;
use crate::sidebar::Sidebar;

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
    let pb = move || {
        Some(PathBuf::from(
            "../exif-samples/jpg/orientation/landscape_6.jpg",
        ))
    };

    let (md_key, md_key_set) = create_signal::<Option<PathBuf>>(pb());

    // TODO this should be under the control of a setting forms.
    let (root_path, _root_path_set) =
        create_signal(String::from("../exif-samples"));

    let (search_query, search_query_set) = create_signal(String::new());

    // Index is a derrived signal that depends on a semi static root_path.
    let index = Signal::derive(move || {
        let path_str = root_path.get();
        let path = Path::new(&path_str);
        Index::new(path)
    });

    // Images depends on both signals index and search_query.
    let images = Signal::derive(move || {
        let sq = search_query.get().chars().collect::<Vec<char>>();
        // log!("inside images query function with query {}", sq);
        index
            .get()
            .model
            .search_query(&sq)
            .iter()
            .enumerate()
            .map(|(i, (pb, f32))| (i, (pb.clone(), *f32)))
            .collect::<Vec<(usize, (PathBuf, f32))>>()
    });

    // Use key to extract metadata from the md_store.
    let md = Signal::derive(move || {
        md_key
            .get()
            .and_then(|key| index.get().md_store.get(&key).cloned())
    });

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();
        log!("pressed enter {:#?}", &value);
        search_query_set.set(value);
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

          <p>{ move || match images.get().len() {
             0 => String::from("No results found"),
             1 => String::from("1 image found"),
             l => {
                 format!("{l} images found")
             }
             }}
          </p>

          <p id="key">{ move || {
              let pb: PathBuf = md_key.get().unwrap_or_default();
              let key = pb.as_path().display().to_string();
              format!("key: {key}")
             }}
          </p>

          <p >{ move || {
            let s = search_query.get();
            format!("search query: {s}")
              }}
          </p>

          <Transition
            fallback =move || view!{ <p>"Loading"</p> }
          >
          <div class="flex">
            // <Sidebar md/>
            <ImageGallery images=images index md_key_set />
          </div>
         </Transition>
      </div>
    }
}
