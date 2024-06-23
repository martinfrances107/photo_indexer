use exif::Field;
use leptos::component;
use leptos::create_local_resource;
use leptos::create_server_action;
use leptos::server;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::ServerFnError;
use leptos::Signal;
use leptos::SignalGet;
use leptos::Transition;
use serde::Deserialize;
use serde::Serialize;

use crate::pages::search::SRElem;

#[server]
pub async fn add_meta_data(
    url: Option<String>,
) -> Result<Option<Vec<Field>>, ServerFnError> {
    use crate::pages::GLOBAL_STATE;

    use tracing::log;
    log::debug!("server: entry metadata");

    match GLOBAL_STATE.lock() {
        Ok(mut state) => match url {
            Some(url) => {
                state.metadata = match state.index.md_store.get(&url) {
                    Some(metadata) => Some(metadata.clone()),
                    None => None,
                };
                Ok(state.metadata.clone())
            }
            None => {
                state.metadata = None;
                Ok(state.metadata.clone())
            }
        },
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_metadata() -> Result<Option<Vec<Field>>, ServerFnError> {
    use crate::pages::GLOBAL_STATE;
    let metadata = match GLOBAL_STATE.lock() {
        Ok(state) => state.metadata.clone(),
        Err(e) => {
            panic!("get_query - could not unlock {e}");
        }
    };

    Ok(metadata)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchResult {
    pub entries: Vec<SRElem>,
}

#[component]
pub fn ImageGallery(entries: Signal<Vec<SRElem>>) -> impl IntoView {
    let metadata_action = create_server_action::<AddMetaData>();
    let metadata_resource = create_local_resource(
        move || metadata_action.version().get(),
        |_| get_metadata(),
    );
    let metadata = Signal::derive(move || match metadata_resource.get() {
        Some(Ok(metadata)) => metadata,
        _ => None,
    });

    view! {
      <Transition fallback=|| {
          view! { <p>"SideBar..."</p> }
      }>
        // Sidebar
        {move || {
            metadata
                .get()
                .map_or_else(
                    || view! { <div id="side-menu-empty" class="w-0"></div> },
                    |data| {
                        view! {
                          <div id="side-menu" class="bg-slate-800 inline-block rounded shaddow-md">
                            <button
                              class="font-medium pr-4 pt-2 text-right text-lg w-full"
                              on:click=move |_| metadata_action.dispatch(AddMetaData { url: None })
                              title="close"
                            >
                              "X"
                            </button>
                            <hr class="m-1"/>
                            <div class="
                            [&>*:nth-child(even)]:bg-gray-400
                            overflow-hidden
                            w-[240px]
                            }}">
                              <For
                                each=move || data.clone()
                                key=|field| { field.ifd_num }
                                let:field
                              >
                                <p class="ps-0.5">{field.tag.to_string()}</p>
                                <p class="pe-2.5 text-right">{field.display_value().to_string()}</p>
                              </For>
                            </div>
                          </div>
                        }
                    },
                )
        }}

      </Transition>
      <div class="
      bg-slate-800
      content-start
      dark:text-slate-950
      flex
      flex-wrap
      gap-y-2
      justify-start
      min-h-full
      rounded-lg
      shadow-inner
      shadow-slate-700
      w-full
      ">

        <Transition fallback=|| view! { <div></div> }>
          <For each=move || entries.get().into_iter() key=|e| e.key let:data>

            <div class="hover:bg-slate-600 mx-2 my-4 relative rounded text-left w-[280px]">
              <figure class="bg-slate-100 pt-2 rounded-t">
                <img
                  alt={data.description.clone().replace('"', "")}
                  class="aspect-square mx-auto w-[274px] h-[160px]"
                  src=data.url.clone()
                  title={data.description.clone().replace('"', "")}
                />
                <figcaption>
                  {if data.description.is_empty() {
                      view! { <p class="break-words line-clamp-3 min-h-12 pt-4">"No description"</p> }
                  } else {
                      view! { <p class="break-words line-clamp-3 min-h-12 pt-4">{data.description}</p> }
                  }}

                </figcaption>
              </figure>
              <button
                class="absolute bg-black/50 font-mono p-3 rounded-full right-4 text-white text-right top-4"
                on:click=move |_| metadata_action
                        .dispatch(AddMetaData {
                            url: Some(data.url.clone()),
                        })

                title="Open metadata"
              >

                "M"
              </button>
              <button
                class="absolute bg-black/50 font-mono p-3 rounded-full right-4 text-white text-right top-20"
                on:click=move |_| {}
                title="FULLSCREEN"
              >
                "F"
              </button>
            </div>

          </For>
        </Transition>
      </div>
    }
}
