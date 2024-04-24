use std::path::PathBuf;

use leptos::component;
use leptos::logging::log;
use leptos::view;
use leptos::For;
use leptos::IntoAttribute;
use leptos::IntoView;
use leptos::Memo;
use leptos::ReadSignal;
use leptos::Resource;
use leptos::ServerFnError;
use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::WriteSignal;

use crate::indexer::Index;
use crate::pages::search::SearchResult;

#[component]
pub fn ImageGallery(
    // index: Signal<Index>,
    images: Resource<usize, Result<SearchResult, ServerFnError>>,
    md_key_set: WriteSignal<Option<PathBuf>>,
) -> impl IntoView {
    let entries = match images.get() {
        Some(Ok(SearchResult { entries })) => entries,
        _ => {
            // panic!("image gallery failed to get resource");
            vec![]
        }
    };

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
      each=move || entries.clone()
      key=move |(i, _)| *i
      let:data
      >
      // view=move |(_, (pb, _))| {
        // TODO find a better way than clone.
        {
          // log!("{:#?}", data);
        // let (_, (pb, _)) = data;
        // let pb1 = pb.clone();
        // let pb2 = pb.clone();
        // let pb3 = pb.clone();
        // let pb4 = pb;
        //  view!{
        //     <div class="p-2 mb-4 rounded text-left" style="width:280px;">
        //       <figure class="bg-slate-100 rounded-t" >
        //          <img
        //            width="274" height="160"
        //            class="aspect-square mx-auto"
        //            src={pb1.into_os_string().into_string().unwrap()}
        //          />
        //          <figcaption>
        //            {pb2.file_name().unwrap().to_str().unwrap().to_string()}
        //            <p>
        //              {
        //                 let ds = index.get().description_store;
        //                 ds.get(&pb3 ).map_or_else(|| view!{
        //                   <p class="w-full">"No description"</p>
        //                 }, |name| view!{
        //                   <p class="break-words w-full">{name}</p>
        //                 })
        //               }
        //             <button on:click=move |_| {
        //               log!("button clicked");
        //               // console_log!("button clicked cl");
        //               println!("on the server click metadata");
        //               md_key_set.set(Some(pb4.clone()));
        //              }>"Metadata"</button>

        //            </p>
        //          </figcaption>
        //       </figure>
        //      </div>
        //   }
        }
          </For>

      </section>
    }
}
