use std::rc::Rc;
use std::cell::RefCell;

use failure::Error;

use smpl::{parse_module, UnparsedModule};
use smpl::interpreter::*;

pub const STATE_RUN: i32 = 0;
pub const STATE_END: i32 = 1;

const RT_CHOICE: &'static str = "choice";
const RT_CLEAR_CHOICES: &'static str = "clear_choices";

const RT_INIT_CTXT: &'static str = "init_ctxt";
const RT_NEW_DATA: &'static str = "new_data";

const RT_SET_STATE: &'static str = "set_state";

const RT_SET_DATA: &'static str = "set_data";
const RT_GET_DATA: &'static str = "get_data";

pub const CTXT_STATE: &'static str = "state";
pub const CTXT_CHOICE: &'static str = "choice_list";

pub const CHOICE_HANDLE: &'static str = "handle";
pub const CHOICE_DISPLAY: &'static str = "display";


const RT_LIB: &'static str = include_str!("rt.smpl");

pub fn vm_module() -> VmModule {
    let parsed = parse_module(UnparsedModule::anonymous(RT_LIB)).unwrap();
    let vm_module = VmModule::new(parsed)
        .add_builtin(RT_INIT_CTXT, init_ctxt)
        .add_builtin(RT_CHOICE, choice)
        .add_builtin(RT_CLEAR_CHOICES, clear_choices)
        .add_builtin(RT_SET_STATE, set_state)
        .add_builtin(RT_SET_DATA, set_data)
        .add_builtin(RT_GET_DATA, get_data)
        .add_builtin(RT_NEW_DATA, new_data)
    ;

    vm_module
}

fn init_ctxt(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let c = args.unwrap().pop().unwrap();
    let mut c = irmatch!(c; Value::Struct(c) => c);
    c.set_field(CTXT_STATE.to_string(), Value::Int(STATE_RUN));
    c.set_field(CTXT_CHOICE.to_string(), Value::Array(Vec::new()));

    Ok(Value::Struct(c))
}

fn choice(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();
    let display = args.pop().unwrap();
    let handler = args.pop().unwrap();
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let list = context.ref_field(CTXT_CHOICE).unwrap();
    let mut list = list.borrow_mut();
    let list = irmatch!(*list; Value::Array(ref mut vec) => vec);

    let mut choice = Struct::new();
    choice.set_field(CHOICE_HANDLE.to_owned(), handler);
    choice.set_field(CHOICE_DISPLAY.to_owned(), display);

    list.push(Rc::new(RefCell::new(Value::Struct(choice))));

    Ok(Value::Struct(context))
}

fn clear_choices(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let list = context.ref_field(CTXT_CHOICE).unwrap();
    let mut list = list.borrow_mut();
    let list = irmatch!(*list; Value::Array(ref mut vec) => vec);

    list.clear();

    Ok(Value::Struct(context))
}

fn set_state(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let new_state = args.pop().unwrap();
    let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    context.set_field(CTXT_STATE.to_owned(), new_state);

    Ok(Value::Struct(context))
}

fn new_data(_args: Option<Vec<Value>>) -> Result<Value, Error> {
    Ok(Value::Struct(Struct::new()))
}

fn set_data(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let to_set = args.pop().unwrap();
    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let mut data_s = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    data_s.set_field(name, to_set);

    Ok(Value::Struct(data_s))
}

fn get_data(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let data = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let value = data.get_field(&name)
        .expect(&format!("Unknown flag {}", name));

    Ok(value)
}
