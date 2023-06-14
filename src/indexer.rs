use core::fmt;
use core::fmt::Display;

use std::io::Error;
use std::path::Path;

use exif::Tag;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Debug)]
pub(crate) struct DocLink {
    pub de: DirEntry,
    pub doc: String,
}

impl Display for DocLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {:?} {:?} ", self.de.path(), self.doc)
    }
}

#[derive(Debug)]
pub(crate) struct Indexer {
    doc_links: Vec<DocLink>,
}

impl Indexer {
    pub(crate) fn new(root: &Path) -> Result<Self, Error> {
        let extensions = [".jpg", ".gif", ".png", ".jpeg"];
        Self::new_with_extension(root, extensions)
    }

    /// Equivalent to "find . -name *.extension"
    ///
    pub(crate) fn new_with_extension<const N: usize>(
        root: &Path,
        extensions: [&str; N],
    ) -> Result<Self, Error> {
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

        let n_files = files.len();
        let mut doc_links: Vec<DocLink> = Vec::with_capacity(n_files);
        for de in files {
            let file = std::fs::File::open(de.path())?;
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
                    let doc = fragments.concat();
                    doc_links.push(DocLink { de, doc });
                }
                Err(e) => {
                    eprintln!("---- Skipping invalid field entry ---");
                    eprintln!("{e}");
                    eprintln!("-------------------------------------");
                }
            }
        }

        // println!("{:?}", docs);
        Ok(Self { doc_links })
    }
}
