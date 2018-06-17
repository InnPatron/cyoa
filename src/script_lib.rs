use std::rc::Rc;
use std::cell::RefCell;

use smpl::Module;
use smpl::parse_module;
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

pub fn include(modules: &mut Vec<Module>) {
    modules.push(parse_module(RT_LIB).unwrap());
}

pub fn map_builtins(vm: &mut VM) {
    vm.insert_builtin(RT_MOD, RT_CHOICE, Box::new(Choice));
    vm.insert_builtin(RT_MOD, RT_CLEAR_CHOICES, Box::new(ClearChoices));

    vm.insert_builtin(RT_MOD, RT_SET_FLAG, Box::new(SetFlag));
    vm.insert_builtin(RT_MOD, RT_SET_INT, Box::new(SetInt));
    vm.insert_builtin(RT_MOD, RT_SET_FLOAT, Box::new(SetFloat));

    vm.insert_builtin(RT_MOD, RT_GET_FLAG, Box::new(GetFlag));
    vm.insert_builtin(RT_MOD, RT_GET_INT, Box::new(GetInt));
    vm.insert_builtin(RT_MOD, RT_GET_FLOAT, Box::new(GetFloat));
}

pub struct Choice;

impl BuiltInFn for Choice {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();
        let display = args.pop().unwrap();
        let handler = args.pop().unwrap();
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let list = context.ref_field(CTXT_CHOICE).unwrap();
        let mut list = list.borrow_mut();
        let mut list = irmatch!(*list; Value::Array(ref mut vec) => vec);

        let mut choice = Struct::new();
        choice.set_field(CHOICE_HANDLE.to_owned(), handler);
        choice.set_field(CHOICE_DISPLAY.to_owned(), display);

        list.push(Rc::new(RefCell::new(Value::Struct(choice))));

        Value::Struct(context)
    }
}

pub struct ClearChoices;

impl BuiltInFn for ClearChoices {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let list = context.ref_field(CTXT_CHOICE).unwrap();
        let mut list = list.borrow_mut();
        let mut list = irmatch!(*list; Value::Array(ref mut vec) => vec);

        list.clear();

        Value::Struct(context)
    }
}

pub struct SetFlag;

impl BuiltInFn for SetFlag {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let to_set = args.pop().unwrap();
        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_FLAG).unwrap();
        let mut data = data.borrow_mut();
        let mut data = irmatch!(*data; Value::Struct(ref mut st) => st);

        data.set_field(name, to_set);

        Value::Struct(context)
    }
}

pub struct SetInt;

impl BuiltInFn for SetInt {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let to_set = args.pop().unwrap();
        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_INT).unwrap();
        let mut data = data.borrow_mut();
        let mut data = irmatch!(*data; Value::Struct(ref mut st) => st);

        data.set_field(name, to_set);

        Value::Struct(context)
    }
}

pub struct SetFloat;

impl BuiltInFn for SetFloat {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let to_set = args.pop().unwrap();
        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_FLOAT).unwrap();
        let mut data = data.borrow_mut();
        let mut data = irmatch!(*data; Value::Struct(ref mut st) => st);

        data.set_field(name, to_set);

        Value::Struct(context)
    }
}

pub struct GetFlag;

impl BuiltInFn for GetFlag {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_FLAG).unwrap();
        let data = data.borrow();
        let data = irmatch!(*data; Value::Struct(ref st) => st);

        let value = data.get_field(&name)
            .expect(&format!("Unknown flag {}", name));

        value
    }
}

pub struct GetInt;

impl BuiltInFn for GetInt {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_INT).unwrap();
        let data = data.borrow();
        let data = irmatch!(*data; Value::Struct(ref st) => st);

        let value = data.get_field(&name)
            .expect(&format!("Unknown int {}", name));

        value
    }
}

pub struct GetFloat;

impl BuiltInFn for GetFloat {
    fn execute(&self, args: Option<Vec<Value>>) -> Value {
        let mut args = args.unwrap();

        let name = irmatch!(args.pop().unwrap(); Value::String(s) => s);
        let mut context = irmatch!(args.pop().unwrap(); Value::Struct(c) => c);

        let data = context.ref_field(CTXT_FLOAT).unwrap();
        let data = data.borrow();
        let data = irmatch!(*data; Value::Struct(ref st) => st);

        let value = data.get_field(&name)
            .expect(&format!("Unknown float {}", name));

        value
    }
}
