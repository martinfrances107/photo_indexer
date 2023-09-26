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
use leptos_router::A;

use crate::indexer::Index;

#[component]
pub fn Search() -> impl IntoView {
    let root = Path::new(&"../exif-samples");

    let (value, set_value) = create_signal(0);
    let (md_key_get, md_key_set) = create_signal::<Option<PathBuf>>(Some(
        PathBuf::from("../exif-samples/jpg/orientation/landscape_6.jpg"),
    ));
    let (index_get, _index_set) = create_signal(Index::new(root));
    let (search_query_get, search_query_set) =
        create_signal::<Vec<char>>(vec![]);

    let images = Signal::derive(move || {
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
    });

    // Use key to extract metadata from the md_store.
    let md_data = Signal::derive(move || match md_key_get.get() {
        Some(key) => index_get
            .get()
            .md_store
            .get(&key)
            .map(|fields| fields.to_vec()),
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
               String::from_iter(search_query_get.get())
             }
           />

         </form>

         <p class="mb-2">{ move || summary.get()}</p>
         <p>{move || value.get()}</p>
         <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
         <button on:click=move |_| set_value.set(0)>"Clear"</button>
         <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>

        <p id="key">{move ||{
          let pb: PathBuf = md_key_get.get().unwrap_or_default();
          pb.as_path().display().to_string();
        }
        }</p>
        <div class="flex">
         <Transition
           fallback =move || view!{ <p>"Loading"</p> }
         >

         {
          move || {
            match md_data.get(){
              Some(data) => {
                view!{
                  <div id="side-menu" class="inline-block">
                    <A href="">Close</A>
                    <div
                      class="
                      [&>*:nth-child(even)]:bg-gray-100
                      [&>*:nth-child(odd)]:bg-gray-300
                      overflow-hidden
                      w-[240px]
                      }}">
                      <For
                        each =move || data.clone()
                        key = move |field| {field.ifd_num}
                        view = move |field| {
                          view!{
                            <p>{ field.tag.to_string() }</p>
                            <p class="text-right" >{ field.display_value().to_string() }</p>
                          }
                        }
                      />
                      </div>
                  </div>
                }
              },
              None => {
                view!{<div id="side-menu-empty" class="w-0"></div>}
              }
            }

        }
      }

        <section class="
          flex
          flex-wrap
          gap-y-4
          rounded
          px-2 py-4
          justify-evenly
          dark:text-slate-950 bg-slate-600" >

        <For
          each=move || images.get()
          key=move |(i, _)| *i
          view=move |(_, (pb, _))| {
            // TODO find a better way than clone.
            let pb1 = pb.clone();
            let pb2 = pb.clone();
            let pb3 = pb.clone();
            let pb4 = pb.clone();
             view!{
                <div class="p-2 mb-4 rounded text-left" style="width:280px;">
                  <figure
                    class="bg-slate-100 rounded-t"
                    // on::click=move |e: u32| {
                    //   log!("{}", e);
                    // }
                    >
                     <img
                       width="274" height="160"
                       class="aspect-square mx-auto"
                       src={pb1.into_os_string().to_owned().into_string().unwrap()}
                     />
                     <figcaption>
                       {pb2.file_name().unwrap().to_str().unwrap().to_string()}
                       <p>
                         {
                            let ds = index_get.get().description_store.clone();
                            match ds.get(&pb3) {
                              Some(name) => view!{<p class="break-words w-full">{name}</p>},
                              None => view!{<p class="w-full">"No description"</p>}
                            }
                          }
                        <button on:click=move |_| {
                          log!("button clicked");
                          // console_log!("button clicked cl");
                          println!("on the server click metadata");
                          md_key_set.set(Some(pb4.to_owned()))
                         }>"Metadata"</button>

                       </p>
                     </figcaption>
                  </figure>
                 </div>
              }
          }
        />

       </section>
      </Transition>
      </div>
    </div>
    }
}
