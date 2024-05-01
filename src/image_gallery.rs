use std::path::PathBuf;

use exif::Field;
use serde::Deserialize;
use serde::Serialize;

use leptos::component;
use leptos::create_local_resource;
use leptos::create_server_action;
use leptos::logging::log;
use leptos::server;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::ServerFnError;
use leptos::Signal;
use leptos::SignalGet;
use leptos::Transition;

use crate::pages::search::SRElem;

#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;

#[server]
pub async fn add_meta_data(
    filename: Option<PathBuf>,
) -> Result<Option<Vec<Field>>, ServerFnError> {
    log!("server: entry metadata");

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {

            match filename {
              Some(filename) => {

                state.metadata = match state.index.md_store.get(&filename) {
                  Some(metadata) => Some(metadata.clone()),
                  None => None,
                };
                Ok(state.metadata.clone())

              }
              None => {

                state.metadata = None;
                Ok(state.metadata.clone())

              }


            }


        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_metadata() -> Result<Option<Vec<Field>>, ServerFnError> {
    let metadata = match GLOBAL_STATE.lock() {
        Ok(state) => state.metadata.clone(),
        Err(e) => {
            panic!("get_query - could not unlock {e}");
        }
    };

    Ok(metadata)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
      <Transition fallback=move || {
          view! { <p>"SideBar..."</p> }
      }>

        {// Sidebar
        move || {
            metadata
                .get()
                .map_or_else(
                    || view! { <div id="side-menu-empty" class="w-0"></div> },
                    |data| {
                        view! {
                          <div id="side-menu" class="inline-block">
                            <button on:click=move |_| {
                                metadata_action.dispatch(AddMetaData { filename: None });
                            }>"Close"</button>
                            <div class="
                            [&>*:nth-child(even)]:bg-gray-100
                            [&>*:nth-child(odd)]:bg-gray-300
                            overflow-hidden
                            w-[240px]
                            }}">
                              <For
                                each=move || data.clone()
                                key=move |field| { field.ifd_num }
                                let:field
                              >

                                <p>{field.tag.to_string()}</p>
                                <p class="text-right">{field.display_value().to_string()}</p>

                              </For>
                            </div>
                          </div>
                        }
                    },
                )
        }}

      </Transition>
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

        <Transition fallback=move || view! { <p>"Loading Image Gallery"</p> }>
          <For each=move || entries.get().into_iter().enumerate() key=move |(i, _)| *i let:data>

            <div class="p-2 mb-4 rounded text-left" style="width:280px;">
              <figure class="bg-slate-100 rounded-t">
                <img
                  width="274"
                  height="160"
                  class="aspect-square mx-auto"
                  src=data.1.path_rank.0.clone().display().to_string()
                />
                <figcaption>

                  {if data.1.description.is_empty() {
                      view! { <p class="w-full">"No description"</p> }
                  } else {
                      view! { <p class="break-words w-full">{data.1.description}</p> }
                  }}
                  <button on:click=move |_| {
                      metadata_action
                          .dispatch(AddMetaData {
                              filename: Some(data.1.path_rank.0.clone()),
                          });
                  }>"Metadata"</button>
                </figcaption>
              </figure>
            </div>

          </For>
        </Transition>
      </section>
    }
}
