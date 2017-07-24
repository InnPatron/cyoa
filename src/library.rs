use std::path::PathBuf;

const manifest_name: &'static str = "cyoa";

pub struct Library(Vec<Story>);

pub struct Story {
    root: PathBuf,
    metadata: Metadata
}

#[derive(Deserialize)]
pub struct Metadata {
    name: String,
    author: Option<String>,
    version: String,
    notes: Option<String>

}
