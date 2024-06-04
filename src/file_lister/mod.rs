use leptos::server;
use leptos::ServerFnError;

use serde::Deserialize;
use serde::Serialize;

pub(crate) mod lister;

// A request by the client to to change the root directory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ListUrlResult {
    listed_urls: Vec<String>,
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
// timeout aquiring lock.
// IMAGE_PREFIX santitisation check.
// [url] must map to a valid directory.
//
#[server]
pub async fn add_list_url(url: String) -> Result<String, ServerFnError> {
    use crate::pages::UrlSanitizationError;
    use crate::pages::GLOBAL_STATE;
    use crate::pages::IMAGE_PREFIX;

    leptos::logging::log!("server: entry add_root_url");
    match GLOBAL_STATE.lock() {
        Ok(mut state) => {
            match state.set_list_dir_from_url(url) {
                Ok(_) => Ok(String::from("add_list_url")),
                Err(UrlSanitizationError::MissingPrefix) => {
                    Err(ServerFnError::Args(format!(
                        "URL must be prefixed with {IMAGE_PREFIX}"
                    )))
                }
                Err(UrlSanitizationError::InvalidDirectory) => {
                    // TODO: In production, this will leak infomation to an attacker
                    // Should I emmit a bland Internal Server error message?
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

#[server]
pub async fn get_list_url(
    version: usize,
) -> Result<ListUrlResult, ServerFnError> {
    use walkdir::WalkDir;

    match crate::pages::GLOBAL_STATE.lock() {
        Ok(mut state) => {
            let container_dir = state.container_dir().clone();
            let listed_urls = WalkDir::new(state.list_dir())
                .max_depth(1)
                .into_iter()
                .filter_entry(|e| is_not_hidden(e))
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
                    match entry.path().strip_prefix(container_dir.clone()) {
                        Ok(url) => Some(url.display().to_string()),
                        Err(_) => None,
                    }
                })
                .collect();

            // state.list_url = url;
            state.listed_urls = listed_urls;
            Ok(ListUrlResult {
                listed_urls: state.listed_urls.clone(),
                version: version + 1,
            })
        }
        Err(e) => {
            let err_msg = format!(
                "get_list_url() failed to unlock() global state {e:#?}"
            );
            println!("get_list_url{}", err_msg);
            log::error!("{}", err_msg);
            // TODO: In production, this will leak infomation to an attacker
            // Should I emmit a bland Internal Server error message?
            Err(ServerFnError::ServerError(err_msg))
        }
    }
}

// #[server]
// pub async fn add_selected_url(url: String) -> Result<(), ServerFnError> {

//     use crate::indexer::Index;
//     use crate::pages::GLOBAL_STATE;

//     leptos::logging::log!("server: entry add_root_dir");
//     // SANITIZE: Reject if not a valid directory
//     // ALSO check access permissions.
//     match GLOBAL_STATE.lock() {
//         Ok(mut state) => {
//             // SANITIZATION
//             // Reject urls without a prefix "/images"
//             // Reject invalid directory names ( within the container directory ).
//             let selected_dir = match url.strip_prefix(IMAGE_PREFIX) {
//                 Some(filename_suffix) => {
//                     state.container_dir.join(filename_suffix)
//                 }
//                 None => {
//                     // malformed input.
//                     return Err(ServerFnError::Args(format!(
//                         "URL must be prefixed with {IMAGE_PREFIX}"
//                     )));
//                 }
//             };

//             if selected_dir.is_dir() {
//                 // reject suspicious input.
//                 return Err(ServerFnError::Args(String::from(
//                     "rejecting selected url",
//                 )));
//             }

//             state.index =
//                 Index::new(selected_dir.clone(), state.container_dir.clone());
//             state.entries = vec![];
//             state.selected_dir = selected_dir;
//             Ok(())
//         }
//         Err(e) => {
//             panic!("/search query - could not unlock {e}");
//         }
//     }
// }

// #[server]
// pub async fn get_selected_dir() -> Result<SelectedUrlResult, ServerFnError> {
//     use crate::pages::GLOBAL_STATE;

//     let selected_dir = match GLOBAL_STATE.lock() {
//         Ok(state) => state.selected_dir.clone(),
//         Err(e) => {
//             panic!("get_root_dir() - could not unlock {e}");
//         }
//     };

//     Ok(SelectedUrlResult {
//         url: selected_dir.display().to_string(),
//     })
// }
