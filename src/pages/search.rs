use std::path::Path;
use std::path::PathBuf;

use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::*;
use leptos_meta::Style;
use tracing::info;

use crate::indexer::Index;

#[component]
pub fn Search() -> impl IntoView {
    let root = Path::new(&"../exif-samples");

    let (index_get, _index_set) = create_signal(Index::new(root));
    let (search_query_get, search_query_set) = create_signal::<Vec<char>>(vec![]);

    let images = move || {
        let query = search_query_get.get();
        println!("inside derived {query:#?} ");
        let index = index_get.get();
        index
            .model
            .search_query(&query)
            .iter()
            .enumerate()
            .map(|(i, (pb, f32))| (i, (pb.clone(), *f32)))
            .collect::<Vec<(usize, (PathBuf, f32))>>()
    };

    let summary = move || {
        let images = images();

        match images.len() {
            0 => String::from("No results found"),
            1 => String::from("1 image found"),
            _ => {
                format!("{} images found", images.len())
            }
        }
    };

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
           />

         </form>

         <p class="mb-2">{ move || summary()}</p>

         <section class="flex flex-wrap gap-y-4 rounded px-2 py-4 justify-evenly dark:text-slate-950 bg-slate-600" >
           <Transition
             fallback =move || view!{ <p>"Loading"</p> }
           >

            <For
              each=move || images()
              key=move |(i, _)| *i
              view=move |(_, (pb, _r))| {
                 view!{

                    <figure class="bg-slate-200 p-2 rounded text-left">
                      <img
                        width="420" height="420"
                        class="aspect-square mx-auto"
                        src={pb.clone().into_os_string().into_string().unwrap()}
                      />
                      <figcaption class="mb-4">
                        {pb.file_name().unwrap().to_str().unwrap().to_string()}
                      </figcaption>
                     </figure>

                  }
              }
            />

          </Transition>
        </section>
       </div>
    }
}
