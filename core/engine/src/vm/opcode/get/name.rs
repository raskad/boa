use crate::{
    error::JsNativeError,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult, JsValue,
};

/// `GetName` implements the Opcode Operation for `Opcode::GetName`
///
/// Operation:
///  - Find a binding on the environment chain and push its value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetName;

impl GetName {
    fn operation(value: u32, index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut binding_locator = context.vm.frame().code_block.bindings[index].clone();
        context.find_runtime_binding(&mut binding_locator)?;
        let result = context.get_binding(&binding_locator)?.ok_or_else(|| {
            let name = binding_locator.name().to_std_string_escaped();
            JsNativeError::reference().with_message(format!("{name} is not defined"))
        })?;
        context.vm.stack[(rp + value) as usize] = result;
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetName {
    const NAME: &'static str = "GetName";
    const INSTRUCTION: &'static str = "INST - GetName";
    const COST: u8 = 4;

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

/// `GetLocator` implements the Opcode Operation for `Opcode::GetLocator`
///
/// Operation:
///  - Find a binding on the environment and set the `current_binding` of the current frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetLocator;

impl GetLocator {
    fn operation(index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let mut binding_locator = context.vm.frame().code_block.bindings[index].clone();
        context.find_runtime_binding(&mut binding_locator)?;

        context.vm.frame_mut().binding_stack.push(binding_locator);

        Ok(CompletionType::Normal)
    }
}

impl Operation for GetLocator {
    const NAME: &'static str = "GetLocator";
    const INSTRUCTION: &'static str = "INST - GetLocator";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u8>() as usize;
        Self::operation(index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u16>() as usize;
        Self::operation(index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u32>() as usize;
        Self::operation(index, context)
    }
}

/// `GetNameAndLocator` implements the Opcode Operation for `Opcode::GetNameAndLocator`
///
/// Operation:
///  - Find a binding on the environment chain and push its value to the stack, setting the
///    `current_binding` of the current frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetNameAndLocator;

impl GetNameAndLocator {
    fn operation(value: u32, index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut binding_locator = context.vm.frame().code_block.bindings[index].clone();
        context.find_runtime_binding(&mut binding_locator)?;
        let result = context.get_binding(&binding_locator)?.ok_or_else(|| {
            let name = binding_locator.name().to_std_string_escaped();
            JsNativeError::reference().with_message(format!("{name} is not defined"))
        })?;

        context.vm.frame_mut().binding_stack.push(binding_locator);
        context.vm.stack[(rp + value) as usize] = result;
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetNameAndLocator {
    const NAME: &'static str = "GetNameAndLocator";
    const INSTRUCTION: &'static str = "INST - GetNameAndLocator";
    const COST: u8 = 4;

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

/// `GetNameOrUndefined` implements the Opcode Operation for `Opcode::GetNameOrUndefined`
///
/// Operation:
///  - Find a binding on the environment chain and push its value. If the binding does not exist push undefined.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetNameOrUndefined;

impl GetNameOrUndefined {
    fn operation(value: u32, index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut binding_locator = context.vm.frame().code_block.bindings[index].clone();

        let is_global = binding_locator.is_global();

        context.find_runtime_binding(&mut binding_locator)?;

        let result = if let Some(value) = context.get_binding(&binding_locator)? {
            value
        } else if is_global {
            JsValue::undefined()
        } else {
            let name = binding_locator.name().to_std_string_escaped();
            return Err(JsNativeError::reference()
                .with_message(format!("{name} is not defined"))
                .into());
        };

        context.vm.stack[(rp + value) as usize] = result;
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetNameOrUndefined {
    const NAME: &'static str = "GetNameOrUndefined";
    const INSTRUCTION: &'static str = "INST - GetNameOrUndefined";
    const COST: u8 = 4;

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
