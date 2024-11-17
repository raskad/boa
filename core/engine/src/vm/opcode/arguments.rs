use crate::{
    builtins::function::arguments::{MappedArguments, UnmappedArguments},
    vm::CompletionType,
    Context, JsResult,
};

use super::Operation;

/// `CreateMappedArgumentsObject` implements the Opcode Operation for `Opcode::CreateMappedArgumentsObject`
///
/// Operation:
///  - TODO: doc
#[derive(Debug, Clone, Copy)]
pub(crate) struct CreateMappedArgumentsObject;

impl CreateMappedArgumentsObject {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let frame = context.vm.frame();
        let function_object = frame
            .function(&context.vm)
            .clone()
            .expect("there should be a function object");
        let code = frame.code_block().clone();
        let args = frame.arguments(&context.vm).to_vec();
        let env = context
            .vm
            .environments
            .current_declarative_ref()
            .expect("must be declarative");
        let arguments = MappedArguments::new(
            &function_object,
            &code.mapped_arguments_binding_indices,
            &args,
            env,
            context,
        );
        context.vm.stack[(rp + value) as usize] = arguments.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for CreateMappedArgumentsObject {
    const NAME: &'static str = "CreateMappedArgumentsObject";
    const INSTRUCTION: &'static str = "INST - CreateMappedArgumentsObject";
    const COST: u8 = 8;

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

/// `CreateUnmappedArgumentsObject` implements the Opcode Operation for `Opcode::CreateUnmappedArgumentsObject`
///
/// Operation:
///  - TODO: doc
#[derive(Debug, Clone, Copy)]
pub(crate) struct CreateUnmappedArgumentsObject;

impl CreateUnmappedArgumentsObject {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let args = context.vm.frame().arguments(&context.vm).to_vec();
        let arguments = UnmappedArguments::new(&args, context);
        context.vm.stack[(rp + value) as usize] = arguments.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for CreateUnmappedArgumentsObject {
    const NAME: &'static str = "CreateUnmappedArgumentsObject";
    const INSTRUCTION: &'static str = "INST - CreateUnmappedArgumentsObject";
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
        let value = context.vm.read::<u32>().into();
        Self::operation(value, context)
    }
}
