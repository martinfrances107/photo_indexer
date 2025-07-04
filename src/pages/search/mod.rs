use leptos::prelude::ServerFnError;
use leptos::server;

#[cfg(feature = "ssr")]
use actix_web::get;
#[cfg(feature = "ssr")]
use actix_web::http::StatusCode;
#[cfg(feature = "ssr")]
use actix_web::HttpResponse;

use serde::Deserialize;
use serde::Serialize;

use crate::pages::AddQuery;

#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;
pub mod view;

// Search Result Element
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SRElem {
    pub description: String,
    // key: Ensures images in the gallery have a
    // unique id, otherwise images are not correctly refreshed.
    pub key: usize,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SearchResult {
    pub entries: Vec<SRElem>,
}

#[allow(clippy::unused_async)]
#[server]
pub async fn update_query(aq: AddQuery) -> Result<(), ServerFnError> {
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.query = aq.query;
            state.query_version = state.query_version + 1;
            Ok(())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

// TODO weird leptos default naming convention
// get_query get the result of the last query
// ie get a list of images.
#[server]
pub async fn get_query() -> Result<SearchResult, ServerFnError> {
    use crate::pages::IMAGE_PREFIX;

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            let entries_raw = state.index.model.search_query(&state.query);
            state.entries = entries_raw
                .iter()
                .enumerate()
                .map(|(i, path_rank)| {
                    let key = crate::util::cantor_pair(state.query_version, i);

                    // Construct URL from filename
                    let url = path_rank
                        .0
                        .strip_prefix(state.selected_dir.clone())
                        .map_or_else(
                            |_| String::default(),
                            |filename| {
                                format!("{IMAGE_PREFIX}{}", filename.display())
                            },
                        );

                    let description = state
                        .index
                        .description_store
                        .get(&url)
                        .map_or_else(String::default, ToString::to_string);

                    SRElem {
                        description,
                        key,
                        url,
                    }
                })
                .collect();
            Ok(SearchResult {
                entries: state.entries.clone(),
            })
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}
