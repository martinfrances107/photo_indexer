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
#[component]
pub fn HomePage() -> impl IntoView {
    // let search_images = create_server_multi_action::<SearchImages>();

    let root = Path::new(&"../exif-samples");
    println!("signal starting");
    let start = Instant::now();
    let (index_get, _index_set) = create_signal(Index::new(&root));
    let duration = start.elapsed();
    println!(" signal done");
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let (search_query_get, search_query_set) = create_signal::<Vec<char>>(vec![]);

    // A derived signal
    // query and image are signal
    // so images changes when they are updated.
    let images = create_memo(move |_| {
        let query = search_query_get.get();
        println!("inside memo");
        let index = index_get.get();
        index.model.search_query(&query)
    });

    view! {
      //  <main class="bg-slate-950">
       <main class="">
         <Style>
           "body { font-weight: bold; }"
           // TODO move this to tailwind.config.js
           // or use value like ... grid-cols-[200px_minmax(900px,_1fr)_100px]
           ".gallery {
              grid-template-columns: repeat( auto-fill, minmax(320px, 1fr) );
            }"
         </Style>

         <section>
           <h1>"Photo Indexer"</h1>

           <form
            class="m-2"
            on:submit=|ev| ev.prevent_default()
           >

             <input
               on:change=move |ev|{
                 let val = event_target_value(&ev);
                 log!("pressed enter");
                 search_query_set.set(val.chars().collect());
               }
               type="text"
             />

           </form>

         </section>

         <section class="gallery rounded grid bg-slate-600" >
         <Transition
           fallback =move || view!{ <p>"Loading"</p> }
         >
         <p>"Go"</p>
         <p>{move || {search_query_get.get()}}</p>

          <For
            each=move || images.get()
            key=|r_img| r_img.1 as usize // rank
            view=move | ri| {
             view!{
               <h1>Next</h1>
               <p>rank {move || {ri.1}}</p>
               <p>value{move || {ri.clone().0.into_os_string().into_string().unwrap()}}</p>
              }
            }
          />

          </Transition>
          </section>
       </main>
    }
}
