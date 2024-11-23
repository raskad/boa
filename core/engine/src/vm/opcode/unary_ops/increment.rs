use crate::{
    value::{JsValue, Numeric},
    vm::{opcode::Operation, CompletionType},
    Context, JsBigInt, JsResult,
};

/// `ToNumeric` implements the Opcode Operation for `Opcode::ToNumeric`
#[derive(Debug, Clone, Copy)]
pub(crate) struct ToNumeric;

impl ToNumeric {
    fn operation(src: u32, dst: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + src) as usize].clone();

        let value = match &value {
            // Do not convert integers, because operations (like to_string) on numbers are slower!
            JsValue::Integer(_) => value,
            _ => JsValue::from(value.to_numeric(context)?),
        };

        context.vm.stack[(rp + dst) as usize] = value;
        Ok(CompletionType::Normal)
    }
}

impl Operation for ToNumeric {
    const NAME: &'static str = "ToNumeric";
    const INSTRUCTION: &'static str = "INST - ToNumeric";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u8>().into();
        let src = context.vm.read::<u8>().into();
        Self::operation(src, dst, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u16>().into();
        let src = context.vm.read::<u16>().into();
        Self::operation(src, dst, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u32>();
        let src = context.vm.read::<u32>();
        Self::operation(src, dst, context)
    }
}

/// `Inc` implements the Opcode Operation for `Opcode::Inc`
///
/// Operation:
///  - Unary `++` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Inc;

impl Inc {
    #[allow(clippy::unnecessary_wraps)]
    fn operation(src: u32, dst: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + src) as usize].clone();
        // let value = match value {
        //     JsValue::Integer(number) if number < i32::MAX => JsValue::from(number + 1),
        //     JsValue::Rational(value) => JsValue::from(value + 1f64),
        //     JsValue::BigInt(bigint) => JsBigInt::add(&bigint, &JsBigInt::one()).into(),
        //     _ => unreachable!("there is always a call to ToNumeric before Inc"),
        // };

        let (numeric, value) = match value {
            JsValue::Integer(number) if number < i32::MAX => {
                (JsValue::from(number), JsValue::from(number + 1))
            }
            _ => match value.to_numeric(context)? {
                Numeric::Number(number) => (JsValue::from(number), JsValue::from(number + 1f64)),
                Numeric::BigInt(bigint) => (
                    JsValue::from(bigint.clone()),
                    JsValue::from(JsBigInt::add(&bigint, &JsBigInt::one())),
                ),
            },
        };
        context.vm.stack[(rp + src) as usize] = numeric;
        context.vm.stack[(rp + dst) as usize] = value;
        Ok(CompletionType::Normal)
    }
}

impl Operation for Inc {
    const NAME: &'static str = "Inc";
    const INSTRUCTION: &'static str = "INST - Inc";
    const COST: u8 = 3;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u8>().into();
        let src = context.vm.read::<u8>().into();
        Self::operation(src, dst, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u16>().into();
        let src = context.vm.read::<u16>().into();
        Self::operation(src, dst, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let dst = context.vm.read::<u32>();
        let src = context.vm.read::<u32>();
        Self::operation(src, dst, context)
    }
}
