use crate::{
    builtins::Number,
    value::Numeric,
    vm::{opcode::Operation, CompletionType},
    Context, JsBigInt, JsResult,
};
use std::ops::Neg as StdNeg;

pub(crate) mod decrement;
pub(crate) mod increment;
pub(crate) mod logical;
pub(crate) mod void;

pub(crate) use decrement::*;
pub(crate) use increment::*;
pub(crate) use logical::*;
pub(crate) use void::*;

/// `TypeOf` implements the Opcode Operation for `Opcode::TypeOf`
///
/// Operation:
///  - Unary `typeof` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TypeOf;

impl TypeOf {
    fn operation(value_register: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value_register) as usize].clone();
        context.vm.stack[(rp + value_register) as usize] = value.js_type_of().into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for TypeOf {
    const NAME: &'static str = "TypeOf";
    const INSTRUCTION: &'static str = "INST - TypeOf";
    const COST: u8 = 1;

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

/// `Pos` implements the Opcode Operation for `Opcode::Pos`
///
/// Operation:
///  - Unary `+` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pos;

impl Pos {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        context.vm.stack[addr] = value.to_number(context)?.into();
        Ok(CompletionType::Normal)
    }
}

impl Operation for Pos {
    const NAME: &'static str = "Pos";
    const INSTRUCTION: &'static str = "INST - Pos";
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
        let value = context.vm.read::<u32>().into();
        Self::operation(value, context)
    }
}

/// `Neg` implements the Opcode Operation for `Opcode::Neg`
///
/// Operation:
///  - Unary `-` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Neg;

impl Neg {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        match value.to_numeric(context)? {
            Numeric::Number(number) => context.vm.stack[addr] = number.neg().into(),
            Numeric::BigInt(bigint) => context.vm.stack[addr] = JsBigInt::neg(&bigint).into(),
        }
        Ok(CompletionType::Normal)
    }
}

impl Operation for Neg {
    const NAME: &'static str = "Neg";
    const INSTRUCTION: &'static str = "INST - Neg";
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
        let value = context.vm.read::<u32>().into();
        Self::operation(value, context)
    }
}

/// `BitNot` implements the Opcode Operation for `Opcode::BitNot`
///
/// Operation:
///  - Unary bitwise `~` operator.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BitNot;

impl BitNot {
    fn operation(value: u32, context: &mut Context) -> JsResult<CompletionType> {
        let addr = (context.vm.frame().rp + value) as usize;
        let value = context.vm.stack[addr].clone();
        match value.to_numeric(context)? {
            Numeric::Number(number) => context.vm.stack[addr] = Number::not(number).into(),
            Numeric::BigInt(bigint) => context.vm.stack[addr] = JsBigInt::not(&bigint).into(),
        }
        Ok(CompletionType::Normal)
    }
}

impl Operation for BitNot {
    const NAME: &'static str = "BitNot";
    const INSTRUCTION: &'static str = "INST - BitNot";
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
        let value = context.vm.read::<u32>().into();
        Self::operation(value, context)
    }
}
