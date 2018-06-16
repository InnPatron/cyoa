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

#[derive(Debug, Clone)]
pub struct GameOption {
    pub display: String,
}
