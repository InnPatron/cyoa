use std::rc::Rc;
use std::cell::RefCell;

use failure::Error;

use smpl::{parse_module, UnparsedModule};
use smpl::interpreter::*;

pub const STATE_RUN: i32 = 0;
pub const STATE_IMAGE: i32 = 1;
pub const STATE_END: i32 = 2;

const RT_MOD: &'static str = "rt";

const RT_CHOICE: &'static str = "choice";
const RT_CLEAR_CHOICES: &'static str = "clear_choices";

const RT_SET_FLAG: &'static str = "set_flag";
const RT_SET_INT: &'static str = "set_int";
const RT_SET_FLOAT: &'static str = "set_float";

const RT_GET_FLAG: &'static str = "get_flag";
const RT_GET_INT: &'static str = "get_int";
const RT_GET_FLOAT: &'static str = "get_float";

pub const CTXT_STATE: &'static str = "state";
pub const CTXT_BACKGROUND: &'static str = "background";
pub const CTXT_DISPLAY: &'static str = "display";
pub const CTXT_CHOICE: &'static str = "choice_list";
pub const CTXT_FONT: &'static str = "font";
pub const CTXT_FLAG: &'static str = "flag_data";
pub const CTXT_INT: &'static str = "int_data";
pub const CTXT_FLOAT: &'static str = "float_data";

pub const CHOICE_HANDLE: &'static str = "handle";
pub const CHOICE_DISPLAY: &'static str = "display";


const RT_LIB: &'static str = include_str!("rt.smpl");

pub fn new_context() -> Struct {
    let mut s = Struct::new();

    s.set_field(CTXT_STATE.to_owned(),
                Value::Int(STATE_RUN));

    s.set_field(CTXT_BACKGROUND.to_owned(), 
                Value::String("".to_string()));

    s.set_field(CTXT_DISPLAY.to_owned(), 
                Value::String("".to_string()));

    s.set_field(CTXT_FONT.to_owned(), 
                Value::String("".to_string()));

    s.set_field(CTXT_CHOICE.to_owned(), 
                Value::Array(Vec::new()));

    s.set_field(CTXT_FLAG.to_owned(), 
                Value::Struct(Struct::new()));

    s.set_field(CTXT_INT.to_owned(), 
                Value::Struct(Struct::new()));

    s.set_field(CTXT_FLOAT.to_owned(), 
                Value::Struct(Struct::new()));
    s
}

pub fn vm_module() -> VmModule {
    let parsed = parse_module(UnparsedModule::anonymous(RT_LIB)).unwrap();
    let vm_module = VmModule::new(parsed)
        .add_builtin(RT_CHOICE, choice)
        .add_builtin(RT_CHOICE, clear_choices)

        .add_builtin(RT_CHOICE, set_flag)
        .add_builtin(RT_CHOICE, set_int)
        .add_builtin(RT_CHOICE, set_float)

        .add_builtin(RT_CHOICE, get_flag)
        .add_builtin(RT_CHOICE, get_int)
        .add_builtin(RT_CHOICE, get_float)
    ;

    vm_module
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

fn set_flag(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let to_set = args.pop().unwrap();
    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_FLAG).unwrap();
    let mut data = data.borrow_mut();
    let data = irmatch!(*data; Value::Struct(ref mut st) => st);

    data.set_field(name, to_set);

    Ok(Value::Struct(context))
}

fn set_int(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let to_set = args.pop().unwrap();
    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_INT).unwrap();
    let mut data = data.borrow_mut();
    let data = irmatch!(*data; Value::Struct(ref mut st) => st);

    data.set_field(name, to_set);

    Ok(Value::Struct(context))
}

fn set_float(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let to_set = args.pop().unwrap();
    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_FLOAT).unwrap();
    let mut data = data.borrow_mut();
    let data = irmatch!(*data; Value::Struct(ref mut st) => st);

    data.set_field(name, to_set);

    Ok(Value::Struct(context))
}

fn get_flag(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_FLAG).unwrap();
    let data = data.borrow();
    let data = irmatch!(*data; Value::Struct(ref st) => st);

    let value = data.get_field(&name)
        .expect(&format!("Unknown flag {}", name));

    Ok(value)
}

fn get_int(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_INT).unwrap();
    let data = data.borrow();
    let data = irmatch!(*data; Value::Struct(ref st) => st);

    let value = data.get_field(&name)
        .expect(&format!("Unknown int {}", name));

    Ok(value)
}

fn get_float(args: Option<Vec<Value>>) -> Result<Value, Error> {
    let mut args = args.unwrap();

    let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
    let context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

    let data = context.ref_field(CTXT_FLOAT).unwrap();
    let data = data.borrow();
    let data = irmatch!(*data; Value::Struct(ref st) => st);

    let value = data.get_field(&name)
        .expect(&format!("Unknown float {}", name));

    Ok(value)
}
