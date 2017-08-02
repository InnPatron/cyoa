use popstcl_core::internal::*;
use popstcl_core::RcValue;

#[derive(Clone, Debug)]
pub struct Display(pub RcValue);

impl Cmd for Display {
    
    #[allow(unused)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        exact_args!(args, 1);
        {
            cir_extract!(args[0] => String)?;
        }
        *self.0.borrow_mut() = args[0].value.inner_clone();
        
        Ok(ExecSignal::NextInstruction(None))
    }
}

#[derive(Clone, Debug)]
pub struct GameState(pub RcValue);

impl Cmd for GameState {
    #[allow(unused)]
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, CmdErr> {
        max_args!(args, 1);
        if args.len() == 0 {
            return Ok(ExecSignal::NextInstruction(Some(self.0.clone())));
        } else {
            {
                cir_extract!(args[0] => Number)?;
            }
            *self.0.borrow_mut() = args[0].value.inner_clone();
        }
        
        Ok(ExecSignal::NextInstruction(None))
    }
}
