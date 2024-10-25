use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `GetPrivateField` implements the Opcode Operation for `Opcode::GetPrivateField`
///
/// Operation:
///  - Get a private property by name from an object an push it on the stack.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetPrivateField;

impl GetPrivateField {
    fn operation(
        operand_types: u8,
        dst: u32,
        object: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let name = context.vm.frame().code_block().constant_string(index);
        let rp = context.vm.frame().rp;
        let object = context
            .vm
            .frame()
            .read_value::<0>(operand_types, object, &context.vm);
        let base_obj = object.to_object(context)?;

        let name = context
            .vm
            .environments
            .resolve_private_identifier(name)
            .expect("private name must be in environment");

        let result = base_obj.private_get(&name, context)?;
        context.vm.stack[(rp + dst) as usize] = result;
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetPrivateField {
    const NAME: &'static str = "GetPrivateField";
    const INSTRUCTION: &'static str = "INST - GetPrivateField";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let dst = context.vm.read::<u8>().into();
        let object = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(operand_types, dst, object, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let dst = context.vm.read::<u16>().into();
        let object = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(operand_types, dst, object, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let dst = context.vm.read::<u32>();
        let object = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(operand_types, dst, object, index, context)
    }
}
