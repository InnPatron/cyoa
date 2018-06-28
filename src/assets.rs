use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;
use std::fs;
use std::fs::File;

use image;
use find_folder;

use conrod;
use conrod::backend::glium::glium;
use conrod::Ui;
use conrod::image as conimage;
use smpl::{Module, parse_module};

use super::library::StoryHandle;


const SCRIPT_FILE_EXTENSION: &'static str = "smpl";
const IMAGE_FILE_EXTENSION: &'static str = "png";
const FONT_FILE_EXTENSION: &'static str = "ttf";

pub enum AssetErr {
    ImageErr(String),
    FontErr(String),
    ScriptErr(String),
}

impl ::std::fmt::Display for AssetErr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AssetErr::ImageErr(ref s) => write!(f, "Image error:\n{}", s),
            AssetErr::FontErr(ref s) => write!(f, "Font error:\n{}", s),
            AssetErr::ScriptErr(ref s) => write!(f, "Script error:\n{}", s),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ImageHandle {
    pub id: conimage::Id,
    pub h: u32,
    pub w: u32,
}

#[derive(Copy, Clone)]
pub struct FontHandle {
    pub id: conrod::text::font::Id,
}

pub struct StoryAssets {
    pub images: HashMap<String, ImageHandle>,
    pub fonts: HashMap<String, FontHandle>,
    pub scripts: Vec<Module>, 
}

impl StoryAssets {
    pub fn load(handle: &StoryHandle, 
                ui: &mut Ui, 
                display: &glium::Display,
                image_map: &mut conimage::Map<glium::texture::Texture2d>) -> Result<StoryAssets, AssetErr> {
        let assets = find_folder::Search::Kids(1)
            .of(handle.root.clone())
            .for_folder("assets")
            .expect("Unable to read assets folder");
        let images = find_folder::Search::Kids(1)
            .of(assets.clone())
            .for_folder("images")
            .expect("Unable to read images folder");
        let fonts = find_folder::Search::Kids(1)
            .of(assets.clone())
            .for_folder("fonts")
            .expect("Unable to read fonts folder");
        let scripts = find_folder::Search::Kids(1)
            .of(handle.root.clone())
            .for_folder("scripts")
            .expect("Unable to read src folder");
        
        Ok(StoryAssets {
            images: load_images(images, display, image_map)?,
            fonts: load_fonts(fonts, ui)?,
            scripts: load_scripts(scripts)?,
        })
    }
}

fn load_images(image_folder: PathBuf, 
               display: &glium::Display,
               image_map: &mut conimage::Map<glium::texture::Texture2d>) -> Result<HashMap<String, ImageHandle>, AssetErr> {
    let mut map = HashMap::new();
    
    for entry in fs::read_dir(image_folder).unwrap() {
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
                if extension == IMAGE_FILE_EXTENSION {
                    let rgba_image = image::open(path.clone())
                        .map_err(|_| AssetErr::ImageErr(format!("Failed to open image {}", path.display())))?
                        .to_rgba();

                    let dimensions = rgba_image.dimensions();
                    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), dimensions);
                    let texture = glium::texture::Texture2d::new(display, raw_image)
                        .map_err(|_| AssetErr::ImageErr(format!("Failed to create texture for image {}", path.display())))?;

                    let w = texture.get_width();
                    let h = texture.get_height().unwrap();

                    let id = image_map.insert(texture);
                    map.insert(name, ImageHandle { id: id, h:h, w:w });
                }
            }

            // Skip files without proper extension
        }
    }

    Ok(map)
}

fn load_fonts(font_folder: PathBuf, ui: &mut Ui) -> Result<HashMap<String, FontHandle>, AssetErr> {
    let mut map = HashMap::new();
    
    for entry in fs::read_dir(font_folder).unwrap() {
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
                if extension == FONT_FILE_EXTENSION {
                    let id = ui.fonts.insert_from_file(path.clone())
                        .map_err(|e| AssetErr::FontErr(format!("Error loading font {}\n {}", path.display(), e)))?;

                    let font = FontHandle {
                        id: id,
                    };

                    map.insert(name, font);
                }
            }
            // Skip files without proper extension
        }
    }

    Ok(map)
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
