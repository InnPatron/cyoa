use popstcl_core::internal::*;
use popstcl_core::RcValue;

#[derive(Clone, Debug)]
pub struct Display(pub RcValue);

impl Cmd for Display {
    fn execute(&self, stack: &mut Stack, args: Vec<CIR>) -> Result<ExecSignal, ExecErr> {
        exact_args!(args, 1);
        {
            cir_extract!(args[0] => String)?;
        }
        *self.0.borrow_mut() = args[0].value.inner_clone();
        
        Ok(ExecSignal::NextInstruction(None))
    }
}
