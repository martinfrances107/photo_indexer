use core::f32::consts::E;
use std::ffi::OsStr;
// use std::fs::{self, DirEntry};
use std::io::Error;
use std::path::{Path, PathBuf};

use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct Indexer {
    files: Vec<DirEntry>,
}

impl Indexer {
    pub(crate) fn new(root: &Path) -> Result<Self, Error> {
        let extensions = vec![".jpg", ".gif", ".png", ".jpeg"];
        Self::new_with_extension(root, extensions)
    }

    /// Equivalent to "find . -name *.extension"
    ///
    /// find ""./" -regex ".*\.\(jpg\|gif\|png\|jpeg\)"
    pub(crate) fn new_with_extension(root: &Path, extensions: Vec<&str>) -> Result<Self, Error> {
        let files = WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let f_name = entry.file_name().to_string_lossy();
                for extension in &extensions {
                    if f_name.ends_with(extension) {
                        return Some(entry);
                    }
                }

                None
            })
            .collect::<Vec<DirEntry>>();
        Ok(Self { files })
    }
}
