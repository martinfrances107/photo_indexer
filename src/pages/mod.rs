pub mod about;
pub mod search;

use cfg_if::cfg_if;

pub static IMAGE_PREFIX: &str = r"images/";

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddQuery {
    query: Vec<char>,
}

cfg_if! {
  if #[cfg(feature="ssr")]{

    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::LazyLock;

    use exif::Field;

    use crate::pages::search::SRElem;
    use crate::indexer::Index;


    pub(crate) enum UrlSanitizationError {
      MissingPrefix,
      InvalidDirectory,
    }

    #[derive(Debug)]
    pub(crate) struct DirectorySetError{}

    #[derive(Clone, Debug, Default)]
    pub(crate) struct GlobalState {
      pub entries: Vec<SRElem>,
      pub index: Index,
      pub metadata: Option<Vec<Field>>,
      pub query: Vec<char>,
      pub query_version: usize,

    // PRIVATE: setters ensure all directories must be valid.
    // at time of writing.
    //
    // TODO: Could watch/notify these directories incase
    // another program modified the filesystem.
      container_dir: PathBuf,
      list_dir: PathBuf,
      selected_dir: PathBuf,
    }

    impl GlobalState {
      // Reject urls without a prefix "/images"
      // Reject invalid DIRECTORY names ( within the container directory ).
      fn sanitize_url(&self, url: &str) -> Result<PathBuf, UrlSanitizationError> {

            let list_dir = match url.strip_prefix(IMAGE_PREFIX) {
              Some(filename_suffix) => {
                  self.container_dir.join(filename_suffix)
              }
              None => {
                  // Malformed input.
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

      pub(crate) fn list_dir_set(&mut self, dir: PathBuf) -> Result<(), DirectorySetError>{
        if dir.is_dir() {
          self.list_dir = dir;
          Ok(())
        }
        else {
          Err(DirectorySetError{})
        }
      }

      pub(crate) fn set_list_dir_from_url(&mut self, url: &str) -> Result<(), UrlSanitizationError>{
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

      pub(crate)  fn selected_dir_set(&mut self, dir: PathBuf) -> Result<(), DirectorySetError>{
        if dir.is_dir() {
          self.selected_dir = dir;
          Ok(())
        }
        else {
          Err(DirectorySetError{})
        }
      }

      pub(crate) fn selected_dir(&self) -> PathBuf {
        self.selected_dir.clone()
      }

      pub(crate) fn container_dir_set(&mut self, dir: PathBuf) -> Result<(), DirectorySetError>{
        if dir.is_dir() {
          self.container_dir = dir;
          Ok(())
        }
        else {
          Err(DirectorySetError{})
        }
      }

      pub(crate) fn container_dir(&self) -> PathBuf {
        self.container_dir.clone()
      }

    }

      pub(crate) static GLOBAL_STATE: LazyLock<Mutex<GlobalState>> =
      LazyLock::new(|| {
        Mutex::new(GlobalState::default())
      });
    }

}
