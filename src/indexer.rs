use std::path::Path;
use std::time::SystemTime;

use exif::Tag;

use seroost_lib::model::Model;
use tracing::info;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub(crate) struct Index {
    // pub doc_links: Vec<DocLink>,
    pub model: Model,
}

/// Will error if the root directory is invalid.
impl Index {
    pub(crate) fn new(root: &Path) -> Self {
        let extensions = [".jpg", ".gif", ".png", ".jpeg"];
        Self::new_with_extension(root, extensions)
    }

    /// Equivalent to "find . -name *.extension"
    ///
    pub(crate) fn new_with_extension<const N: usize>(root: &Path, extensions: [&str; N]) -> Self {
        // TODO If availble load model from file.
        // let model: Arc<Mutex<Model>> = Arc::new(Mutex::new(Default::default()));
        let mut model = Model::default();

        //this is the same as let glob = glob("**/*.{png, jpg}");
        let image_entries = WalkDir::new(root)
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
                        model: Model::default(),
                    }
                }
                Ok(file) => {
                    let mut bufreader = std::io::BufReader::new(&file);
                    let exifreader = exif::Reader::new();

                    // info!("File {:#?}", file);
                    match exifreader.read_from_container(&mut bufreader) {
                        Ok(exif) => {
                            let mut description = String::from("No description");
                            let mut content: Vec<char> = vec![];
                            for field in exif.fields() {
                                match field.tag {
                                    Tag::ImageDescription => {
                                        description = format!("{}", field.display_value());
                                        let mut other = format!(
                                            " {} {}",
                                            field.tag,
                                            field.display_value().with_unit(&exif)
                                        )
                                        .chars()
                                        .collect();
                                        content.append(&mut other);
                                    }
                                    Tag::MakerNote => {
                                        // TODO Do I decode this as u8 strings???
                                        let mut other = format!(" {}", field.tag).chars().collect();
                                        content.append(&mut other);
                                    }
                                    Tag::UserComment => {
                                        // TODO Do I decode this as u8 strings??
                                        let mut other =
                                            format!(" {} ", field.tag).chars().collect();
                                        // info!("UserComment {}", other);
                                        content.append(&mut other);
                                    }

                                    Tag::Sharpness => {
                                        let mut other = " sharpess".chars().collect();
                                        content.append(&mut other);
                                    }
                                    Tag::Saturation => {
                                        let mut other = " sharpess".chars().collect();
                                        content.append(&mut other);
                                    }
                                    Tag::ExifVersion => {
                                        let mut other = " version".chars().collect();
                                        content.append(&mut other);
                                    }
                                    Tag::InteroperabilityVersion => {
                                        // let mut other = " Interop !!! ".to_string();
                                        // content.append(other);
                                    }
                                    Tag::ImageUniqueID => {
                                        // Todo
                                        // ImageUniqueID: "77c6274bd589ad50395891e84a8b673b\"

                                        // let mut other = " ID !!! ".into();
                                        // content.append(&other);
                                        //
                                        // seroost::Lexer how are alphanumerics handled?
                                    }
                                    _ => {
                                        let mut other = format!(
                                            " {} {}",
                                            field.tag,
                                            field.display_value().with_unit(&exif)
                                        )
                                        .chars()
                                        .collect();
                                        content.append(&mut other);
                                    }
                                }
                            }

                            model.add_document(filename, SystemTime::now(), &content);
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

        Self { model }
    }
}
