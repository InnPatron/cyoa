use std::rc::Rc;
use std::cell:: RefCell;

use failure::Error;
use smpl::interpreter::{AVM, Struct as SmplStruct, Value, FnHandle, Std, VmModule};

use crate::library::StoryHandle;
use crate::assets::*;
use crate::script_lib;

pub const MAIN_MODULE: &'static str = "main";
pub const INIT_FN: &'static str = "start";

#[derive(Debug, Fail)]
pub enum GameErr {
    #[fail(display = "{}", _0)]
    AssetErr(AssetErr),
    #[fail(display = "Script Error: {}", _0)]
    ScriptErr(String),
}

impl From<AssetErr> for GameErr {
    fn from(e: AssetErr) -> GameErr {
        GameErr::AssetErr(e)
    }
}

pub struct GameInstance {
    vm: AVM,
    context: Context,
}

impl GameInstance {
    pub fn new(handle: &StoryHandle) -> Result<GameInstance, Error> {

        let assets = StoryAssets::load(handle)?;

        let mut scripts = assets.scripts
            .into_iter()
            .map(|module| VmModule::new(module))
            .collect::<Vec<_>>();

        scripts.push(script_lib::vm_module());

        let vm = AVM::new(Std::std(), scripts)
            .map_err(|e| GameErr::ScriptErr(format!("Failed to start VM.\n{:?}", e)))?;

        let init = vm.query_module(MAIN_MODULE, INIT_FN)
            .map_err(|e| GameErr::ScriptErr(format!("Failed to query {} for {}\n {:?}",
                                                    MAIN_MODULE,
                                                    INIT_FN,
                                                    e)))?
            .ok_or(GameErr::ScriptErr(format!("Failed to find {} in module {}",
                                              INIT_FN,
                                              MAIN_MODULE)))?;

        // Assume fn init() is Fn(Context) -> Context
        let result = vm.eval_fn_args_sync(init, None)?;
        let result = match result {
            Value::Struct(s) => s,

            _ => return Err(GameErr::ScriptErr(format!("{} should return a context",
                                                       INIT_FN))
                            .into()),
        };

        let context = Context(result);
        
        Ok(GameInstance {
            vm: vm,
            context: context,
        })
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn execute_choice(&mut self, choice: &Choice) -> Result<(), Error> {
        // Assume choice handlers are Fn(Context) -> Context
        let mut temp = SmplStruct::new();
        ::std::mem::swap(&mut temp, &mut self.context.0);
        let result = self
            .vm.eval_fn_args_sync(choice.handle(), 
                                  Some(vec![Value::Struct(temp)]))?;

        let result = irmatch!(result; Value::Struct(s) => s);
        self.context = Context(result);

        Ok(())
    }
}

pub struct Context(SmplStruct);

impl Context {

    pub fn state(&self) -> i32 {
        irmatch!(self.0.get_field(script_lib::CTXT_STATE)
                 .expect(&format!("Ctxt missing '{}' field", script_lib::CTXT_STATE));
                 Value::Int(i) => i)
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
