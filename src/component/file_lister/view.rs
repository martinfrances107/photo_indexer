use leptos::component;
use leptos::web_sys::HtmlInputElement;
use leptos::IntoView;

use leptos::ev::MouseEvent;
use leptos::html::Input;
use leptos::logging::error;
use leptos::logging::log;
use leptos::logging::warn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::Resource;
use leptos::prelude::ServerAction;
use leptos::prelude::Signal;
use leptos::prelude::Transition;
use leptos::view;
use leptos::web_sys::wasm_bindgen::JsCast;

use crate::component::file_lister::get_list_url;
use crate::component::file_lister::AddListUrl;
use crate::pages::IMAGE_PREFIX;

/// Right hand side side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn Lister() -> impl IntoView {
    // use leptos::prelude::IntoMaybeErased;

    let input_ref = NodeRef::<Input>::new();

    let list_url_action: ServerAction<AddListUrl> = ServerAction::new();
    // Currently this is triggers on page load.
    // TODO: Could trigger this on first open of the tray.
    list_url_action.dispatch(IMAGE_PREFIX.to_string().into());

    let list_urls_resource =
        Resource::new(move || list_url_action.version().get(), get_list_url);

    let list_url = Signal::derive(move || match list_urls_resource.get() {
        Some(Ok(result)) => result.listed_urls,
        // Client side initial value
        // Response failure.
        Some(Err(e)) => {
            error!("{e:#?}");
            Vec::new()
        }
        None => {
            log!("FileLister/Lister: DerivedSignal - list_url - asked for resource got None");
            Vec::new()
        }
    });

    let list_root = Signal::derive(move || match list_urls_resource.get() {
        Some(Ok(result)) => result.root_url,
        // Client side initial value
        // Response failure.
        Some(Err(e)) => {
            error!("{e:#?}");
            String::new()
        }
        None => {
            log!("FileLister/Lister: DerivedSignal - list_root - asked for resource got None");
            String::new()
        }
    });

    let selection_click = move |event: MouseEvent| {
        event.prevent_default();
        if let Some(target) = event.target() {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                let value = input.value();
                log!("input value {}", value);
                list_url_action
                    .dispatch(format!("{IMAGE_PREFIX}{value}").into());
            }
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        match input_ref.get() {
            Some(input) => {
                let value = input.value();
                list_url_action
                    .dispatch(format!("{IMAGE_PREFIX}{value}").into());
            }
            None => {
                warn!("input_ref has been dropped");
            }
        }
    };

    let refresh_click = move |ev: MouseEvent| {
        ev.prevent_default();
        log!("refresh {ev:#?}");
    };

    view! {
      <h2 class="mp-2 text-center">"Select a directory to index"</h2>
      <Transition>
        <p>{list_root}</p>
      </Transition>
      <ol class="flex flex-wrap gap-2 p-2 " on:click=selection_click>
        <Transition fallback=move || {
          view! {
            <li>
              <button on:click=refresh_click>"Refresh"</button>
            </li>
          }
        }>
          <For each=move || { list_url.get().into_iter() } key=|(i, _)| { *i } let:data>
            <li>
              <input
                class="cursor-grab dark:bg-neutral-400 dark:focus:bg-neutral-300 dark:hover:bg-neutral-300 p-2 rounded"
                name="dir"
                readonly
                type="text"
                value=data.1
              />
            </li>
          </For>
        </Transition>
      </ol>

      <form on:submit=on_submit class="dark:text-slate-950 p-2">
        <label class="hidden" for="fl">
          Search
        </label>
        <Transition fallback = || {
          view!{
            <input
            class="block dark:bg-slate-100"
            id="fl"
            placeholder="select directory"
            type="text"
          />
          }
        }>
          <input
            class="block dark:bg-slate-100"
            id="fl"
            node_ref=input_ref
            value=list_root
            type="text"
          />
        </Transition>

        <input
          class="bg-sky-700 block cursor-grab dark:text-white rounded mt-3 p-2 hover:bg-sky-600"
          type="submit"
          title="Select"
          value="UPDATE"
        />
      </form>
    }
}
