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

use smpl::interpreter::{VM, Struct as SmplStruct, Value, FnHandle};

use library::StoryHandle;
use assets::*;
use script_lib;

pub const MAIN_MODULE: &'static str = "main";
pub const INIT_FN: &'static str = "start";

pub enum GameErr {
    AssetErr(AssetErr),
    ScriptErr(String),
}

impl ::std::fmt::Display for GameErr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GameErr::AssetErr(ref e) => write!(f, "{}", e),
            GameErr::ScriptErr(ref s) => write!(f, "Script error:\n{}", s),
        }
    }
}

impl From<AssetErr> for GameErr {
    fn from(e: AssetErr) -> GameErr {
        GameErr::AssetErr(e)
    }
}

pub struct GameInstance {
    images: HashMap<String, ImageHandle>,
    fonts: HashMap<String, FontHandle>,
    vm: VM,
    context: Context,
}

impl GameInstance {
    pub fn new(handle: &StoryHandle, 
               ui: &mut Ui, 
               display: &glium::Display, 
               image_map: &mut conimage::Map<glium::texture::Texture2d>) -> Result<GameInstance, GameErr> {

        let assets = StoryAssets::load(handle, ui, display, image_map)?;

        let mut scripts = assets.scripts;
        script_lib::include(&mut scripts);
        
        let mut vm = VM::new(scripts)
            .map_err(|e| GameErr::ScriptErr(format!("Failed to start VM.\n{:?}", e)))?;

        script_lib::map_builtins(&mut vm);

        let init = vm.query_module(MAIN_MODULE, INIT_FN)
            .map_err(|e| GameErr::ScriptErr(format!("Failed to query {} for {}\n {:?}",
                                                    MAIN_MODULE,
                                                    INIT_FN,
                                                    e)))?
            .ok_or(GameErr::ScriptErr(format!("Failed to find {} in module {}",
                                              INIT_FN,
                                              MAIN_MODULE)))?;

        let context = script_lib::new_context();

        // Assume fn init() is Fn(Context) -> Context
        let result = vm.eval_fn_args(init, vec![Value::Struct(context)]);
        let result = irmatch!(result; Value::Struct(s) => s);

        let context = Context(result);
        
        Ok(GameInstance {
            images: assets.images,
            fonts: assets.fonts,
            vm: vm,
            context: context,
        })
    }

    pub fn get_image(&self, name: &str) -> Option<ImageHandle> {
        self.images.get(name).map(|h| h.clone())
    }

    pub fn get_font(&self, name: &str) -> Option<FontHandle> {
        self.fonts.get(name).map(|h| h.clone()) 
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn execute_choice(&mut self, choice: &Choice) {
        // Assume choice handlers are Fn(Context) -> Context
        let mut temp = SmplStruct::new();
        ::std::mem::swap(&mut temp, &mut self.context.0);
        let result = self.vm.eval_fn_args(choice.handle(), vec![Value::Struct(temp)]);

        let result = irmatch!(result; Value::Struct(s) => s);
        self.context = Context(result);
    }
}

pub struct Context(SmplStruct);

impl Context {

    pub fn state(&self) -> i32 {
        irmatch!(self.0.get_field(script_lib::CTXT_STATE)
                 .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_STATE));
                 Value::Int(i) => i)
    }

    pub fn background(&self) -> String {
        irmatch!(self.0.get_field(script_lib::CTXT_BACKGROUND)
                 .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_BACKGROUND));
                 Value::String(s) => s)
    }

    pub fn display(&self) -> String {
        irmatch!(self.0.get_field(script_lib::CTXT_DISPLAY)
                 .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_DISPLAY));
                 Value::String(s) => s)
    }

    pub fn font(&self) -> String {
        irmatch!(self.0.get_field(script_lib::CTXT_FONT)
                 .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_FONT));
                 Value::String(s) => s)
    }

    pub fn choices(&self) -> Vec<Choice> {
        let choices = irmatch!(self.0.get_field(script_lib::CTXT_CHOICE)
                               .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_CHOICE));
                               Value::Array(a) => a);

        choices.into_iter()
            .map(|smpl_struct| Choice(smpl_struct.clone()))
            .collect()
    }

    pub fn set_state(&mut self, state: i32) {
        self.0.set_field(script_lib::CTXT_STATE.to_owned(), Value::Int(state));
    }
}

#[derive(Debug, Clone)]
pub struct Choice(Rc<RefCell<Value>>);

impl Choice {
    pub fn display(&self) -> String {
        let choice = self.0.borrow();
        let choice = irmatch!(*choice; Value::Struct(ref s) => s);
        irmatch!(choice.get_field(script_lib::CHOICE_DISPLAY)
                 .expect(&format!("Choice missing '{}' field", script_lib::CHOICE_DISPLAY));
                 Value::String(s) => s)
    }

    pub fn handle(&self) -> FnHandle {
        let choice = self.0.borrow();
        let choice = irmatch!(*choice; Value::Struct(ref s) => s);
        irmatch!(choice.get_field(script_lib::CHOICE_HANDLE)
                 .expect(&format!("Choice missing '{}' field", script_lib::CHOICE_HANDLE));
                 Value::Function(f) => f)
    }
}
