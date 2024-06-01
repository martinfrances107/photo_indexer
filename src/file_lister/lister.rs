use leptos::component;
use leptos::create_local_resource;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::create_signal;
use leptos::ev;
use leptos::ev::MouseEvent;
use leptos::html;
use leptos::logging::log;

use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::NodeRef;

use leptos::Signal;
use leptos::SignalGet;
use leptos::SignalSet;
use leptos::Transition;

use crate::file_lister::get_list_url;
use crate::file_lister::AddListUrl;
use crate::pages::IMAGE_PREFIX;

/// Right handside side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn Lister() -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();

    let (current_selection, current_selection_set) =
        create_signal::<String>(String::from(IMAGE_PREFIX));

    let list_url_action = create_server_action::<AddListUrl>();

    let list_urls_resource = create_local_resource(
        move || list_url_action.version().get(),
        get_list_url,
    );

    let list_url = Signal::derive(move || match list_urls_resource.get() {
        Some(Ok(result)) => {
            // // Integrity/Sync check.
            // // What is requested should be contained in the resposne.
            // log!(
            //     "current selection {:#?} - {:#?}",
            //     current_selection.get(),
            //     result.list_url
            // );
            // debug_assert!(current_selection.get() == result.list_url);
            result.listed_urls
        }
        // Client side initial value
        // Response failure.
        Some(Err(e)) => {
            log!("{e:#?}");
            vec!["client_error_x".into()]
        }
        None => {
            log!("None");
            vec!["client_none_x".into()]
        }
    });

    let selection_click = move |event: MouseEvent| {
        event.prevent_default();
        log!("{event:#?}");
        current_selection_set.set(String::from("wait ..."));
        // Update form with pre-prepared value.
        if let Some(event) = event.related_target() {
            log!("event {event:#?}");
        } else {
            log!("failed to get event");
        }
    };

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let url = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();

        current_selection_set.set(url.clone());
        list_url_action.dispatch(AddListUrl { url });
    };

    let refresh_click = move |ev: MouseEvent| {
        log!("refresh {ev:#?}");
    };

    view! {
      <div>
        <h2>File Lister</h2>
        <p>{move || current_selection.get()}</p>
        <ol>
          <Transition fallback=move || {
              view! {
                <li>
                  <button on:click=refresh_click>{"hello"}</button>
                </li>
              }
          }>
            <For
              each=move || { list_url.get().into_iter().enumerate() }
              key=|(i, _)| { *i }
              let:data
            >
              <li>
                <button on:click=selection_click>{data.1}</button>
              </li>
            </For>
          </Transition>
        </ol>

        <form on:submit=on_submit class="dark:text-slate-700 px-6 py-2 text-center">
          <label class="hidden" for="search">
            Search
          </label>
          <input id="search" type="text" placeholder="select directory" node_ref=input_element/>
          <input type="submit" title="Select" value="SUBMIT"/>
        </form>

      </div>
    }
}
