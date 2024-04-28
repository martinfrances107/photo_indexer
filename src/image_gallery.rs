use std::path::PathBuf;

use leptos::component;
use leptos::logging::log;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::Signal;
use leptos::SignalGet;
use leptos::Transition;
use leptos::WriteSignal;

use crate::pages::search::SRElem;

#[component]
pub fn ImageGallery(
    entries: Signal<Vec<SRElem>>,
    md_key_set: WriteSignal<Option<PathBuf>>,
) -> impl IntoView {
    view! {

      <section class="
        dark:text-slate-950 bg-slate-600
        flex
        flex-wrap
        gap-y-4
        min-h-full
        justify-evenly
        px-2 py-4
        rounded-t-lg
        w-full
        ">
        <Transition
         fallback =move || view!{ <p>"Loading Image Gallery"</p> }
        >
          <For
          each=move || entries.get().into_iter().enumerate()
          key=move |(i, _)| *i
          let:data
          >

            <div class="p-2 mb-4 rounded text-left" style="width:280px;">
              <figure class="bg-slate-100 rounded-t" >
                 <img
                   width="274" height="160"
                   class="aspect-square mx-auto"
                   src={data.1.path_rank.0.clone().display().to_string()}
                 />
                 <figcaption>
                   {data.1.description}
                   <p>
                    //  {
                    //     let ds = index.get().description_store;
                    //     ds.get(&pb3 ).map_or_else(|| view!{
                    //       <p class="w-full">"No description"</p>
                    //     }, |name| view!{
                    //       <p class="break-words w-full">{name}</p>
                    //     })
                    //   }
                    <button on:click=move |_| {
                      // md_key_set.set(Some(pb4.clone()));
                     }>"Metadata"</button>

                   </p>
                 </figcaption>
              </figure>
            </div>
          </For>
        </Transition>
      </section>
    }
}
