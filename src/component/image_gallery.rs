use exif::Field;
use leptos::prelude::component;
use leptos::prelude::server;
use leptos::prelude::IntoView;
use leptos::prelude::ServerFnError;
use leptos::prelude::Signal;
use serde::Deserialize;
use serde::Serialize;

use crate::pages::search::SRElem;

#[allow(clippy::unused_async)]
#[server]
pub async fn add_meta_data(
    url: Option<String>,
) -> Result<Option<Vec<Field>>, ServerFnError> {
    use crate::pages::GLOBAL_STATE;
    use leptos::logging::debug_warn;

    debug_warn!("server: entry metadata");

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            if let Some(url) = url {
                state.metadata = state.index.md_store.get(&url).cloned();
            } else {
                state.metadata = None;
            }
            Ok(state.metadata.clone())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

#[server]
pub async fn get_metadata() -> Result<Option<Vec<Field>>, ServerFnError> {
    use crate::pages::GLOBAL_STATE;
    match GLOBAL_STATE.lock() {
        Ok(state) => Ok(state.metadata.clone()),
        Err(_) => Err(ServerFnError::ServerError(
            "Global state unavailable".to_string(),
        )),
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchResult {
    pub entries: Vec<SRElem>,
}

#[component]
pub fn ImageGallery(entries: Signal<Vec<SRElem>>) -> impl IntoView {
    use leptos::prelude::ClassAttribute;
    use leptos::prelude::ElementChild;
    use leptos::prelude::For;
    use leptos::prelude::Get;
    use leptos::prelude::GlobalAttributes;
    use leptos::prelude::OnAttribute;
    use leptos::prelude::Read;
    use leptos::prelude::Resource;
    use leptos::prelude::ServerAction;
    use leptos::prelude::Show;
    use leptos::prelude::Transition;
    use leptos::view;

    let metadata_action: ServerAction<AddMetaData> = ServerAction::new();

    let metadata_resource = Resource::new(
        move || metadata_action.version().get(),
        |_| async move {
            // as
            let a = get_metadata().await;

            match a {
                Ok(a) => a.unwrap_or_default(),
                Err(_e) => {
                    vec![]
                }
            }
        },
    );

    view! {
      <Transition fallback=|| {
        view! { <p>"SideBar..."</p> }
      }>
        // Sidebar
        <Show
          when=move || { metadata_resource.read().as_ref().is_some_and(|v| !v.is_empty()) }
          fallback=|| {
            view! { <div id="side-menu-empty" class="w-0"></div> }
          }
        >

          <div
            class="bg-slate-800 inline-block rounded shadow-inner shadow-slate-700"
            id="side-menu"
          >
            <button
              class="font-medium pr-4 pt-2 text-right text-lg w-full text-neutral-400 hover:text-neutral-300"
              on:click=move |_| {
                metadata_action.dispatch(AddMetaData { url: None });
              }
              title="close"
            >
              "X"
            </button>
            <hr class="m-1 dark:bg-slate-100" />
            <div class="
            [&>*:nth-child(even)]:dark:bg-neutral-400
            [&>*:nth-child(even)]:text-slate-950
            overflow-hidden
            w-[240px]
            }}">
              <For
                each=move || { metadata_resource.get().unwrap() }
                key=|field| { field.ifd_num }
                let:field
              >
                <p class="ps-0.5">{field.tag.to_string()}</p>
                <p class="pe-2.5 text-right">{field.display_value().to_string()}</p>
              </For>
            </div>
          </div>

        </Show>
      </Transition>
      <div class="
      bg-slate-800
      content-start
      dark:text-slate-950
      flex
      flex-wrap
      gap-y-2
      gap-x-2
      justify-start
      min-h-full
      rounded-lg
      shadow-inner
      shadow-slate-700
      w-full
      ">
        <Transition fallback=|| view! { <div></div> }>
          <For each=move || entries.get().into_iter() key=|e| e.key let:data>

            <div class="hover:bg-slate-600 relative rounded text-left w-[280px]">
              {
                let image_alt = data.description.replace('"', "");
                let url = data.url.clone();
                let button_url = data.url.clone();
                let show_description = !data.description.is_empty();
                let img_title = data.description.replace('"', "");
                view! {
                  <figure class="bg-slate-100 pt-2 rounded-t">
                    <img
                      alt=move || image_alt.clone()
                      class="block mx-auto w-[274px] h-[160px]"
                      src=url
                      title=move || img_title.clone()
                    />
                    <figcaption>
                      <Show
                        when=move || show_description
                        fallback=|| {
                          view! {
                            <p class="break-words line-clamp-3 min-h-12 ml-1 pt-4">"No description"</p>
                          }
                        }
                      >
                        <p class="break-words line-clamp-3 min-h-12 ml-1 pt-4">
                          {data.description.replace('"', "")}
                        </p>
                      </Show>
                    </figcaption>
                  </figure>
                  <button
                    class="absolute bg-slate-950/50 hover:bg-slate-950 font-mono p-3 rounded-full right-4 text-white text-right top-4"
                    on:click=move |_| {
                      metadata_action
                        .dispatch(AddMetaData {
                          url: Some(button_url.clone()),
                        });
                    }
                    title="Open metadata"
                  >
                    "M"
                  </button>
                  <button
                    class="absolute bg-slate-950/50 hover:bg-slate-950 font-mono p-3 rounded-full right-4 text-white text-right top-20"
                    on:click=move |_| {}
                    title="FULLSCREEN"
                  >
                    "F"
                  </button>
                }
              }
            </div>
          </For>
        </Transition>
      </div>
    }
}
