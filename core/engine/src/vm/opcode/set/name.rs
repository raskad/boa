use boa_ast::scope::{BindingLocator, BindingLocatorScope};

use crate::{
    environments::Environment,
    vm::{opcode::Operation, CompletionType},
    Context, JsNativeError, JsResult,
};

/// `ThrowMutateImmutable` implements the Opcode Operation for `Opcode::ThrowMutateImmutable`
///
/// Operation:
///  - Throws an error because the binding access is illegal.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ThrowMutateImmutable;

impl ThrowMutateImmutable {
    fn operation(context: &mut Context, index: usize) -> JsResult<CompletionType> {
        let name = context.vm.frame().code_block().constant_string(index);

        Err(JsNativeError::typ()
            .with_message(format!(
                "cannot mutate an immutable binding '{}'",
                name.to_std_string_escaped()
            ))
            .into())
    }
}

impl Operation for ThrowMutateImmutable {
    const NAME: &'static str = "ThrowMutateImmutable";
    const INSTRUCTION: &'static str = "INST - ThrowMutateImmutable";
    const COST: u8 = 2;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u8>();
        Self::operation(context, index as usize)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u16>() as usize;
        Self::operation(context, index)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let index = context.vm.read::<u32>();
        Self::operation(context, index as usize)
    }
}

/// `SetName` implements the Opcode Operation for `Opcode::SetName`
///
/// Operation:
///  - Find a binding on the environment chain and assign its value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetName;

impl SetName {
    fn operation(value: u32, index: usize, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let code_block = context.vm.frame().code_block();
        let mut binding_locator = code_block.bindings[index].clone();
        let strict = code_block.strict();

        context.find_runtime_binding(&mut binding_locator)?;

        verify_initialized(&binding_locator, context)?;

        context.set_binding(&binding_locator, value, strict)?;

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetName {
    const NAME: &'static str = "SetName";
    const INSTRUCTION: &'static str = "INST - SetName";
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

/// `SetNameByLocator` implements the Opcode Operation for `Opcode::SetNameByLocator`
///
/// Operation:
///  - Assigns a value to the binding pointed by the `current_binding` of the current frame.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetNameByLocator;

impl SetNameByLocator {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let frame = context.vm.frame_mut();
        let strict = frame.code_block.strict();
        let binding_locator = frame
            .binding_stack
            .pop()
            .expect("locator should have been popped before");
        let value = context.vm.stack[(rp + value) as usize].clone();

        verify_initialized(&binding_locator, context)?;

        context.set_binding(&binding_locator, value, strict)?;

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetNameByLocator {
    const NAME: &'static str = "SetNameByLocator";
    const INSTRUCTION: &'static str = "INST - SetNameByLocator";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        Self::operation(value, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        Self::operation(value, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        Self::operation(value, context)
    }
}

/// Checks that the binding pointed by `locator` exists and is initialized.
fn verify_initialized(locator: &BindingLocator, context: &mut Context) -> JsResult<()> {
    if !context.is_initialized_binding(locator)? {
        let key = locator.name();
        let strict = context.vm.frame().code_block.strict();

        let message = match locator.scope() {
            BindingLocatorScope::GlobalObject if strict => Some(format!(
                "cannot assign to uninitialized global property `{}`",
                key.to_std_string_escaped()
            )),
            BindingLocatorScope::GlobalObject => None,
            BindingLocatorScope::GlobalDeclarative => Some(format!(
                "cannot assign to uninitialized binding `{}`",
                key.to_std_string_escaped()
            )),
            BindingLocatorScope::Stack(index) => match context.environment_expect(index) {
                Environment::Declarative(_) => Some(format!(
                    "cannot assign to uninitialized binding `{}`",
                    key.to_std_string_escaped()
                )),
                Environment::Object(_) if strict => Some(format!(
                    "cannot assign to uninitialized property `{}`",
                    key.to_std_string_escaped()
                )),
                Environment::Object(_) => None,
            },
        };

        if let Some(message) = message {
            return Err(JsNativeError::reference().with_message(message).into());
        }
    }

    Ok(())
}
