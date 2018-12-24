use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;
use std::fs;
use std::fs::File;

use smpl::{Module, parse_module};

use super::library::StoryHandle;

const SCRIPT_FILE_EXTENSION: &'static str = "smpl";

pub enum AssetErr {
    ScriptErr(String),
}

impl ::std::fmt::Display for AssetErr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AssetErr::ScriptErr(ref s) => write!(f, "Script error:\n{}", s),
        }
    }
}

pub struct StoryAssets {
    pub scripts: Vec<Module>, 
}

impl StoryAssets {
    pub fn load(handle: &StoryHandle) -> Result<StoryAssets, AssetErr> {
        let assets = find_folder::Search::Kids(1)
            .of(handle.root.clone())
            .for_folder("assets")
            .expect("Unable to read assets folder");
        let scripts = find_folder::Search::Kids(1)
            .of(handle.root.clone())
            .for_folder("scripts")
            .expect("Unable to read src folder");
        
        Ok(StoryAssets {
            scripts: load_scripts(scripts)?,
        })
    }
}

fn load_scripts(scripts_folder: PathBuf) -> Result<Vec<Module>, AssetErr> {
    let mut modules = Vec::new();

    for entry in fs::read_dir(scripts_folder).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() == false {
            continue;
        } else {
            let path = entry.path();
            let name = path.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            if let Some(extension) = path.extension() {
                if extension == SCRIPT_FILE_EXTENSION {
                    let mut file = File::open(path.clone())
                        .map_err(|e| AssetErr::ScriptErr(format!("Error opening script {}.\n{}", path.display(), e)))?;

                    let mut contents = String::new();

                    file.read_to_string(&mut contents)
                        .map_err(|e| AssetErr::ScriptErr(format!("Error reading script {}.\n{}", path.display(), e)))?;
                    
                    modules.push(
                        parse_module(&contents)
                        .map_err(|e| AssetErr::ScriptErr(
                                format!("Failed to parse script {}.\n{:?}", 
                                        path.display(), 
                                        e
                                        )
                                )
                            )?
                        );
                }
            }

            // Skip files without proper extension
        }
    }

    Ok(modules)
}
