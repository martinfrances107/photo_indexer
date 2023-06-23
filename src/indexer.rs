use core::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;

use leptos::create_rw_signal;
use leptos::RwSignal;
use leptos::Scope;
use leptos::SignalGet;

use exif::Tag;
use log::info;
use uuid::Uuid;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DocLink {
    uuid: Uuid,
    pub de: RwSignal<String>,
    pub doc: RwSignal<String>,
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

#[derive(Clone, Debug)]
pub(crate) struct Index {
    pub doc_links: Vec<DocLink>,
}

/// Will error if the root directory is invalid.
impl Index {
    pub(crate) fn new(cx: Scope, root: &Path) -> Self {
        let extensions = [".jpg", ".gif", ".png", ".jpeg"];
        Self::new_with_extension(cx, root, extensions)
    }

    /// Equivalent to "find . -name *.extension"
    ///
    pub(crate) fn new_with_extension<const N: usize>(
        cx: Scope,
        root: &Path,
        extensions: [&str; N],
    ) -> Self {
        // I think much of this is the same as let glob = glob("**/*.{png, jpg}");
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

        info!("indexing complete about to start server");

        let n_files = files.len();
        info!("{}", format!("n files {}", n_files));

        let mut doc_links: Vec<DocLink> = Vec::with_capacity(n_files);
        for de in files {
            let de_str = de.path().to_str().unwrap().to_string();
            match std::fs::File::open(de.path()) {
                Err(_) => return Self { doc_links: vec![] },
                Ok(file) => {
                    let mut bufreader = std::io::BufReader::new(&file);
                    let exifreader = exif::Reader::new();

                    match exifreader.read_from_container(&mut bufreader) {
                        Ok(exif) => {
                            let fragments = exif
                                .fields()
                                .map(|field| {
                                    match field.tag {
                                        Tag::MakerNote => {
                                            // TODO Do I decode this as u8 strngs???
                                            format!(" {}", field.tag)
                                        }
                                        Tag::UserComment => {
                                            // TODO Do I decode this as u8 strings??
                                            format!(" {}", field.tag)
                                        }

                                        Tag::Sharpness => " ".to_string(),
                                        Tag::Saturation => " ".to_string(),
                                        Tag::ExifVersion => " ".to_string(),
                                        Tag::InteroperabilityVersion => "".to_string(),
                                        Tag::ImageUniqueID => {
                                            // Todo
                                            // ImageUniqueID: "77c6274bd589ad50395891e84a8b673b\"
                                            " ".to_string()
                                        }
                                        _ => {
                                            format!(
                                                " {} {}",
                                                field.tag,
                                                field.display_value().with_unit(&exif)
                                            )
                                        }
                                    }
                                })
                                .collect::<Vec<String>>();
                            let doc = create_rw_signal(cx, fragments.concat());
                            doc_links.push(DocLink {
                                uuid: Uuid::new_v4(),
                                de: create_rw_signal(cx, de_str),
                                doc,
                            });
                        }
                        Err(e) => {
                            // log!("skipping invalid field entry");
                            info!("skipping invalid field entry");
                            eprintln!("---- Skipping invalid field entry ---");
                            eprintln!("{e}");
                            eprintln!("-------------------------------------");
                        }
                    }
                }
            }
        }

        Self { doc_links }
    }
}
