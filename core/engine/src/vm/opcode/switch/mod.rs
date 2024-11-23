use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `Case` implements the Opcode Operation for `Opcode::Case`
///
/// Operation:
///  - Pop the two values of the stack, strict equal compares the two values,
///    if true jumps to address, otherwise push the second pop'ed value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Case;

impl Case {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(
        address: u32,
        value: u32,
        condition: u32,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let condition = context.vm.stack[(rp + condition) as usize].clone();
        if value.strict_equals(&condition) {
            context.vm.frame_mut().pc = address;
        }
        Ok(CompletionType::Normal)
    }
}

impl Operation for Case {
    const NAME: &'static str = "Case";
    const INSTRUCTION: &'static str = "INST - Case";
    const COST: u8 = 2;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        let value = context.vm.read::<u8>().into();
        let condition = context.vm.read::<u8>().into();
        Self::operation(address, value, condition, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        let value = context.vm.read::<u16>().into();
        let condition = context.vm.read::<u16>().into();
        Self::operation(address, value, condition, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let address = context.vm.read::<u32>();
        let value = context.vm.read::<u32>();
        let condition = context.vm.read::<u32>();
        Self::operation(address, value, condition, context)
    }
}
