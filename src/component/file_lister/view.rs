use leptos::component;
use leptos::IntoView;

/// Right handside side bar.
///
/// Form is used to set the indexer to a new value.
#[component]
pub fn Lister() -> impl IntoView {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlInputElement;

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
    use leptos::NodeRef;
    use leptos::Signal;
    use leptos::SignalGet;
    use leptos::SignalSet;
    use leptos::Transition;
    use log::error;

    use crate::component::file_lister::get_list_url;
    use crate::component::file_lister::AddListUrl;
    use crate::pages::IMAGE_PREFIX;

    let input_element: NodeRef<html::Input> = create_node_ref();

    let (select_value, select_value_set) = create_signal(String::from("AAA"));

    let (current_selection, current_selection_set) =
        create_signal::<String>(String::from(IMAGE_PREFIX));

    let list_url_action = create_server_action::<AddListUrl>();

    let list_urls_resource = create_local_resource(
        move || list_url_action.version().get(),
        get_list_url,
    );

    let list_url = Signal::derive(move || match list_urls_resource.get() {
        Some(Ok(result)) => result.listed_urls,
        // Client side initial value
        // Response failure.
        Some(Err(e)) => {
            error!("{e:#?}");
            vec![]
        }
        None => {
            log!("FileLister/Lister: DerivedSignal - list_url - asked for resource got None");
            vec![]
        }
    });

    let selection_click = move |event: MouseEvent| {
        event.prevent_default();
        match event.target() {
            Some(target) => match target.dyn_ref::<HtmlInputElement>() {
                Some(input) => {
                    let value = input.value();
                    list_url_action.dispatch(AddListUrl {
                        url: format!("{IMAGE_PREFIX}{value}"),
                    });

                    // Move vlaue into the inputvalue
                    match input_element.get() {
                        Some(_) => {
                            select_value_set.set(value);
                        }
                        None => {
                            log::warn!(
                                "file_lister: input not found in the DOM"
                            );
                        }
                    }
                }
                None => {
                    log::warn!("selection_click() - Extracted a target that was not a HtmlInputElement");
                }
            },
            None => {
                log::warn!("selection_click() - Could not extract a target");
            }
        };
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
        ev.prevent_default();
        log!("refresh {ev:#?}");
    };

    view! {
        <h2 class="mp-2 text-center">"Select a directory to index"</h2>
        <p>{move || current_selection.get()}</p>
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
                <!-- "css issue - Want to set hover on input element, but conventional css fails" -->
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

        <form
          on:submit=on_submit
          class="dark:text-slate-700 p-2"
        >
          <label class="hidden" for="fl">
            Search
          </label>
          <input
            class="block"
            id="fl"
            placeholder="select directory"
            node_ref=input_element
            value={select_value}
            type="text"
          />

          <input
            class="bg-sky-700 block cursor-grab dark:text-white rounded mt-3 p-2 hover:bg-sky-600"
            type="submit"
            title="Select"
            value="UPDATE"
          />
        </form>
    }
}
