use leptos::component;
use leptos::create_local_resource;
use leptos::create_node_ref;
use leptos::create_server_action;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::view;
use leptos::IntoView;
use leptos::NodeRef;
use leptos::Signal;
use leptos::SignalGet;
use leptos::Transition;

/// A settings form calls root_path_set ( Todo: it hard coded for now ).
///
/// This triggers Index to update,
/// Index is a async process - which is from here onwards is
/// is considered semi static.
///
/// When the user enters a search terms
/// The Index is queried and a set of images produced.
///
#[component]
pub fn Search() -> impl IntoView {
    use crate::component::image_gallery::ImageGallery;
    use crate::component::settings::pannel::Pannel as SettingsPannel;
    use crate::pages::search::get_query;
    use crate::pages::search::AddQuery;
    use crate::pages::search::SearchResult;

    let search_query_action = create_server_action::<AddQuery>();

    let images = create_local_resource(
        move || search_query_action.version().get(),
        get_query,
    );

    let entries = Signal::derive(move || match images.get() {
        Some(Ok(SearchResult { entries, .. })) => entries,
        _ => {
            vec![]
        }
    });

    let count_string = Signal::derive(move || {
        let len = entries.get().len();
        match len {
            0 => String::from("No results found"),
            1 => String::from("1 image found"),
            l => {
                format!("{l} images found")
            }
        }
    });

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let query = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();

        search_query_action.dispatch(AddQuery { query });
    };

    view! {
      <div class="my-0 mx-auto">

        <form on:submit=on_submit class="dark:text-slate-700 px-6 py-2 text-center">
          <label class="hidden" for="search">
            Search
          </label>
          <input
            id="search"
            class="p-2"
            type="text"
            placeholder="Search EXIF data"
            node_ref=input_element
          />
          <input
            type="submit"
            title="Search"
            value=" "
            class="bg-sky-700 cursor-grab rounded-r-lg p-2 hover:bg-sky-600 w-[3.5rem]"
          />
        </form>

        <Transition fallback=|| view! { <p>"Loading count"</p> }>
          <p class="m-6 text-right">{move || count_string.get()}</p>
        </Transition>

        <div class="flex">
          <ImageGallery entries/>
          <SettingsPannel/>
        </div>

      </div>
    }
}
