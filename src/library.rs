use std::path::PathBuf;
use std::fs;
use std::io::Read;
use toml;

use find_folder;

const MANIFEST_NAME: &'static str = "cyoa.toml";

pub struct StoryHandle {
    pub root: PathBuf,
    pub metadata: Metadata
}

#[derive(Deserialize)]
pub struct Metadata {
    pub name: String,
    pub author: Option<String>,
    pub version: String,
    pub notes: Option<String>,
    pub main: String,
}

pub fn scan_library() -> Vec<StoryHandle> {
    let mut library = Vec::new();
    let libfolder = find_folder::Search::ParentsThenKids(3, 3).for_folder("library").expect("Could not find library");
    for entry in fs::read_dir(libfolder).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let mut manifest_path = entry.path();
            manifest_path.push(MANIFEST_NAME);
            if manifest_path.exists() {
                let metadata = {
                    let mut buffer = String::new();
                    let mut manifest = fs::File::open(manifest_path).unwrap();
                    manifest.read_to_string(&mut buffer).unwrap();
                    buffer
                };
                if let Ok(metadata) = toml::from_str(&metadata) {
                    library.push(StoryHandle {
                        root: entry.path(),
                        metadata: metadata,
                    });
                }
            }
        }
    }
    library
}
