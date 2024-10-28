use crate::{
    builtins::iterable::IteratorHint,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `GetIterator` implements the Opcode Operation for `Opcode::GetIterator`
///
/// Operation:
///  - Initialize an iterator
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetIterator;

impl GetIterator {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let iterator = value.get_iterator(IteratorHint::Sync, context)?;
        context.vm.frame_mut().iterators.push(iterator);
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetIterator {
    const NAME: &'static str = "GetIterator";
    const INSTRUCTION: &'static str = "INST - GetIterator";
    const COST: u8 = 6;

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

/// `GetAsyncIterator` implements the Opcode Operation for `Opcode::GetAsyncIterator`
///
/// Operation:
///  - Initialize an async iterator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct GetAsyncIterator;

impl GetAsyncIterator {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let iterator = value.get_iterator(IteratorHint::Async, context)?;
        context.vm.frame_mut().iterators.push(iterator);
        Ok(CompletionType::Normal)
    }
}

impl Operation for GetAsyncIterator {
    const NAME: &'static str = "GetAsyncIterator";
    const INSTRUCTION: &'static str = "INST - GetAsyncIterator";
    const COST: u8 = 6;

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
