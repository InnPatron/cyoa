use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::fs::File;

use image;
use find_folder;
use conrod::backend::glium::glium;
use conrod::Ui;
use conrod;
use conrod::image as conimage;
use super::library::StoryHandle;

pub struct Context {
    pub assets: StoryAssets,
}

impl Context {
    pub fn new(handle: &StoryHandle, 
               ui: &mut Ui, 
               display: &glium::Display, 
               image_map: &mut conimage::Map<glium::texture::Texture2d>) -> Context {

        let assets = StoryAssets::load(handle, ui, display, image_map);
        unimplemented!();
    }
}

pub struct StoryAssets {
    pub images: HashMap<String, ImageHandle>,
    pub fonts: HashMap<String, conrod::text::font::Id>,
    pub scripts: Rc<HashMap<String, File>>,
}

impl StoryAssets {
    pub fn load(handle: &StoryHandle, 
                ui: &mut Ui, 
                display: &glium::Display,
                image_map: &mut conimage::Map<glium::texture::Texture2d>) -> StoryAssets {
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
        
        StoryAssets {
            images: load_images(images, display, image_map),
            fonts: load_fonts(fonts, ui),
            scripts: Rc::new(load_scripts(scripts)),
        }
    }
}

fn load_images(image_folder: PathBuf, 
               display: &glium::Display,
               image_map: &mut conimage::Map<glium::texture::Texture2d>) -> HashMap<String, ImageHandle> {
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
            let extension = path.extension().unwrap();
            if extension == "png" {
                let rgba_image = image::open(path.clone()).unwrap().to_rgba();
                let dimensions = rgba_image.dimensions();
                let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&rgba_image.into_raw(), dimensions);
                let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
                let w = texture.get_width();
                let h = texture.get_height().unwrap();
                let id = image_map.insert(texture);
                map.insert(name, ImageHandle { id: id, h:h, w:w });
            }
        }
    }

    map
}

fn load_fonts(font_folder: PathBuf, ui: &mut Ui) -> HashMap<String, conrod::text::font::Id> {
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
            let extension = path.extension().unwrap();
            if extension == "ttf" {
                let id = ui.fonts.insert_from_file(path.clone()).unwrap();
                map.insert(name, id);
            }
        }
    }

    map
}

fn load_scripts(scripts_folder: PathBuf) -> HashMap<String, File> {
    let mut map = HashMap::new();
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
            let extension = path.extension().unwrap();
            if extension == "popstcl" {
                let file = File::open(path.clone()).unwrap();
                map.insert(name, file);
            }
        }
    }
    map
}

pub struct ImageHandle {
    pub id: conimage::Id,
    pub h: u32,
    pub w: u32,
}

#[derive(Debug, Clone)]
pub struct GameOption {
    pub display: String,
}
