use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use exif::Field;
use exif::Tag;

use seroost_lib::model::Model;
use tracing::info;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Index {
    // pub doc_links: Vec<DocLink>,
    pub description_store: HashMap<PathBuf, String>,
    pub model: Model,
    pub md_store: HashMap<PathBuf, Vec<Field>>,
}

/// Will error if the root directory is invalid.
impl Index {
    pub(crate) fn new(root: &Path) -> Self {
        let extensions = [".jpg", ".gif", ".png", ".jpeg"];
        Self::new_with_extension(root, extensions)
    }

    /// Equivalent to "find . -name *.extension"
    pub(crate) fn new_with_extension<const N: usize>(
        root: &Path,
        extensions: [&str; N],
    ) -> Self {
        // TODO If availble load model from file.
        // let model: Arc<Mutex<Model>> = Arc::new(Mutex::new(Default::default()));
        let mut model = Model::default();
        let mut md_store = HashMap::default();
        let mut description_store = HashMap::default();

        //this is the same as let glob = glob("**/*.{png, jpg}");
        let image_entries = WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
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

        info!("Indexing complete: About to start server");

        let n_files = image_entries.len();
        info!("{}", format!("n files {}", n_files));

        // TODO Make multithreaded
        // Given a list of files spawn a new thread for each file loading.
        for de in image_entries {
            // Can ALWAYS unwrap the file inside the loop containing valid filenames?
            let filename = de.path().to_path_buf();
            match std::fs::File::open(de.path()) {
                Err(_) => {
                    return Self {
                        description_store,
                        model,
                        md_store,
                    }
                }
                Ok(file) => {
                    let mut bufreader = std::io::BufReader::new(&file);
                    let exifreader = exif::Reader::new();

                    match exifreader.read_from_container(&mut bufreader) {
                        Ok(exif) => {
                            let mut content = String::new();
                            for field in exif.fields() {
                                // MakerNote is a proprietary binary format block
                                // do not pass to indexer.
                                if field.tag != Tag::MakerNote {
                                    let dv =
                                        format!("{}", field.display_value());
                                    content.push_str(&dv);
                                }

                                // Special case ImageDescription
                                // Will be displayed before other metadata.
                                if field.tag == Tag::ImageDescription {
                                    description_store.insert(
                                        filename.clone(),
                                        format!("{}", field.display_value()),
                                    );
                                }
                            }
                            md_store.insert(
                                filename.clone(),
                                // Strip ImageDescription from meta data list destined for display.
                                // ImageDescription will be shown before the metadata.
                                exif.fields()
                                    .filter(|&f| f.tag != Tag::ImageDescription)
                                    .filter(|&f| f.tag != Tag::MakerNote)
                                    .cloned()
                                    .collect(),
                            );

                            model.add_document(
                                filename,
                                SystemTime::now(),
                                &content.chars().collect::<Vec<char>>(),
                            );
                        }
                        Err(e) => {
                            info!("skipping invalid field entry");
                            eprintln!("---- Skipping invalid field entry ---");
                            eprintln!("{e}");
                            eprintln!("-------------------------------------");
                        }
                    }
                }
            }
        }

        Self {
            description_store,
            model,
            md_store,
        }
    }
}
