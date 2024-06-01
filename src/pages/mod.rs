pub mod about;
pub mod search;

use cfg_if::cfg_if;

pub static IMAGE_PREFIX: &str = r"images/";

cfg_if! {
  if #[cfg(feature="ssr")]{

    use std::path::PathBuf;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use exif::Field;

    use crate::pages::search::SRElem;
    use crate::indexer::Index;

    pub(crate) enum UrlSanitizationError {
      MissingPrefix,
      InvalidDirectory,
    }

    #[derive(Clone, Debug)]
    pub(crate) struct GlobalState {
      pub container_dir: PathBuf,
      pub selected_dir: PathBuf,
      pub index: Index,
      pub query: Vec<char>,
      pub entries: Vec<SRElem>,
      list_dir: PathBuf,
      pub listed_urls: Vec<String>,
      pub metadata: Option<Vec<Field>>,
    }

    // Getters and setters for
    impl GlobalState {
      fn sanitize_url(&self, url: String) -> Result<PathBuf, UrlSanitizationError> {
           // SANITIZATION
            // Reject urls without a prefix "/images"
            // Reject invalid DIRECTORY names ( within the container directory ).
            let list_dir = match url.strip_prefix(IMAGE_PREFIX) {
              Some(filename_suffix) => {
                  PathBuf::from(self.container_dir.join(filename_suffix))
              }
              None => {
                  // malformed input.
                  return Err(UrlSanitizationError::MissingPrefix)
              }
          };

          // TODO stop potential leak.
          // SECURITY: MUST confirm directory is a SUB directory of the container.
          if !list_dir.is_dir() {
              // Reject suspicious input.
              return Err(UrlSanitizationError::InvalidDirectory);
          }

          Ok(list_dir)
      }

      pub(crate) fn set_list_dir_from_url(&mut self, url: String) -> Result<(), UrlSanitizationError>{
        match self.sanitize_url(url) {
          Ok(dir) => {
            self.list_dir = dir;
            Ok(())
          }
          Err(e) => {
            Err(e)
          }
        }
      }

      pub(crate) fn list_dir(&self) -> PathBuf {
        self.list_dir.clone()
      }

      // TODO MISSING: set_selected_dir_from_url()
      // TODO MISSING: selected_dir()
    }

    impl Default for GlobalState {
      fn default() -> Self {
        // Initialisation value required for the period before initialisation
        // from command line arguments.
        let root_dir = PathBuf::from("../exif-samples") ;

        Self {
          container_dir: root_dir.clone(),
          index: Index::new(root_dir.clone(), root_dir.clone()),
          query: vec![],
          entries: vec![],
          list_dir: root_dir.clone(),
          listed_urls: vec!["default a".into()],
          metadata: None,
          selected_dir: root_dir,
        }
      }
    }

    lazy_static! {
      pub(crate) static ref GLOBAL_STATE: Mutex<GlobalState> =
      Mutex::new(GlobalState::default());
    }
  }
}
