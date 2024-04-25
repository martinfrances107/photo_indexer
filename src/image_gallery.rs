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

#[component]
pub fn ImageGallery(
    entries: Signal<Vec<std::string::String>>,
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
                   src={data.1.clone()}
                 />
                 <figcaption>
                   {data.1}
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
