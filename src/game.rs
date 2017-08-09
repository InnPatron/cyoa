use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::cell::{Cell, RefCell};

use image;
use find_folder;
use conrod::backend::glium::glium;
use conrod::Ui;
use conrod;
use conrod::image as conimage;
use super::library::StoryHandle;

use popstcl_core::*;
use popstcl_core::internal::Vm;
use commands::*;

pub struct Context {
    pub vm: RefCell<Vm>,
    pub assets: StoryAssets,
    pub pipes: Rc<VmPipes>
}

impl Context {
    pub fn new(handle: &StoryHandle, 
               ui: &mut Ui, 
               display: &glium::Display, 
               image_map: &mut conimage::Map<glium::texture::Texture2d>) -> Context {
        use commands::*;

        let pipes = VmPipes {
            vm_out: Default::default(),
            game_state: Default::default(),
            options: Default::default(),
            font: Default::default()
        };
        let pipes = Rc::new(pipes);
        let mut vm = Vm::new_with_main_module(cyoa_env(pipes.clone()).consume());

        Context {
            vm: RefCell::new(vm),
            assets: StoryAssets::load(handle, ui, display, image_map),
            pipes: pipes 
        }
    }
}

#[derive(Debug)]
pub struct VmPipes {
    pub vm_out: RefCell<String>,
    pub game_state: Cell<i32>,
    pub options: RefCell<Vec<GameOption>>,
    pub font: RefCell<String>,
}

pub fn cyoa_env(pipes: Rc<VmPipes>) -> EnvBuilder {
    let mut builder = std_env();
    builder.insert_value("display", Value::Cmd(Box::new(Display(pipes.clone()))));
    builder.insert_value("state", Value::Cmd(Box::new(GameState(pipes.clone()))));
    builder.insert_value("cyoa", Value::Cmd(Box::new(NewMod(pipes.clone()))));
    builder.insert_value("option", Value::Cmd(Box::new(AddOption(pipes.clone()))));
    builder.insert_value("clear-options", Value::Cmd(Box::new(ClearOptions(pipes.clone()))));
    builder.insert_value("font", Value::Cmd(Box::new(Font(pipes.clone()))));
    builder
}

pub struct StoryAssets {
    pub images: HashMap<String, ImageHandle>,
    pub fonts: HashMap<String, conrod::text::font::Id>,
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
        
        StoryAssets {
            images: load_images(images, display, image_map),
            fonts: load_fonts(fonts, ui),
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

pub struct ImageHandle {
    pub id: conimage::Id,
    pub h: u32,
    pub w: u32,
}

#[derive(Debug, Clone)]
pub struct GameOption {
    pub display: String,
    pub consequence: Program,
}
