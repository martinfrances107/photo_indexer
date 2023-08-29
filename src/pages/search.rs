use std::path::Path;
use std::time::Instant;

use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::*;
use leptos_meta::Style;
use tracing::info;

use crate::indexer::Index;

/// Holds main search bar and results.
///
///

#[component]
pub fn Search() -> impl IntoView {
    // let search_images = create_server_multi_action::<SearchImages>();

    let root = Path::new(&"../exif-samples");
    // println!("signal starting");
    // let start = Instant::now();
    let (index_get, _index_set) = create_signal(Index::new(&root));
    // let duration = start.elapsed();
    // println!(" signal done");
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    let (search_query_get, search_query_set) = create_signal::<Vec<char>>(vec![]);

    // A derived signal
    // query and image are signal
    // so images changes when they are updated.
    let images = move || {
        let query = search_query_get.get();
        println!("inside derived {query:#?} ");
        let index = index_get.get();
        // let query = "sky".chars().collect::<Vec<char>>();
        index.model.search_query(&query)
    };

    let n_found = move || format!("{} images found", images().len());

    view! {
      <div class="dark:bg-slate-950 dark:text-white my-0 mx-auto font-roboto">

         <Style>
           "body { font-weight: bold; }"
           // TODO move this to tailwind.config.js
           // or use value like ... grid-cols-[200px_minmax(900px,_1fr)_100px]
           ".gallery {
              grid-template-columns: repeat( auto-fill, minmax(320px, 1fr) );
            }"
         </Style>

         <h1 class="p-6 font-light text-8xl">"Photo Indexer"</h1>

         <form
           class="px-6 py-2 dark:text-slate-950"
          //  on:submit=|ev| ev.prevent_default()
         >

           <input
             on:change=move |ev|{
               let val = event_target_value(&ev).chars().collect();
               log!("pressed enter {:#?}", &val);
               search_query_set.set(val);
             }
             type="text"
           />

         </form>
         <p>{move || n_found()}</p>

         <section class="gallery rounded grid bg-slate-600" >
         <Transition
           fallback =move || view!{ <p>"Loading"</p> }
         >

          <For
            each=move || images()
            key=|r_img| r_img.1 as usize // rank
            view=move |ri| {
              let src = ri.clone().0.into_os_string().into_string().unwrap();
               view!{
                 <div class="gallery-item rounded">
                 <figure>
                   <img
                     width="280" height="280"
                     class="aspect-square"
                     src={src}
                   />
                   <figcaption class="text-center">
                     // {doc_link.filename.get()}
                   </figcaption>
                 </figure>
                 <p>
                   // {doc_link.description.get()}
                 </p>

                 </div>
                }
            }
          />

          </Transition>
          </section>
       </div>
    }
}
