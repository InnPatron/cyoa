use std::rc::Rc;
use std::cell::{RefCell, Cell};

use popstcl_core::internal::*;
use popstcl_core::RcValue;
use game::VmPipes;

#[derive(Clone, Debug)]
pub struct Display(pub Rc<VmPipes>);

impl Cmd for Display {
    
    #[allow(unused)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        {
            cir_extract!(args[0] => String)?;
        }
        *self.0.vm_out.borrow_mut() = args[0].value.to_string();
        
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct GameState(pub Rc<VmPipes>);

impl Cmd for GameState {
    #[allow(unused)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        max_args!(args, 1);
        if args.len() == 0 {
            let state = self.0.game_state.get() as f64;
            return Ok(ExecSignal::NextInstruction(Some(state.into())));
        } else {
            let v: i32 = *cir_extract!(args[0] => Number)? as i32;
            self.0.game_state.set(v);
        }
        
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct NewMod(pub Rc<VmPipes>);

impl Cmd for NewMod {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 0);
        use game;
        let env = game::cyoa_env(self.0.clone()).consume();
        
        Ok(ExecSignal::NextInstruction(Some(StdModule::new(env).into())))
    }
}
