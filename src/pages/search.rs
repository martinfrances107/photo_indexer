use std::path::Path;

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

    // A derived signal
    // query and image are signal
    // so images changes when they are updated.
    let images = move || {
        let query = search_query_get.get();
        println!("inside derived {query:#?} ");
        let index = index_get.get();
        index.model.search_query(&query)
    };

    let summary = move || {
        let images = images();
        if images.is_empty() {
            String::from("No results found")
        } else {
            format!("{} images found", images.len())
        }
    };

    view! {
      <div class="dark:bg-slate-950 dark:text-white my-0 mx-auto font-roboto">

         <Style>
           // TODO move this to tailwind.config.js
           // or use value like ... grid-cols-[200px_minmax(900px,_1fr)_100px]
           ".gallery {
              grid-template-columns: repeat( auto-fill, minmax(320px, 1fr) );
            }"
         </Style>

         <form class="px-6 py-2 dark:text-slate-950" >

           <input
             on:change=move |ev|{
               let val = event_target_value(&ev).chars().collect();
               log!("pressed enter {:#?}", &val);
               search_query_set.set(val);
             }
             type="text"
           />

         </form>

         <p>{ summary()}</p>

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
