use leptos::component;

use leptos::prelude::NodeRef;
use leptos::IntoView;

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
    use leptos::ev::SubmitEvent;
    use leptos::html;
    use leptos::prelude::Action;
    use leptos::prelude::ClassAttribute;
    use leptos::prelude::ElementChild;
    use leptos::prelude::Get;
    use leptos::prelude::GlobalAttributes;
    use leptos::prelude::NodeRefAttribute;
    use leptos::prelude::OnAttribute;
    use leptos::prelude::Resource;
    use leptos::prelude::Signal;
    use leptos::prelude::Transition;
    use leptos::view;

    use crate::component::image_gallery::ImageGallery;
    use crate::component::settings::panel::Panel as SettingsPanel;
    use crate::pages::search::get_query;
    use crate::pages::search::update_query;
    use crate::pages::search::SearchResult;
    use crate::pages::AddQuery;

    let search_query_action =
        Action::new(|aq: &AddQuery| update_query(aq.clone()));

    let images = Resource::new(
        move || {
            // inputs.
            search_query_action.version().get()
        },
        |_| {
            // fetcher
            get_query()
        },
    );

    // I want to simplify this by removing this Signal::derive()
    // but I need to find a way of making a async resource.
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

    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let query = input_element
            .get()
            .expect("<input> should be mounted.")
            .value();
        let aq = AddQuery {
            query: query.chars().collect(),
        };
        search_query_action.dispatch(aq);
    };

    view! {
      <div class="my-0 m-auto">

        <form
          id="search"
          on:submit=on_submit
          class="justify-center dark:text-slate-950 flex h-[3.5rem]"
        >
          <label class="hidden" for="search-text">
            Search
          </label>
          <input
            class="p-2 focus:outline-none dark:bg-slate-100"
            id="search-text"
            placeholder="Search EXIF data"
            node_ref=input_element
            type="text"
          />
          <button form="search" title="Search" value=" " class="w-[56px] w-[56px]">
          <svg
            class="group"
            id="searchSVG"
            viewBox="0 0 512 512"
            width="100%"
            height="100%"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="512" height="512" class="fill-neutral-400 group-hover:fill-neutral-300" />
            <path class="fill-sky-700  cursor-grab rounded-r-lg group-hover:fill-sky-600"
               d="m 256,0 c 70.70966,0 256,0 256,0 0,0 0,185.29031 0,256 0,70.70967 0,255.99999 0,255.99999 0,0 -185.29034,0 -256,0 -70.70968,0 -256,0 -256,0 C 0,511.99999 0,326.70967 0,256 0,185.29031 0,0 0,0 0,0 185.29032,0 256,0 Z M 136.25807,301.41935 H 256 v 73.1871 c 0,11.04518 13.41936,16.61935 21.1613,8.77419 L 395.14838,264.77419 c 4.85163,-4.8516 4.85163,-12.59354 0,-17.44516 L 277.1613,128.61935 C 269.31614,120.77418 256,126.34837 256,137.39355 v 73.18709 H 136.25807 c -6.81291,0 -12.3871,5.57419 -12.3871,12.3871 v 66.06451 c 0,6.81292 5.57419,12.3871 12.3871,12.3871 z"
              />
          </svg>

          </button>
        </form>

        <Transition fallback=|| view! { <p>"Loading count"</p> }>
          <p class="m-6 text-right">{count_string}</p>
        </Transition>

        <div class="flex gap-2">
          <ImageGallery entries />
          <SettingsPanel />
        </div>

      </div>
    }
}
