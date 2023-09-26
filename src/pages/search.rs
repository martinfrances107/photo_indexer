use std::path::Path;
use std::path::PathBuf;

use leptos::component;
use leptos::create_signal;
use leptos::event_target_value;
use leptos::logging::log;
use leptos::view;
use leptos::For;
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::SignalUpdate;
use leptos::Transition;

use crate::image_gallery::ImageGallery;
use crate::indexer::Index;
use crate::sidebar::Sidebar;

#[component]
pub fn Search() -> impl IntoView {
    let root = Path::new(&"../exif-samples");

    let (value, set_value) = create_signal(0);
    let (md_key, md_key_set) = create_signal::<Option<PathBuf>>(Some(
        PathBuf::from("../exif-samples/jpg/orientation/landscape_6.jpg"),
    ));
    let (index, _index_set) = create_signal(Index::new(root));
    let (search_query, search_query_set) = create_signal::<Vec<char>>(vec![]);

    let images = Signal::derive(move || {
        let query = search_query.get();
        println!("inside derived {query:#?} ");
        let index = index.get();
        index
            .model
            .search_query(&query)
            .iter()
            .enumerate()
            .map(|(i, (pb, f32))| (i, (pb.clone(), *f32)))
            .collect::<Vec<(usize, (PathBuf, f32))>>()
    });

    // Use key to extract metadata from the md_store.
    let md = Signal::derive(move || match md_key.get() {
        Some(key) => {
            index.get().md_store.get(&key).map(|fields| fields.to_vec())
        }
        None => None,
    });

    let summary = Signal::derive(move || {
        let images = images.get();

        match images.len() {
            0 => String::from("No results found"),
            1 => String::from("1 image found"),
            _ => {
                format!("{} images found", images.len())
            }
        }
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
             prop:value = {move ||
               String::from_iter(search_query.get())
             }
           />

         </form>

         <p class="mb-2">{ move || summary.get()}</p>
         <p>{move || value.get()}</p>
         <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
         <button on:click=move |_| set_value.set(0)>"Clear"</button>
         <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>

        <p id="key">{move ||{
          let pb: PathBuf = md_key.get().unwrap_or_default();
          pb.as_path().display().to_string();
        }
        }</p>
        <Transition
          fallback =move || view!{ <p>"Loading"</p> }
        >
        <div class="flex">

        <Sidebar md/>
        <ImageGallery images index md_key_set />

       </div>
       </Transition>
    </div>
    }
}
