use leptos::server;
use leptos::ServerFnError;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "ssr")]
use crate::pages::GLOBAL_STATE;
#[cfg(feature = "ssr")]
use crate::util::cantor_pair;

pub(crate) mod view;

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
    // counter that increments every for query.
    pub version: usize,
}

#[server]
pub async fn add_query(query: String) -> Result<(), ServerFnError> {
    use tracing::log;
    log::debug!("server: entry search_query");

    let sq = query.chars().collect::<Vec<char>>();

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            state.query = sq;
            Ok(())
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}

// TODO wierd leptos default naming convention
// get_query get the result of the last query
// ie get a list of images.
#[server]
pub async fn get_query(version: usize) -> Result<SearchResult, ServerFnError> {
    use crate::pages::IMAGE_PREFIX;

    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            let entries_raw = state.index.model.search_query(&state.query);
            state.entries = entries_raw
                .iter()
                .enumerate()
                .map(|(i, path_rank)| {
                    let key = cantor_pair(version, i);

                    // Construct url from filename
                    let url = match path_rank
                        .0
                        .strip_prefix(state.selected_dir.clone())
                    {
                        Ok(filename) => {
                            format!(
                                "{IMAGE_PREFIX}{}",
                                filename.display().to_string()
                            )
                        }
                        Err(_) => String::default(),
                    };

                    let description = match state
                        .index
                        .description_store
                        .get(&url)
                    {
                        Some(description) => description.to_string(),
                        None => String::default(),
                    };

                    SRElem {
                        description,
                        key,
                        url,
                    }
                })
                .collect();
            Ok(SearchResult {
                entries: state.entries.clone(),
                version,
            })
        }
        Err(e) => {
            panic!("/search query - could not unlock {e}");
        }
    }
}
