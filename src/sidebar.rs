use exif::Field;

use leptos::component;
use leptos::view;
use leptos::For;
use leptos::IntoView;
use leptos::Signal;
use leptos::SignalGet;
use leptos_router::A;

#[component]
pub fn Sidebar(md: Signal<Option<Vec<Field>>>) -> impl IntoView {
    match md.get() {
        Some(data) => {
            view! {
              <div id="side-menu" class="inline-block">
                <A href="">Close</A>
                <div
                  class="
              [&>*:nth-child(even)]:bg-gray-100
              [&>*:nth-child(odd)]:bg-gray-300
              overflow-hidden
              w-[240px]
              }}">
                  <For
                    each =move || data.clone()
                    key = move |field| {field.ifd_num}
                    let:field
                  >

                      <p>{ field.tag.to_string() }</p>
                      <p class="text-right" >{ field.display_value().to_string() }</p>

                  </For>
                  </div>
              </div>
            }
        }
        None => {
            view! {<div id="side-menu-empty" class="w-0"></div>}
        }
    }
}
