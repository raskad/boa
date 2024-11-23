use crate::{
    object::internal_methods::InternalMethodContext,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `SetPrototype` implements the Opcode Operation for `Opcode::SetPrototype`
///
/// Operation:
///  - Sets the prototype of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPrototype;

impl SetPrototype {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(object: u32, value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object) as usize].clone();
        let value = context.vm.stack[(rp + value) as usize].clone();

        let prototype = if let Some(prototype) = value.as_object() {
            Some(prototype.clone())
        } else if value.is_null() {
            None
        } else {
            return Ok(CompletionType::Normal);
        };

        let object = object.as_object().expect("object is not an object");
        object
            .__set_prototype_of__(prototype, &mut InternalMethodContext::new(context))
            .expect("cannot fail per spec");

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPrototype {
    const NAME: &'static str = "SetPrototype";
    const INSTRUCTION: &'static str = "INST - SetPrototype";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let value = context.vm.read::<u8>().into();
        Self::operation(object, value, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let value = context.vm.read::<u16>().into();
        Self::operation(object, value, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>();
        let value = context.vm.read::<u32>();
        Self::operation(object, value, context)
    }
}
