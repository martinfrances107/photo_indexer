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
      <div class="my-0 m-auto">

        <form
          id="search"
          on:submit=on_submit
          class="justify-center dark:text-slate-700 flex h-[3.5rem]"
        >
          <label class="hidden" for="search-text">
            Search
          </label>
          <input
            class="p-2"
            id="search-text"
            placeholder="Search EXIF data"
            node_ref=input_element
            type="text"
          />
          <button form="search" title="Search" value=" ">
            <svg
              id="searchSVG"
              class="bg-sky-700 cursor-grab fill-sky-900 rounded-r-lg hover:bg-sky-600"
              viewBox="0 0 512 512"
              width="100%"
              height="100%"
              version="1.1"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              xmlns="http://www.w3.org/2000/svg"
              xmlns:svg="http://www.w3.org/2000/svg"
            >
              <path d="m -610.98815,-246.28042 c 68.31986,0 247.34788,0 247.34788,0 0,0 0,185.290321 0,256.0000039 0,70.7096771 0,255.9999961 0,255.9999961 0,0 -179.02802,0 -247.34788,0 -68.31988,0 -247.34788,0 -247.34788,0 0,0 0,-185.290319 0,-255.9999961 0,-70.7096829 0,-256.0000039 0,-256.0000039 0,0 179.028,0 247.34788,0 z M -726.68312,55.138934 h 115.69497 v 73.187106 c 0,11.04518 12.96582,16.61935 20.4461,8.77419 L -476.54262,18.493776 c 4.68766,-4.851603 4.68766,-12.593542 0,-17.4451634 L -590.54205,-117.66106 c -7.58001,-7.84517 -20.4461,-2.27098 -20.4461,8.7742 v 73.187082 h -115.69497 c -6.58265,0 -11.96845,5.574196 -11.96845,12.387102 V 42.75184 c 0,6.812915 5.3858,12.387094 11.96845,12.387094 z"/>
            </svg>
          </button>
        </form>

        <Transition fallback=|| view! { <p>"Loading count"</p> }>
          <p class="m-6 text-right">{move || count_string.get()}</p>
        </Transition>

        <div class="flex gap-2">
          <ImageGallery entries/>
          <SettingsPannel/>
        </div>

      </div>
    }
}
