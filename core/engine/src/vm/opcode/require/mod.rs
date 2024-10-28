use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `RequireObjectCoercible` implements the Opcode Operation for `Opcode::RequireObjectCoercible`
///
/// Operation:
///  - Call `RequireObjectCoercible` on the stack value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RequireObjectCoercible;

impl RequireObjectCoercible {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        value.require_object_coercible()?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for RequireObjectCoercible {
    const NAME: &'static str = "RequireObjectCoercible";
    const INSTRUCTION: &'static str = "INST - RequireObjectCoercible";
    const COST: u8 = 1;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        Self::operation(value, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        Self::operation(value, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>().into();
        Self::operation(value, context)
    }
}
