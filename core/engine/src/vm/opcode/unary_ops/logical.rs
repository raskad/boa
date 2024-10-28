use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `LogicalNot` implements the Opcode Operation for `Opcode::LogicalNot`
///
/// Operation:
///  - Unary logical `!` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LogicalNot;

impl LogicalNot {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        context.vm.stack[addr] = (!value.to_boolean()).into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for LogicalNot {
    const NAME: &'static str = "LogicalNot";
    const INSTRUCTION: &'static str = "INST - LogicalNot";
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
