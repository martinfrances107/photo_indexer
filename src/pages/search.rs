use std::path::Path;
use std::path::PathBuf;

use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::*;

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

    let md_store = move || index_get.get().md_store;

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
             placeholder="Search EXIF data"
             prop:value = {move ||
               String::from_iter(search_query_get.get())
             }
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
              view=move |(_, (pb, _))| {
                 view!{
                    <div class="p-2 mb-4 rounded text-left" style="width:430px;">
                      <figure class="bg-slate-100 rounded-t">
                         <img
                           width="420" height="420"
                           class="aspect-square mx-auto"
                           src={pb.clone().into_os_string().into_string().unwrap()}
                         />
                         <figcaption>
                           {pb.file_name().unwrap().to_str().unwrap().to_string()}
                           <p>

                               {
                                let ds = index_get.get().description_store.clone();
                                match ds.get(&pb) {
                                  Some(name) => view!{<p class="break-words w-full">{name}</p>},
                                  None => view!{<p class="w-full">"No description"</p>}
                                }
                              }

                           </p>
                         </figcaption>
                      </figure>
                      <details class="bg-slate-100 rounded-b">
                        <summary>
                          MetaData
                        </summary>
                        <div class="[&>*:nth-child(even)]:bg-gray-100 [&>*:nth-child(odd)]:bg-gray-300">
                          <For
                          // each = move || { 1i32..10i32}
                          each =move || { md_store().get(&pb).expect("failed to extract fields from md_store").clone()}
                          // each = move || { index_get.get().md_store}
                          key = move |field| {field.ifd_num}
                          view = move |field| { view!{
                            <p>{ field.tag.to_string() }</p>
                            <p class="text-right" >{ field.display_value().to_string() }</p>
                          }}
                          />
                        </div>
                      </details>
                     </div>
                  }
              }
            />

          </Transition>
        </section>
       </div>
    }
}
