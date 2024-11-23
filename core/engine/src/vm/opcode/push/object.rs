use crate::{
    builtins::OrdinaryObject,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `PushEmptyObject` implements the Opcode Operation for `Opcode::PushEmptyObject`
///
/// Operation:
///  - Push empty object `{}` value on the stack.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PushEmptyObject;

impl PushEmptyObject {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(dst: u32, context: &mut Context) -> JsResult<CompletionType> {
        let o = context
        .intrinsics()
        .templates()
        .ordinary_object()
        .create(OrdinaryObject, Vec::default());
    let rp = context.vm.frame().rp;
    context.vm.stack[(rp + dst) as usize] = o.into();
Ok(CompletionType::Normal)
    }
}

impl Operation for PushEmptyObject {
    const NAME: &'static str = "PushEmptyObject";
    const INSTRUCTION: &'static str = "INST - PushEmptyObject";
    const COST: u8 = 1;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u8>().into();
        Self::operation(dst,context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u16>().into();
        Self::operation(dst,context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u32>();
        Self::operation(dst,context)
    }
}
