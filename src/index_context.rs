use leptos::ReadSignal;

use crate::indexer::Index;

#[derive(Copy, Clone)]
pub struct IndexContext {
    pub index: ReadSignal<Index>,
}
