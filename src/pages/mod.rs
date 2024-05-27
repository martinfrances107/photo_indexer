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

    #[derive(Clone, Debug)]
    pub struct GlobalState {
      pub container_dir: PathBuf,
      pub selected_dir: PathBuf,
      pub index: Index,
      pub query: Vec<char>,
      pub entries: Vec<SRElem>,
      // url value fed back to client.
      pub list_url: String,
      pub listed_urls: Vec<String>,
      pub metadata: Option<Vec<Field>>,
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
          list_url: IMAGE_PREFIX.to_string(),
          listed_urls: vec!["a".into(), "b".into(), "c".into()],
          metadata: None,
          selected_dir: root_dir,
        }
      }
    }

    lazy_static! {
      pub static ref GLOBAL_STATE: Mutex<GlobalState> =
      Mutex::new(GlobalState::default());
    }
  }
}
