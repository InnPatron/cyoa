use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

use image;
use find_folder;
use conrod::backend::glium::glium;
use conrod::Ui;
use conrod;
use super::library::StoryHandle;

use popstcl_core::*;
use popstcl_core::internal::Vm;

pub struct Context {
    pub vm_out: RcValue,
    pub game_state: RcValue,
    pub vm: Vm,
    pub assets: StoryAssets,
}

impl Context {
    pub fn new(handle: &StoryHandle, ui: &mut Ui, display: &glium::Display) -> Context {
        use commands::*;

        let mut vm = basic_vm();
        let vm_out = RcValue::new(0.0.into_value());
        let game_state = RcValue::new(0.0.into_value());
        vm.insert_value("display", Value::Cmd(Box::new(Display(vm_out.clone()))));
        vm.insert_value("state", Value::Cmd(Box::new(GameState(game_state.clone()))));

        Context {
            vm_out: vm_out,
            game_state: game_state,
            vm: vm,
            assets: StoryAssets::load(handle, ui, display)
        }
    }
}

pub struct StoryAssets {
    pub images: HashMap<String, glium::texture::Texture2d>,
    pub fonts: HashMap<String, conrod::text::font::Id>,
}

impl StoryAssets {
    pub fn load(handle: &StoryHandle, ui: &mut Ui, display: &glium::Display) -> StoryAssets {
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
        
        StoryAssets {
            images: load_images(images, display),
            fonts: load_fonts(fonts, ui),
        }
    }
}

fn load_images(image_folder: PathBuf, display: &glium::Display) -> HashMap<String, glium::texture::Texture2d> {
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
                map.insert(name, texture);
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
