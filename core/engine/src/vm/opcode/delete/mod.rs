use crate::{
    error::JsNativeError,
    object::internal_methods::InternalMethodContext,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `DeletePropertyByName` implements the Opcode Operation for `Opcode::DeletePropertyByName`
///
/// Operation:
///  - Deletes a property by name of an object
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeletePropertyByName;

impl DeletePropertyByName {
    fn operation(
        object_register: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object_register) as usize].clone();
        let object = object.to_object_owned(context)?;
        let code_block = context.vm.frame().code_block();
        let key = code_block.constant_string(index).into();
        let strict = code_block.strict();

        let result = object.__delete__(&key, &mut InternalMethodContext::new(context))?;
        if !result && strict {
            return Err(JsNativeError::typ()
                .with_message("Cannot delete property")
                .into());
        }
        context.vm.stack[(rp + object_register) as usize] = result.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for DeletePropertyByName {
    const NAME: &'static str = "DeletePropertyByName";
    const INSTRUCTION: &'static str = "INST - DeletePropertyByName";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(object, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(object, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(object, index, context)
    }
}

/// `DeletePropertyByValue` implements the Opcode Operation for `Opcode::DeletePropertyByValue`
///
/// Operation:
///  - Deletes a property by value of an object
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeletePropertyByValue;

impl DeletePropertyByValue {
    fn operation(
        object_register: u32,
        key: u32,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object_register) as usize].clone();
        let key = context.vm.stack[(rp + key) as usize].clone();
        let object = object.to_object_owned(context)?;
        let property_key = key.to_property_key(context)?;

        let result = object.__delete__(&property_key, &mut InternalMethodContext::new(context))?;
        if !result && context.vm.frame().code_block().strict() {
            return Err(JsNativeError::typ()
                .with_message("Cannot delete property")
                .into());
        }
        context.vm.stack[(rp + object_register) as usize] = result.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for DeletePropertyByValue {
    const NAME: &'static str = "DeletePropertyByValue";
    const INSTRUCTION: &'static str = "INST - DeletePropertyByValue";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        Self::operation(object, key, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        Self::operation(object, key, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        Self::operation(object, key, context)
    }
}

/// `DeleteName` implements the Opcode Operation for `Opcode::DeleteName`
///
/// Operation:
///  - Deletes a property by value of an object
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeleteName;

impl DeleteName {
    fn operation(value: u32, index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut binding_locator = context.vm.frame().code_block.bindings[index].clone();

        context.find_runtime_binding(&mut binding_locator)?;

        let deleted = context.delete_binding(&binding_locator)?;

        context.vm.stack[(rp + value) as usize] = deleted.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for DeleteName {
    const NAME: &'static str = "DeleteName";
    const INSTRUCTION: &'static str = "INST - DeleteName";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(value, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(value, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(value, index, context)
    }
}

/// `DeleteSuperThrow` implements the Opcode Operation for `Opcode::DeleteSuperThrow`
///
/// Operation:
///  - Throws an error when trying to delete a property of `super`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeleteSuperThrow;

impl Operation for DeleteSuperThrow {
    const NAME: &'static str = "DeleteSuperThrow";
    const INSTRUCTION: &'static str = "INST - DeleteSuperThrow";
    const COST: u8 = 2;

    fn execute(_: &mut Context) -> JsResult<CompletionType> {
        Err(JsNativeError::reference()
            .with_message("cannot delete a property of `super`")
            .into())
    }
}
