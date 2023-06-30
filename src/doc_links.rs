use core::fmt::Display;
use std::fmt::Formatter;

use uuid::Uuid;

use leptos::RwSignal;
use leptos::SignalGet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DocLink {
    uuid: Uuid,
    pub de: RwSignal<String>,
    /// doc: Merges all exif data, this is the string
    /// from which the index computes TF/IDF
    pub doc: RwSignal<String>,
    /// filename: The last section of the fully qualifed path
    /// if
    ///
    /// Path =  a/b/foo/bar.txt
    ///
    /// then
    ///
    /// filename  = bar.txt
    pub filename: RwSignal<String>,
    /// The EXIF tag "ImageDescription" appears under the image, if present
    pub description: RwSignal<String>,
}

/// Read only field, so only getter
impl DocLink {
    #[inline]
    pub(crate) fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl Display for DocLink {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "{:?}", self.de.get())?;
        writeln!(f, "{:?}", self.doc)
    }
}
