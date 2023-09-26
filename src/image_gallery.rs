use std::path::PathBuf;

use crate::indexer::Index;

use leptos::component;
use leptos::logging::log;
use leptos::view;
use leptos::For;
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::WriteSignal;
// use leptos::SignalWrite;

#[component]
pub(crate) fn ImageGallery(
    index: ReadSignal<Index>,
    images: Signal<Vec<(usize, (PathBuf, f32))>>,
    md_key_set: WriteSignal<Option<PathBuf>>,
) -> impl IntoView {
    view! {
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
                        let ds = index.get().description_store.clone();
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
    }
}
