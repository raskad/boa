use crate::{
    error::JsNativeError,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `ValueNotNullOrUndefined` implements the Opcode Operation for `Opcode::ValueNotNullOrUndefined`
///
/// Operation:
///  - Require the stack value to be neither null nor undefined.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ValueNotNullOrUndefined;

impl ValueNotNullOrUndefined {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        if value.is_null() {
            return Err(JsNativeError::typ()
                .with_message("Cannot destructure 'null' value")
                .into());
        }
        if value.is_undefined() {
            return Err(JsNativeError::typ()
                .with_message("Cannot destructure 'undefined' value")
                .into());
        }
        Ok(CompletionType::Normal)
    }
}

impl Operation for ValueNotNullOrUndefined {
    const NAME: &'static str = "ValueNotNullOrUndefined";
    const INSTRUCTION: &'static str = "INST - ValueNotNullOrUndefined";
    const COST: u8 = 2;

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

/// `IsObject` implements the Opcode Operation for `Opcode::IsObject`
///
/// Operation:
///  - Pushes `true` to the stack if the top stack value is an object, or `false` otherwise.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IsObject;

impl IsObject {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        context.vm.stack[addr] = value.is_object().into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for IsObject {
    const NAME: &'static str = "IsObject";
    const INSTRUCTION: &'static str = "INST - IsObject";
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
