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

#[derive(Clone, Debug, PartialEq)]
pub struct Index {
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
    #[allow(clippy::cognitive_complexity)]
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
                            // Unwrap here side steps unicode issues.
                            // TODI can I strip off absolute path
                            // and extension and just have a file_stem?
                            let mut content =
                                String::from(de.path().to_str().unwrap());
                            content.push(' ');
                            for field in exif.fields() {
                                // MakerNote is a proprietary binary format block
                                // do not pass to indexer.
                                if field.tag != Tag::MakerNote {
                                    content.push_str(&format!(
                                        "{}",
                                        field.display_value()
                                    ));
                                }

                                // Special case ImageDescription
                                // Will be displayed before other metadata.
                                if field.tag == Tag::ImageDescription {
                                    // TODO at this point a valid display_value() is
                                    // "\"          \""
                                    // Must strip out white space and escaped values like \"
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

                            if !content.is_empty() {
                                model.add_document(
                                    filename,
                                    SystemTime::now(),
                                    &content.chars().collect::<Vec<char>>(),
                                );
                            }
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

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
/// Can I refactor?
/// Drop the ROOT_DIR
/// Inject two simulated files.
/// then assert we can see only one returned by search.
mod test {

    static ROOT_DIR: &str = "/home/martin/build/exif-samples";
    extern crate pretty_assertions;

    use super::Index;
    use std::path::Path;
    use std::path::PathBuf;

    // A query the elicits no results.
    #[test]
    fn no_results() {
        let path = Path::new(ROOT_DIR);
        let index = Index::new(path);

        let query = "socks";

        let sq = query.chars().collect::<Vec<char>>();

        assert!(index.model.search_query(&sq).is_empty());
    }

    // This test is broken these two have the same rank
    // and from test to test can change position.
    // canon_hdr_YES.jpg
    // canon_hdr_NO.jpg
    #[ignore]
    #[test]
    fn found_in_filename() {
        let path = Path::new(ROOT_DIR);
        let index = Index::new(path);

        let query = "hdr";

        let sq = query.chars().collect::<Vec<char>>();

        let result = index.model.search_query(&sq);

        let expected = vec![
          (PathBuf::from("/home/martin/build/exif-samples/jpg/Sony_HDR-HC3.jpg"), 0.026654188),
          (PathBuf::from("/home/martin/build/exif-samples/jpg/hdr/canon_hdr_YES.jpg"), 0.015657004),
          (PathBuf::from("/home/martin/build/exif-samples/jpg/hdr/canon_hdr_NO.jpg"), 0.015657004),
          (PathBuf::from("/home/martin/build/exif-samples/jpg/hdr/iphone_hdr_YES.jpg"), 0.009995321),
          (PathBuf::from("/home/martin/build/exif-samples/jpg/hdr/iphone_hdr_NO.jpg"), 0.009906866),
          (PathBuf::from("/home/martin/build/exif-samples/jpg/mobile/HMD_Nokia_8.3_5G_hdr.jpg"), 0.0073168357)]
        ;
        assert_eq!(result, expected);
    }

    #[test]
    fn found_in_description() {
        let path = Path::new(ROOT_DIR);
        let index = Index::new(path);

        // Other words berlin, chinook.
        let query = "Chinook";

        let sq = query.chars().collect::<Vec<char>>();

        let result = index.model.search_query(&sq);

        // assert_eq!(result.len(), 1);

        let expected = vec![(
            PathBuf::from(
                "/home/martin/build/exif-samples/jpg/long_description.jpg",
            ),
            0.01150077_f32,
        )];
        assert_eq!(result, expected);
    }

    #[ignore]
    #[test]
    fn found_in_metadata() {
        let path = Path::new(ROOT_DIR);
        let index = Index::new(path);

        // Other words sanyo, digital
        let query = "olymupus";

        let sq = query.chars().collect::<Vec<char>>();

        let result = index.model.search_query(&sq);

        let expected = vec![(PathBuf::from("hello"), 1_f32)];
        assert_eq!(result, expected);
    }

    // Must find by year
    // DD/MM/YY
    #[ignore]
    #[test]
    fn date() {
        let path = Path::new(ROOT_DIR);
        let index = Index::new(path);

        let query = "2018";

        let sq = query.chars().collect::<Vec<char>>();

        let result = index.model.search_query(&sq);

        let expected = vec![(PathBuf::from("hello"), 1_f32)];
        assert_eq!(result, expected);
    }
}
