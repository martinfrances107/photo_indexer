use leptos::prelude::*;
use serde::Deserialize;
use serde::Serialize;
pub mod view;

// A request by the client to to change the root directory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListUrlResult {
    root_url: String,
    listed_urls: Vec<(usize, String)>,
    version: usize,
}

#[cfg(feature = "ssr")]
fn is_not_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| entry.depth() == 0 || !s.starts_with('.'))
}

// Ingest and sanitize url.
//
// Errors: -
// timeout acquiring lock.
// IMAGE_PREFIX sanitization check.
// [url] must map to a valid directory.
//
#[allow(clippy::unused_async)]
#[server]
pub async fn add_list_url(url: String) -> Result<String, ServerFnError> {
    use crate::pages::UrlSanitizationError;
    use crate::pages::GLOBAL_STATE;
    use crate::pages::IMAGE_PREFIX;

    leptos::logging::log!("server: entry add_root_url");
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            match state.set_list_dir_from_url(&url) {
                Ok(()) => Ok(String::from("add_list_url")),
                Err(UrlSanitizationError::MissingPrefix) => {
                    Err(ServerFnError::Args(format!(
                        "URL must be prefixed with {IMAGE_PREFIX}"
                    )))
                }
                Err(UrlSanitizationError::InvalidDirectory) => {
                    // TODO: In production, this will leak information to an attacker
                    // Should I emit a bland Internal Server error message?
                    Err(ServerFnError::ServerError(String::from(
                        "Not a valid images url",
                    )))
                }
            }
        }
        Err(_e) => Err(ServerFnError::ServerError(String::from(
            "Error acquiring global state",
        ))),
    }
}

#[allow(clippy::unused_async)]
#[server]
pub async fn get_list_url(
    version: usize,
) -> Result<ListUrlResult, ServerFnError> {
    use crate::util::cantor_pair;
    use walkdir::WalkDir;

    let version = version + 1;
    match crate::pages::GLOBAL_STATE.lock() {
        Ok(state) => {
            let container_dir = state.container_dir();
            let root_url = state
                .list_dir()
                .strip_prefix(container_dir.clone())
                .map_or(String::default(), |root| root.display().to_string());
            let uuid_url = WalkDir::new(state.list_dir())
                .max_depth(1)
                .into_iter()
                .filter_entry(is_not_hidden)
                .filter_map(|entry| match entry {
                    Ok(entry) => {
                        if entry.path().is_dir() {
                            Some(entry)
                        } else {
                            None
                        }
                    }
                    Err(_e) => None,
                })
                .filter_map(|entry| {
                    entry
                        .path()
                        .strip_prefix(container_dir.clone())
                        .map_or(None, |url| Some(url.display().to_string()))
                })
                .enumerate()
                .map(|(i, url)| (cantor_pair(i, version), url))
                .collect();

            // state.listed_urls = listed_urls;
            Ok(ListUrlResult {
                root_url,
                listed_urls: uuid_url,
                version,
            })
        }
        Err(e) => {
            let err_msg = format!(
                "get_list_url() failed to unlock() global state {e:#?}"
            );
            leptos::logging::error!("{}", err_msg);
            // TODO: In production, this will leak information to an attacker
            // Should I emit a bland Internal Server error message?
            Err(ServerFnError::ServerError(err_msg))
        }
    }
}
