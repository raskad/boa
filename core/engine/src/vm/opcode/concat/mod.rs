use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult, JsString,
};

/// `ConcatToString` implements the Opcode Operation for `Opcode::ConcatToString`
///
/// Operation:
///  - Concat multiple stack objects into a string.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ConcatToString;

impl ConcatToString {
    fn operation(string: u32, values: &[u32], context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let mut strings = Vec::with_capacity(values.len());
        for value in values {
            let val = context.vm.stack[(rp + value) as usize].clone();
            strings.push(val.to_string(context)?);
        }
        let s = JsString::concat_array(
            &strings
                .iter()
                .map(JsString::as_str)
                .map(Into::into)
                .collect::<Vec<_>>(),
        );
        context.vm.stack[(rp + string) as usize] = s.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for ConcatToString {
    const NAME: &'static str = "ConcatToString";
    const INSTRUCTION: &'static str = "INST - ConcatToString";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let string = context.vm.read::<u8>().into();
        let value_count = context.vm.read::<u8>() as usize;
        let mut values = Vec::with_capacity(value_count);
        for _ in 0..value_count {
            values.push(context.vm.read::<u8>().into());
        }
        Self::operation(string, &values, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let string = context.vm.read::<u16>().into();
        let value_count = context.vm.read::<u16>() as usize;
        let mut values = Vec::with_capacity(value_count);
        for _ in 0..value_count {
            values.push(context.vm.read::<u16>().into());
        }
        Self::operation(string, &values, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let string = context.vm.read::<u32>();
        let value_count = context.vm.read::<u32>() as usize;
        let mut values = Vec::with_capacity(value_count);
        for _ in 0..value_count {
            values.push(context.vm.read::<u32>());
        }
        Self::operation(string, &values, context)
    }
}
