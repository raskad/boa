use crate::{
    builtins::Array,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `RestParameterInit` implements the Opcode Operation for `Opcode::RestParameterInit`
///
/// Operation:
///  - Initialize the rest parameter value of a function from the remaining arguments.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RestParameterInit;

impl RestParameterInit {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(dst: u32, context: &mut Context) -> JsResult<CompletionType> {
        let frame = context.vm.frame();
        let argument_count = frame.argument_count;
        let param_count = frame.code_block().parameter_length;
        let register_count = frame.code_block().register_count;
        let array = if argument_count >= param_count {
            let rest_count = argument_count - param_count + 1;

            let len = context.vm.stack.len() as u32;
            let start = (len - rest_count - register_count) as usize;
            let end = (len - register_count) as usize;

            let args = &context.vm.stack[start..end];

            let array = Array::create_array_from_list(args.iter().cloned(), context);
            context.vm.stack.drain(start..end);

            context.vm.frame_mut().rp -= (start..end).len() as u32;
            context.vm.frame_mut().argument_count -= (start..end).len() as u32;

            array
        } else {
            Array::array_create(0, None, context).expect("could not create an empty array")
        };

        let rp = context.vm.frame().rp;
        context.vm.stack[(rp + dst) as usize] = array.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for RestParameterInit {
    const NAME: &'static str = "RestParameterInit";
    const INSTRUCTION: &'static str = "INST - RestParameterInit";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u8>().into();
        Self::operation(dst, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u16>().into();
        Self::operation(dst, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u32>();
        Self::operation(dst, context)
    }
}
