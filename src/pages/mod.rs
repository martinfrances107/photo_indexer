pub mod about;
pub mod search;

use std::path::PathBuf;
use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::indexer::Index;

#[derive(Clone, Debug)]
pub struct GlobalState {
    pub root_dir: PathBuf,
    pub index: Index,
    pub query: Vec<char>,
    pub entries: Vec<(usize, (PathBuf, f32))>,
}

impl Default for GlobalState {
    fn default() -> Self {
        let root_dir =
            PathBuf::from("../exif-samples");

        Self {
            index: Index::new(&root_dir),
            root_dir,
            query: vec![],
            entries: vec![],
        }
    }
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<GlobalState> =
        Mutex::new(GlobalState::default());
}
