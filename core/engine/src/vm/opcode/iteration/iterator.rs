use crate::{
    builtins::{iterable::create_iter_result_object, Array},
    js_string,
    vm::{opcode::Operation, CompletionType, GeneratorResumeKind},
    Context, JsResult,
};

/// `IteratorNext` implements the Opcode Operation for `Opcode::IteratorNext`
///
/// Operation:
///  - Calls the `next` method of `iterator`, updating its record with the next value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorNext;

impl Operation for IteratorNext {
    const NAME: &'static str = "IteratorNext";
    const INSTRUCTION: &'static str = "INST - IteratorNext";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let mut iterator = context
            .vm
            .frame_mut()
            .iterators
            .pop()
            .expect("iterator stack should have at least an iterator");

        iterator.step(context)?;

        context.vm.frame_mut().iterators.push(iterator);

        Ok(CompletionType::Normal)
    }
}

/// `IteratorFinishAsyncNext` implements the Opcode Operation for `Opcode::IteratorFinishAsyncNext`.
///
/// Operation:
///  - Finishes the call to `Opcode::IteratorNext` within a `for await` loop by setting the current
///    result of the current iterator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorFinishAsyncNext;

impl IteratorFinishAsyncNext {
    fn operation(resume_kind: u32, value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut iterator = context
            .vm
            .frame_mut()
            .iterators
            .pop()
            .expect("iterator on the call frame must exist");

        let resume_kind = context.vm.stack[(rp + resume_kind) as usize].to_generator_resume_kind();

        if matches!(resume_kind, GeneratorResumeKind::Throw) {
            // If after awaiting the `next` call the iterator returned an error, it can be considered
            // as poisoned, meaning we can remove it from the iterator stack to avoid calling
            // cleanup operations on it.
            return Ok(CompletionType::Normal);
        }

        let value = context.vm.stack[(rp + value) as usize].clone();

        iterator.update_result(value, context)?;
        context.vm.frame_mut().iterators.push(iterator);
        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorFinishAsyncNext {
    const NAME: &'static str = "IteratorFinishAsyncNext";
    const INSTRUCTION: &'static str = "INST - IteratorFinishAsyncNext";
    const COST: u8 = 5;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let resume_kind = context.vm.read::<u8>().into();
        let value = context.vm.read::<u8>().into();
        Self::operation(resume_kind, value, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let resume_kind = context.vm.read::<u16>().into();
        let value = context.vm.read::<u16>().into();
        Self::operation(resume_kind, value, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let resume_kind = context.vm.read::<u32>();
        let value = context.vm.read::<u32>();
        Self::operation(resume_kind, value, context)
    }
}

/// `IteratorResult` implements the Opcode Operation for `Opcode::IteratorResult`
///
/// Operation:
///  - Gets the last iteration result of the current iterator record.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorResult;

impl IteratorResult {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let last_result = context
            .vm
            .frame()
            .iterators
            .last()
            .expect("iterator on the call frame must exist")
            .last_result()
            .object()
            .clone();
        context.vm.stack[(rp + value) as usize] = last_result.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorResult {
    const NAME: &'static str = "IteratorResult";
    const INSTRUCTION: &'static str = "INST - IteratorResult";
    const COST: u8 = 3;

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

/// `IteratorValue` implements the Opcode Operation for `Opcode::IteratorValue`
///
/// Operation:
///  - Gets the `value` property of the current iterator record.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorValue;

impl IteratorValue {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut iterator = context
            .vm
            .frame_mut()
            .iterators
            .pop()
            .expect("iterator on the call frame must exist");

        let iter_value = iterator.value(context)?;
        context.vm.stack[(rp + value) as usize] = iter_value;

        context.vm.frame_mut().iterators.push(iterator);

        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorValue {
    const NAME: &'static str = "IteratorValue";
    const INSTRUCTION: &'static str = "INST - IteratorValue";
    const COST: u8 = 5;

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

/// `IteratorDone` implements the Opcode Operation for `Opcode::IteratorDone`
///
/// Operation:
///  - Returns `true` if the current iterator is done, or `false` otherwise
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorDone;

impl IteratorDone {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(done: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context
            .vm
            .frame()
            .iterators
            .last()
            .expect("iterator on the call frame must exist")
            .done();

        context.vm.stack[(rp + done) as usize] = value.into();

        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorDone {
    const NAME: &'static str = "IteratorDone";
    const INSTRUCTION: &'static str = "INST - IteratorDone";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let done = context.vm.read::<u8>().into();
        Self::operation(done, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let done = context.vm.read::<u16>().into();
        Self::operation(done, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let done = context.vm.read::<u32>();
        Self::operation(done, context)
    }
}

/// `IteratorReturn` implements the Opcode Operation for `Opcode::IteratorReturn`
///
/// Operation:
///  - Calls `return` on the current iterator and returns the result.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorReturn;

impl IteratorReturn {
    fn operation(value: u32, called: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;

        let Some(record) = context.vm.frame_mut().iterators.pop() else {
            context.vm.stack[(rp + called) as usize] = false.into();
            return Ok(CompletionType::Normal);
        };

        if record.done() {
            context.vm.stack[(rp + called) as usize] = false.into();
            return Ok(CompletionType::Normal);
        }

        let Some(ret) = record
            .iterator()
            .get_method(js_string!("return"), context)?
        else {
            context.vm.stack[(rp + called) as usize] = false.into();
            return Ok(CompletionType::Normal);
        };

        let old_return_value = context.vm.get_return_value();

        let return_value = ret.call(&record.iterator().clone().into(), &[], context)?;

        context.vm.set_return_value(old_return_value);

        context.vm.stack[(rp + value) as usize] = return_value;
        context.vm.stack[(rp + called) as usize] = true.into();

        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorReturn {
    const NAME: &'static str = "IteratorReturn";
    const INSTRUCTION: &'static str = "INST - IteratorReturn";
    const COST: u8 = 8;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        let called = context.vm.read::<u8>().into();
        Self::operation(value, called, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        let called = context.vm.read::<u16>().into();
        Self::operation(value, called, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        let called = context.vm.read::<u32>();
        Self::operation(value, called, context)
    }
}

/// `IteratorToArray` implements the Opcode Operation for `Opcode::IteratorToArray`
///
/// Operation:
///  - Consume the iterator and construct and array with all the values.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorToArray;

impl IteratorToArray {
    fn operation(array: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;

        let mut iterator = context
            .vm
            .frame_mut()
            .iterators
            .pop()
            .expect("iterator on the call frame must exist");

        let mut values = Vec::new();

        loop {
            let done = match iterator.step(context) {
                Ok(done) => done,
                Err(err) => {
                    context.vm.frame_mut().iterators.push(iterator);
                    return Err(err);
                }
            };

            if done {
                break;
            }

            match iterator.value(context) {
                Ok(value) => values.push(value),
                Err(err) => {
                    context.vm.frame_mut().iterators.push(iterator);
                    return Err(err);
                }
            }
        }

        context.vm.frame_mut().iterators.push(iterator);
        let result = Array::create_array_from_list(values, context);
        context.vm.stack[(rp + array) as usize] = result.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorToArray {
    const NAME: &'static str = "IteratorToArray";
    const INSTRUCTION: &'static str = "INST - IteratorToArray";
    const COST: u8 = 8;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let array = context.vm.read::<u8>().into();
        Self::operation(array, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let array = context.vm.read::<u16>().into();
        Self::operation(array, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let array = context.vm.read::<u32>();
        Self::operation(array, context)
    }
}

/// `IteratorStackEmpty` implements the Opcode Operation for `Opcode::IteratorStackEmpty`
///
/// Operation:
/// - Pushes `true` to the stack if the iterator stack is empty.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorStackEmpty;

impl IteratorStackEmpty {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(empty: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let is_empty = context.vm.frame().iterators.is_empty();
        context.vm.stack[(rp + empty) as usize] = is_empty.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for IteratorStackEmpty {
    const NAME: &'static str = "IteratorStackEmpty";
    const INSTRUCTION: &'static str = "INST - IteratorStackEmpty";
    const COST: u8 = 1;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let empty = context.vm.read::<u8>().into();
        Self::operation(empty, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let empty = context.vm.read::<u16>().into();
        Self::operation(empty, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let empty = context.vm.read::<u32>();
        Self::operation(empty, context)
    }
}

/// `CreateIteratorResult` implements the Opcode Operation for `Opcode::CreateIteratorResult`
///
/// Operation:
/// -  Creates a new iterator result object
#[derive(Debug, Clone, Copy)]
pub(crate) struct CreateIteratorResult;

impl CreateIteratorResult {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(value: u32, done: bool, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let val = context.vm.stack[(rp + value) as usize].clone();
        let result = create_iter_result_object(val, done, context);
        context.vm.stack[(rp + value) as usize] = result;
        Ok(CompletionType::Normal)
    }
}

impl Operation for CreateIteratorResult {
    const NAME: &'static str = "CreateIteratorResult";
    const INSTRUCTION: &'static str = "INST - CreateIteratorResult";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        let done = context.vm.read::<u8>() != 0;
        Self::operation(value, done, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        let done = context.vm.read::<u8>() != 0;
        Self::operation(value, done, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        let done = context.vm.read::<u8>() != 0;
        Self::operation(value, done, context)
    }
}
