use std::path::Path;
use std::path::PathBuf;

use leptos::component;
use leptos::create_memo;
use leptos::create_signal;
use leptos::event_target_value;
use leptos::view;
use leptos::IntoView;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::Transition;

use crate::image_gallery::ImageGallery;
use crate::indexer::Index;
use crate::sidebar::Sidebar;

#[component]
pub fn Search() -> impl IntoView {
    let pb = move || {
        Some(PathBuf::from(
            "../exif-samples/jpg/orientation/landscape_6.jpg",
        ))
    };
    let (md_key, md_key_set) = create_signal::<Option<PathBuf>>(pb());
    let (index, _index_set) =
        create_signal(Index::new(Path::new(&"../exif-samples")));
    let (search_query, search_query_set) =
        create_signal::<String>(String::new());

    let images = create_memo(move |_| {
        // println!("inside derived {query:#?} ");
        index
            .get()
            .model
            // .search_query(&search_query.get().chars().collect::<Vec<char>>())
            .search_query(&"hello".chars().collect::<Vec<char>>())
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

    let summary = Signal::derive(move || {
        let images = images.get();

        match images.len() {
            0 => String::from("No results found"),
            1 => String::from("1 image found"),
            l => {
                format!("{l} images found")
            }
        }
    });

    let key = Signal::derive(move || {
        let pb: PathBuf = md_key.get().unwrap_or_default();
        let key = pb.as_path().display().to_string();
        format!("key: {key}");
    });

    view! {
      <div class="my-0 mx-auto">

         <form class="dark:text-slate-950 px-6 py-2 text-center">

           <label class="hidden" for="search">Search</label>
           <input
             id="search"
             class="p-2"
             on:change=move |ev|{
               let val = event_target_value(&ev).chars().collect();
              //  log!("pressed enter {:#?}", &val);
               search_query_set.set(val);
             }
             type="text"
             placeholder="Search EXIF data"

             prop:value = search_query
           />

         </form>

         <p>{summary.get()}</p>
         <p id="key">{key.get()} </p>
        <Transition
          fallback =move || view!{ <p>"Loading"</p> }
        >
        <div class="flex">

          <Sidebar md/>
          <ImageGallery images=images.into() index md_key_set />

       </div>
       </Transition>
    </div>
    }
}
