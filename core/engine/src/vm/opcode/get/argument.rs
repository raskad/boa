use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `GetArgument` implements the Opcode Operation for `Opcode::GetArgument`
///
/// Operation:
///  - Get i-th argument of the current frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetArgument;

impl GetArgument {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(index: usize, dst: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context
            .vm
            .frame()
            .argument(index, &context.vm)
            .cloned()
            .unwrap_or_default();
        context.vm.stack[(rp + dst) as usize] = value.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetArgument {
    const NAME: &'static str = "GetArgument";
    const INSTRUCTION: &'static str = "INST - GetArgument";
    const COST: u8 = 2;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u8>() as usize;
        let dst = context.vm.read::<u8>().into();
        Self::operation(index, dst, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u16>() as usize;
        let dst = context.vm.read::<u16>().into();
        Self::operation(index, dst, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u32>() as usize;
        let dst = context.vm.read::<u32>();
        Self::operation(index, dst, context)
    }
}
