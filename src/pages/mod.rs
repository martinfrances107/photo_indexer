pub mod about;
pub mod search;

use cfg_if::cfg_if;

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
      pub root_dir: PathBuf,
      pub index: Index,
      pub query: Vec<char>,
      pub entries: Vec<SRElem>,
      pub metadata: Option<Vec<Field>>,
    }


    impl Default for GlobalState {
      fn default() -> Self {
        let root_dir = PathBuf::from("../exif-samples");

        Self {
          index: Index::new(&root_dir),
          root_dir,
          query: vec![],
          entries: vec![],
          metadata: None,
        }
      }
    }

    lazy_static! {
      pub static ref GLOBAL_STATE: Mutex<GlobalState> =
      Mutex::new(GlobalState::default());
    }
  }
}
