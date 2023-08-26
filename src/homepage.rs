use std::path::Path;

use leptos::component;
use leptos::create_signal;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::Scope;
use leptos::ServerFnError;
use leptos::SignalWith;
use leptos::*;
use leptos_meta::Style;
use leptos_router::MultiActionForm;
use log::info;

use crate::gallery::GalleryItem;
use crate::indexer::Index;

#[server(SearchImages, "/api", "Cbor")]
pub async fn search_images(title: String) -> Result<(), ServerFnError> {
    log!("in search {title}");

    log!("in server");

    Ok(())
}

/// Holds main search bar and results.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let search_images = create_server_multi_action::<SearchImages>(cx);

    // let input_ref = create_node_ref::<Input>(cx);

    let root = Path::new(&"../exif-samples");

    let (index, _set_index) = create_signal::<Index>(cx, Index::new(cx, root));
    info!("Indexing complete about to start server.");

    // Initially apply no filter
    let filtered = move || index.with(|index| index.doc_links.to_vec());

    view! { cx,
       <main class="bg-slate-950">
         <Style>
           "body { font-weight: bold; }"
           ".gallery {
              display: grid;
              grid-template-columns: repeat( auto-fill, minmax(320px, 1fr) );
            }"
         </Style>

         <section>

           <h1>"Photo Indexer"</h1>

           <MultiActionForm action=search_images >
             <label>
               "Search EXIF data"
               <input type="text" name="title"/>
             </label>
             <input type="submit" value="Add"/>
           </MultiActionForm>

         </section>

         <section class="
               gallery bg-slate-600
               display: grid;
               grid-template-columns: repeat( auto-fill, minmax(320px, 1fr) );"
             >
         <Transition
         fallback =move || view!{ cx, <p>"Loading"</p>}
         >
         {move || {
           view!{cx,

               <For
                 each=filtered
                 key=|doc_link| doc_link.uuid()
                 view=move |cx, doc_link| {
                   view! {
                     cx,
                     <GalleryItem doc_link/>
                   }
                 }
               />

              }
            }
          }
          </Transition>
          </section>
       </main>
    }
}
