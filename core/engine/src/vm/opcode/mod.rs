use std::iter::FusedIterator;

/// The opcodes of the vm.
use crate::{
    vm::{CompletionType, Registers},
    Context, JsResult, JsValue,
};

// Operation modules
mod arguments;
mod r#await;
mod binary_ops;
mod call;
mod concat;
mod control_flow;
mod copy;
mod define;
mod delete;
mod environment;
mod generator;
mod get;
mod global;
mod iteration;
mod locals;
mod meta;
mod modifier;
mod new;
mod nop;
mod pop;
mod push;
mod rest_parameter;
mod set;
mod switch;
mod templates;
mod to;
mod unary_ops;
mod value;

// Operation structs
#[doc(inline)]
pub(crate) use arguments::*;
#[doc(inline)]
pub(crate) use binary_ops::*;
#[doc(inline)]
pub(crate) use call::*;
#[doc(inline)]
pub(crate) use concat::*;
#[doc(inline)]
pub(crate) use control_flow::*;
#[doc(inline)]
pub(crate) use copy::*;
#[doc(inline)]
pub(crate) use define::*;
#[doc(inline)]
pub(crate) use delete::*;
#[doc(inline)]
pub(crate) use environment::*;
#[doc(inline)]
pub(crate) use generator::*;
#[doc(inline)]
pub(crate) use get::*;
#[doc(inline)]
pub(crate) use global::*;
#[doc(inline)]
pub(crate) use iteration::*;
#[doc(inline)]
pub(crate) use locals::*;
#[doc(inline)]
pub(crate) use meta::*;
#[doc(inline)]
pub(crate) use modifier::*;
#[doc(inline)]
pub(crate) use new::*;
#[doc(inline)]
pub(crate) use nop::*;
#[doc(inline)]
pub(crate) use pop::*;
#[doc(inline)]
pub(crate) use push::*;
#[doc(inline)]
pub(crate) use r#await::*;
#[doc(inline)]
pub(crate) use rest_parameter::*;
#[doc(inline)]
pub(crate) use set::*;
#[doc(inline)]
pub(crate) use switch::*;
#[doc(inline)]
pub(crate) use templates::*;
#[doc(inline)]
pub(crate) use to::*;
#[doc(inline)]
pub(crate) use unary_ops::*;
#[doc(inline)]
pub(crate) use value::*;

use super::{code_block::Readable, GeneratorResumeKind};
use thin_vec::ThinVec;

/// Read type T from code.
///
/// # Safety
///
/// Does not check if read happens out-of-bounds.
pub(crate) const unsafe fn read_unchecked<T>(bytes: &[u8], offset: usize) -> T
where
    T: Readable,
{
    // Safety:
    // The function caller must ensure that the read is in bounds.
    //
    // This has to be an unaligned read because we can't guarantee that
    // the types are aligned.
    unsafe { bytes.as_ptr().add(offset).cast::<T>().read_unaligned() }
}

/// Read type T from code.
#[track_caller]
pub(crate) fn read<T>(bytes: &[u8], offset: usize) -> T
where
    T: Readable,
{
    assert!(offset + size_of::<T>() - 1 < bytes.len());

    // Safety: We checked that it is not an out-of-bounds read,
    // so this is safe.
    unsafe { read_unchecked(bytes, offset) }
}

/// Represents a varying operand kind.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub(crate) enum VaryingOperandKind {
    #[default]
    U8,
    U16,
    U32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VaryingOperand {
    kind: VaryingOperandKind,
    value: u32,
}

impl PartialEq for VaryingOperand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl VaryingOperand {
    #[must_use]
    pub(crate) fn u8(value: u8) -> Self {
        Self {
            kind: VaryingOperandKind::U8,
            value: u32::from(value),
        }
    }
    #[must_use]
    pub(crate) fn u16(value: u16) -> Self {
        Self {
            kind: VaryingOperandKind::U16,
            value: u32::from(value),
        }
    }
    #[must_use]
    pub(crate) const fn u32(value: u32) -> Self {
        Self {
            kind: VaryingOperandKind::U32,
            value,
        }
    }
    #[must_use]
    pub(crate) const fn value(self) -> u32 {
        self.value
    }
    #[must_use]
    pub(crate) const fn kind(self) -> VaryingOperandKind {
        self.kind
    }
}

trait BytecodeConversion: Sized {
    fn to_bytecode(&self, bytes: &mut Vec<u8>);
    fn from_bytecode(bytes: &[u8], pc: &mut usize, varying_kind: VaryingOperandKind) -> Self;
}

impl BytecodeConversion for VaryingOperand {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        match self.kind() {
            VaryingOperandKind::U8 => u8::to_bytecode(&(self.value() as u8), bytes),
            VaryingOperandKind::U16 => u16::to_bytecode(&(self.value() as u16), bytes),
            VaryingOperandKind::U32 => u32::to_bytecode(&self.value(), bytes),
        }
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, varying_kind: VaryingOperandKind) -> Self {
        match varying_kind {
            VaryingOperandKind::U8 => Self::u8(u8::from_bytecode(bytes, pc, varying_kind)),
            VaryingOperandKind::U16 => Self::u16(u16::from_bytecode(bytes, pc, varying_kind)),
            VaryingOperandKind::U32 => Self::u32(u32::from_bytecode(bytes, pc, varying_kind)),
        }
    }
}

impl BytecodeConversion for GeneratorResumeKind {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.push(*self as u8);
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<u8>(bytes, *pc);
        *pc += size_of::<Self>();
        JsValue::from(value).to_generator_resume_kind()
    }
}

impl BytecodeConversion for bool {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.push(u8::from(*self));
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<u8>(bytes, *pc);
        *pc += size_of::<Self>();
        value != 0
    }
}

impl BytecodeConversion for i8 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.push(*self as u8);
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for u8 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.push(*self);
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for i16 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for u16 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for i32 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for u32 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for i64 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for u64 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for f32 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for f64 {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.to_ne_bytes());
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let value = read::<Self>(bytes, *pc);
        *pc += size_of::<Self>();
        value
    }
}

impl BytecodeConversion for ThinVec<u32> {
    fn to_bytecode(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&(self.len() as u32).to_ne_bytes());
        for item in self {
            bytes.extend_from_slice(&item.to_ne_bytes());
        }
    }
    fn from_bytecode(bytes: &[u8], pc: &mut usize, _varying_kind: VaryingOperandKind) -> Self {
        let count = read::<u32>(bytes, *pc);
        *pc += size_of::<u32>();
        let mut result = Self::with_capacity(count as usize);
        for _ in 0..count {
            let item = read::<u32>(bytes, *pc);
            *pc += size_of::<u32>();
            result.push(item);
        }
        result
    }
}

/// Generate [`Opcode`]s and [`Instruction`]s enums.
macro_rules! generate_opcodes {
    ( name $name:ident ) => { $name };
    ( name $name:ident => $mapping:ident ) => { $mapping };

    // If if-block is empty use else-block, use if-block otherwise.
    { if { $($if:tt)+ } else { $($else:tt)* } } => { $($if)+ };
    { if { } else { $($else:tt)* } } => { $($else)* };

    (
        $(
            $(#[$inner:ident $($args:tt)*])*
            $Variant:ident $({
                $(
                    $(#[$fieldinner:ident $($fieldargs:tt)*])*
                    $FieldName:ident : $FieldType:ty
                ),*
            })? $(=> $mapping:ident)?
        ),*
        $(,)?
    ) => {
        /// The opcodes of the vm.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum Opcode {
            $(
                $(#[$inner $($args)*])*
                $Variant
            ),*
        }

        impl From<u8> for Opcode {
            #[inline]
            #[allow(non_upper_case_globals)]
            fn from(value: u8) -> Self {
                $(
                    const $Variant: u8 = Opcode::$Variant as u8;
                )*
                match value {
                    $($Variant => Self::$Variant),*
                }
            }
        }

        impl Opcode {
            const MAX: usize = 2usize.pow(8);

            // TODO: see if this can be exposed on all features.
            #[allow(unused)]
            const NAMES: [&'static str; Self::MAX * 3] = [
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::NAME),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::NAME),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::NAME),*
            ];

            /// Name of this opcode.
            #[must_use]
            // TODO: see if this can be exposed on all features.
            #[allow(unused)]
            pub(crate) const fn as_str(self) -> &'static str {
                Self::NAMES[self as usize]
            }

            const INSTRUCTIONS: [&'static str; Self::MAX * 3] = [
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::INSTRUCTION),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::INSTRUCTION),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::INSTRUCTION),*
            ];

            /// Name of the profiler event for this opcode.
            #[must_use]
            pub(crate) const fn as_instruction_str(self) -> &'static str {
                Self::INSTRUCTIONS[self as usize]
            }

            const SPEND_FNS: [fn(&mut Registers, &mut Context, &mut u32) -> JsResult<CompletionType>; Self::MAX] = [
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::spend_budget_and_execute),*,
            ];

            /// Spends the cost of this opcode into the provided budget and executes it.
            pub(super) fn spend_budget_and_execute(
                self,
                registers: &mut Registers,
                context: &mut Context,
                budget: &mut u32
            ) -> JsResult<CompletionType> {
                Self::SPEND_FNS[self as usize](registers, context, budget)
            }

            const COSTS: [u8; Self::MAX] = [
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::COST),*,
            ];

            /// Return the cost of this opcode.
            pub(super) const fn cost(self) -> u8 {
                Self::COSTS[self as usize]
            }

            const EXECUTE_FNS: [fn(&mut Registers, &mut Context) -> JsResult<CompletionType>; Self::MAX * 3] = [
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::execute),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::execute_u16),*,
                $(<generate_opcodes!(name $Variant $(=> $mapping)?)>::execute_u32),*
            ];

            pub(super) fn execute(self, registers: &mut Registers, context: &mut Context) -> JsResult<CompletionType> {
                Self::EXECUTE_FNS[self as usize](registers, context)
            }
        }

        /// This represents a VM instruction, it contains both opcode and operands.
        ///
        // TODO: An instruction should be a representation of a valid executable instruction (opcode + operands),
        //       so variants like `ReservedN`, or operand width prefix modifiers, idealy shouldn't
        //       be a part of `Instruction`.
        #[derive(Debug, Clone, PartialEq)]
        #[repr(u8)]
        pub(crate) enum Instruction {
            $(
                $(#[$inner $($args)*])*
                $Variant $({
                    $(
                        $(#[$fieldinner $($fieldargs)*])*
                        $FieldName : $FieldType
                    ),*
                })?
            ),*
        }

        impl Instruction {
            /// Convert [`Instruction`] to compact bytecode.
            #[inline]
            #[allow(dead_code)]
            pub(crate) fn to_bytecode(&self, bytes: &mut Vec<u8>) {
                match self {
                    $(
                        Self::$Variant $({
                            $( $FieldName ),*
                        })? => {
                            bytes.push(Opcode::$Variant as u8);
                            $({
                                $( BytecodeConversion::to_bytecode($FieldName, bytes); )*
                            })?
                        }
                    ),*
                }
            }

            /// Convert compact bytecode to [`Instruction`].
            ///
            /// # Panics
            ///
            /// If the provided bytecode is not valid.
            #[inline]
            #[must_use]
            pub(crate) fn from_bytecode(bytes: &[u8], pc: &mut usize, varying_kind: VaryingOperandKind) -> Self {
                let opcode = bytes[*pc].into();
                *pc += 1;
                match opcode {
                    $(
                        Opcode::$Variant => {
                            generate_opcodes!(
                                if {
                                    $({
                                        Self::$Variant {
                                            $(
                                                $FieldName: BytecodeConversion::from_bytecode(bytes, pc, varying_kind)
                                            ),*
                                        }
                                    })?
                                } else {
                                    Self::$Variant
                                }
                            )
                        }
                    ),*
                }
            }

            /// Get the [`Opcode`] of the [`Instruction`].
            #[inline]
            #[must_use]
            // TODO: see if this can be exposed on all features.
            #[allow(unused)]
            pub(crate) const fn opcode(&self) -> Opcode {
                match self {
                    $(
                        Self::$Variant $({ $( $FieldName: _ ),* })? => Opcode::$Variant
                    ),*
                }
            }
        }
    };
}

/// The `Operation` trait implements the execution code along with the
/// identifying Name and Instruction value for an Boa Opcode.
///
/// This trait should be implemented for a struct that corresponds with
/// any arm of the `OpCode` enum.
pub(crate) trait Operation {
    const NAME: &'static str;
    const INSTRUCTION: &'static str;
    const COST: u8;

    /// Execute opcode with [`VaryingOperandKind::U8`] sized [`VaryingOperand`]s.
    fn execute(registers: &mut Registers, context: &mut Context) -> JsResult<CompletionType>;

    /// Execute opcode with [`VaryingOperandKind::U16`] sized [`VaryingOperand`]s.
    ///
    /// By default if not implemented will call [`Reserved::execute_u16()`] which panics.
    fn execute_u16(registers: &mut Registers, context: &mut Context) -> JsResult<CompletionType> {
        Reserved::execute_u16(registers, context)
    }

    /// Execute opcode with [`VaryingOperandKind::U32`] sized [`VaryingOperand`]s.
    ///
    /// By default if not implemented will call [`Reserved::execute_u32()`] which panics.
    fn execute_u32(registers: &mut Registers, context: &mut Context) -> JsResult<CompletionType> {
        Reserved::execute_u32(registers, context)
    }

    /// Spends the cost of this operation into `budget` and runs `execute`.
    fn spend_budget_and_execute(
        registers: &mut Registers,
        context: &mut Context,
        budget: &mut u32,
    ) -> JsResult<CompletionType> {
        *budget = budget.saturating_sub(u32::from(Self::COST));
        Self::execute(registers, context)
    }
}

generate_opcodes! {
    /// Pop the top value from the stack.
    ///
    /// Operands:
    ///
    /// Stack: value **=>**
    Pop,

    /// Push integer `0` on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `0`
    PushZero { dst: VaryingOperand },

    /// Push integer `1` on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `1`
    PushOne { dst: VaryingOperand },

    /// Push `i8` value on the stack.
    ///
    /// Operands: value: `i8`
    ///
    /// Stack: **=>** value
    PushInt8 { dst: VaryingOperand, value: i8 },

    /// Push i16 value on the stack.
    ///
    /// Operands: value: `i16`
    ///
    /// Stack: **=>** value
    PushInt16 { dst: VaryingOperand, value: i16 },

    /// Push i32 value on the stack.
    ///
    /// Operands: value: `i32`
    ///
    /// Stack: **=>** value
    PushInt32 { dst: VaryingOperand, value: i32 },

    /// Push `f32` value on the stack.
    ///
    /// Operands: value: `f32`
    ///
    /// Stack: **=>** value
    PushFloat { dst: VaryingOperand, value: f32 },

    /// Push `f64` value on the stack.
    ///
    /// Operands: value: `f64`
    ///
    /// Stack: **=>** value
    PushDouble { dst: VaryingOperand, value: f64 },

    /// Push `NaN` integer on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `NaN`
    PushNaN { dst: VaryingOperand },

    /// Push `Infinity` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `Infinity`
    PushPositiveInfinity { dst: VaryingOperand },

    /// Push `-Infinity` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `-Infinity`
    PushNegativeInfinity { dst: VaryingOperand },

    /// Push `null` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `null`
    PushNull { dst: VaryingOperand },

    /// Push `true` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `true`
    PushTrue { dst: VaryingOperand },

    /// Push `false` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `false`
    PushFalse { dst: VaryingOperand },

    /// Push `undefined` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `undefined`
    PushUndefined { dst: VaryingOperand },

    /// Push literal value on the stack.
    ///
    /// Like strings and bigints. The index operand is used to index into the `literals`
    /// array to get the value.
    ///
    /// Operands: index: `VaryingOperand`
    ///
    /// Stack: **=>** (`literals[index]`)
    PushLiteral { dst: VaryingOperand, index: VaryingOperand },

    /// Push regexp value on the stack.
    ///
    /// Operands: pattern_index: `VaryingOperand`, flags: `VaryingOperand`
    ///
    /// Stack: **=>** regexp
    PushRegExp { dst: VaryingOperand, pattern_index: VaryingOperand, flags_index: VaryingOperand },

    /// Push empty object `{}` value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    PushEmptyObject { dst: VaryingOperand },

    /// Get the prototype of a superclass and push it on the stack.
    ///
    /// Additionally this sets the `[[prototype]]` of the class and the `DERIVED` flag.
    ///
    /// Operands:
    ///
    /// Stack: class, superclass **=>** class, superclass.prototype
    PushClassPrototype {
        dst: VaryingOperand,
        class: VaryingOperand,
        superclass: VaryingOperand
    },

    /// Set the prototype of a class object.
    ///
    /// Operands:
    ///
    /// Stack: class, prototype **=>** class.prototype
    SetClassPrototype {
        dst: VaryingOperand,
        prototype: VaryingOperand,
        class: VaryingOperand
    },

    /// Set home object internal slot of an object literal method.
    ///
    /// Operands:
    ///
    /// Stack: home, function **=>**
    SetHomeObject {
        function: VaryingOperand,
        home: VaryingOperand
    },

    /// Set the prototype of an object if the value is an object or null.
    ///
    /// Operands:
    ///
    /// Stack: object, value **=>**
    SetPrototype {
        object: VaryingOperand,
        value: VaryingOperand
    },

    /// Push an empty array value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `[]`
    PushNewArray { array: VaryingOperand },

    /// Push a value to an array.
    ///
    /// Operands:
    ///
    /// Stack: array, value **=>** array
    PushValueToArray {
        value: VaryingOperand,
        array: VaryingOperand
    },

    /// Push an empty element/hole to an array.
    ///
    /// Operands:
    ///
    /// Stack: array **=>** array
    PushElisionToArray { array: VaryingOperand },

    /// Push all iterator values to an array.
    ///
    /// Operands:
    ///
    /// Stack: array, iterator, next_method **=>** array
    PushIteratorToArray { array: VaryingOperand },

    /// Binary `+` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs + rhs)
    Add { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `-` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs - rhs)
    Sub { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `/` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs / rhs)
    Div { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `*` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs * rhs)
    Mul { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `%` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs % rhs)
    Mod { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `**` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs ** rhs)
    Pow { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `>>` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs >> rhs)
    ShiftRight { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `<<` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** `(lhs << rhs)`
    ShiftLeft { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `>>>` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs >>> rhs)
    UnsignedShiftRight { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary bitwise `|` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs | rhs)
    BitOr { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary bitwise `&` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs & rhs)
    BitAnd { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary bitwise `^` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs ^ rhs)
    BitXor { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Unary bitwise `~` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** ~value
    BitNot { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `in` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs `in` rhs)
    In { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `in` operator for private names.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: rhs **=>** (private_name `in` rhs)
    InPrivate { dst: VaryingOperand, index: VaryingOperand, rhs: VaryingOperand },

    /// Binary `==` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs `==` rhs)
    Eq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `===` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs `===` rhs)
    StrictEq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `!=` operator.
    NotEq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `!==` operator.
    StrictNotEq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `>` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs > rhs)
    GreaterThan { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `>=` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs >= rhs)
    GreaterThanOrEq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `<` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** `(lhs < rhs)`
    LessThan { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `<=` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** `(lhs <= rhs)`
    LessThanOrEq { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary `instanceof` operator.
    ///
    /// Operands:
    ///
    /// Stack: lhs, rhs **=>** (lhs instanceof rhs)
    InstanceOf { dst: VaryingOperand, lhs: VaryingOperand, rhs: VaryingOperand },

    /// Binary logical `&&` operator.
    ///
    /// This is a short-circuit operator, if the `lhs` value is `false`, then it jumps to `exit` address.
    ///
    /// Operands: exit: `u32`
    ///
    /// Stack: lhs, rhs **=>** (lhs && rhs)
    LogicalAnd { exit: u32, lhs: VaryingOperand },

    /// Binary logical `||` operator.
    ///
    /// This is a short-circuit operator, if the `lhs` value is `true`, then it jumps to `exit` address.
    ///
    /// Operands: exit: `u32`
    ///
    /// Stack: lhs, rhs **=>** (lhs || rhs)
    LogicalOr { exit: u32, lhs: VaryingOperand },

    /// Binary `??` operator.
    ///
    /// This is a short-circuit operator, if the `lhs` value is **not** `null` or `undefined`,
    /// then it jumps to `exit` address.
    ///
    /// Operands: exit: `u32`
    ///
    /// Stack: lhs, rhs **=>** (lhs ?? rhs)
    Coalesce { exit: u32, lhs: VaryingOperand },

    /// Unary `typeof` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (`typeof` value)
    TypeOf { value: VaryingOperand },

    /// Unary logical `!` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (!value)
    LogicalNot,

    /// Unary `+` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (+value)
    Pos,

    /// Unary `-` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (-value)
    Neg,

    /// Unary `++` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (value + 1)
    Inc { dst: VaryingOperand, src: VaryingOperand },

    /// Unary `--` operator.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** (value - 1)
    Dec { dst: VaryingOperand, src: VaryingOperand },

    /// Declare `var` type variable.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>**
    DefVar { index: VaryingOperand },

    /// Declare and initialize `var` type variable.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: value **=>**
    DefInitVar { value: VaryingOperand, index: VaryingOperand },

    /// Initialize a lexical binding.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: value **=>**
    PutLexicalValue { value: VaryingOperand, index: VaryingOperand },

    /// Throws an error because the binding access is illegal.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>**
    ThrowMutateImmutable { index: VaryingOperand },

    /// Get i-th argument of the current frame.
    ///
    /// Returns `undefined` if `arguments.len()` < `index`.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>** value
    GetArgument { index: VaryingOperand, dst: VaryingOperand },

    /// Find a binding on the environment chain and push its value.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>** value
    GetName { value: VaryingOperand, index: VaryingOperand },

    /// Find a binding on the environment and set the `current_binding` of the current frame.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>**
    GetLocator { index: VaryingOperand },

    /// Find a binding on the environment chain and push its value to the stack and its
    /// `BindingLocator` to the `bindings_stack`.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>** value
    GetNameAndLocator { value: VaryingOperand, index: VaryingOperand },

    /// Find a binding on the environment chain and push its value. If the binding does not exist push undefined.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>** value
    GetNameOrUndefined { value: VaryingOperand, index: VaryingOperand },

    /// Find a binding on the environment chain and assign its value.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: value **=>**
    SetName { value: VaryingOperand, index: VaryingOperand },

    /// Assigns a value to the binding pointed by the top of the `bindings_stack`.
    ///
    /// Stack: value **=>**
    SetNameByLocator { value: VaryingOperand },

    /// Deletes a property of the global object.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: **=>** deleted
    DeleteName { value: VaryingOperand, index: VaryingOperand },

    /// Get a property by name from an object an push it on the stack.
    ///
    /// Like `object.name`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, receiver **=>** value
    GetPropertyByName {
        dst: VaryingOperand,
        receiver: VaryingOperand,
        value: VaryingOperand,
        index: VaryingOperand
    },

    /// Get a property by value from an object an push it on the stack.
    ///
    /// Like `object[key]`
    ///
    /// Operands:
    ///
    /// Stack: object, receiver, key **=>** value
    GetPropertyByValue { dst: VaryingOperand, key: VaryingOperand, receiver: VaryingOperand, object: VaryingOperand },

    /// Get a property by value from an object an push the key and value on the stack.
    ///
    /// Like `object[key]`
    ///
    /// Operands:
    ///
    /// Stack: object, receiver, key **=>** key, value
    GetPropertyByValuePush { dst: VaryingOperand, key: VaryingOperand, receiver: VaryingOperand, object: VaryingOperand },

    /// Sets a property by name of an object.
    ///
    /// Like `object.name = value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, receiver, value **=>**
    SetPropertyByName {
        value: VaryingOperand,
        receiver: VaryingOperand,
        object: VaryingOperand,
        index: VaryingOperand
    },

    /// Sets the name of a function object.
    ///
    /// This operation is corresponds to the `SetFunctionName` abstract operation in the [spec].
    ///
    ///  The prefix operand is mapped as follows:
    /// * 0 -> no prefix
    /// * 1 -> "get "
    /// * 2 -> "set "
    ///
    /// Operands: prefix: `u8`
    ///
    /// Stack: name, function **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-setfunctionname
    SetFunctionName { function: VaryingOperand, name: VaryingOperand, prefix: u8 },

    /// Defines a own property of an object by name.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    DefineOwnPropertyByName { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Defines a static class method by name.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, function **=>**
    DefineClassStaticMethodByName {
        function: VaryingOperand,
        class: VaryingOperand,
        index: VaryingOperand
    },

    /// Defines a class method by name.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class_proto, function **=>**
    DefineClassMethodByName {
        function: VaryingOperand,
        class_proto: VaryingOperand,
        index: VaryingOperand
    },

    /// Sets a property by value of an object.
    ///
    /// Like `object[key] = value`
    ///
    /// Operands:
    ///
    /// Stack: object, receiver, key, value **=>**
    SetPropertyByValue {
        value: VaryingOperand,
        key: VaryingOperand,
        receiver: VaryingOperand,
        object: VaryingOperand
    },

    /// Defines a own property of an object by value.
    ///
    /// Operands:
    ///
    /// Stack: object, key, value **=>**
    DefineOwnPropertyByValue {
        value: VaryingOperand,
        key: VaryingOperand,
        object: VaryingOperand
    },

    /// Defines a static class method by value.
    ///
    /// Operands:
    ///
    /// Stack: class, key, function **=>**
    DefineClassStaticMethodByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class: VaryingOperand
    },

    /// Defines a class method by value.
    ///
    /// Operands:
    ///
    /// Stack: class_proto, key, function **=>**
    DefineClassMethodByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class_proto: VaryingOperand
    },

    /// Sets a getter property by name of an object.
    ///
    /// Like `get name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPropertyGetterByName { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Defines a static getter class method by name.
    ///
    /// Like `static get name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, function **=>**
    DefineClassStaticGetterByName {
        function: VaryingOperand,
        class: VaryingOperand,
        index: VaryingOperand
    },

    /// Defines a getter class method by name.
    ///
    /// Like `get name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class_proto, function **=>**
    DefineClassGetterByName {
        function: VaryingOperand,
        class_proto: VaryingOperand,
        index: VaryingOperand
    },

    /// Sets a getter property by value of an object.
    ///
    /// Like `get [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: object, key, value **=>**
    SetPropertyGetterByValue {
        value: VaryingOperand,
        key: VaryingOperand,
        object: VaryingOperand
    },

    /// Defines a static getter class method by value.
    ///
    /// Like `static get [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: class, key, function **=>**
    DefineClassStaticGetterByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class: VaryingOperand
    },

    /// Defines a getter class method by value.
    ///
    /// Like `get [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: class_proto, key, function **=>**
    DefineClassGetterByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class_proto: VaryingOperand
    },

    /// Sets a setter property by name of an object.
    ///
    /// Like `set name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPropertySetterByName { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Defines a static setter class method by name.
    ///
    /// Like `static set name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, function **=>**
    DefineClassStaticSetterByName {
        function: VaryingOperand,
        class: VaryingOperand,
        index: VaryingOperand
    },

    /// Defines a setter class method by name.
    ///
    /// Like `set name() value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class_proto, function **=>**
    DefineClassSetterByName {
        function: VaryingOperand,
        class_proto: VaryingOperand,
        index: VaryingOperand
    },

    /// Sets a setter property by value of an object.
    ///
    /// Like `set [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: object, key, value **=>**
    SetPropertySetterByValue {
        value: VaryingOperand,
        key: VaryingOperand,
        object: VaryingOperand
    },

    /// Defines a static setter class method by value.
    ///
    /// Like `static set [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: class, key, function **=>**
    DefineClassStaticSetterByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class: VaryingOperand
    },

    /// Defines a setter class method by value.
    ///
    /// Like `set [key]() value`
    ///
    /// Operands:
    ///
    /// Stack: class_proto, key, function **=>**
    DefineClassSetterByValue {
        function: VaryingOperand,
        key: VaryingOperand,
        class_proto: VaryingOperand
    },

    /// Set the value of a private property of an object by it's name.
    ///
    /// Like `obj.#name = value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPrivateField { value: VaryingOperand, object: VaryingOperand, index: VaryingOperand },

    /// Define a private property of a class constructor by it's name.
    ///
    /// Like `#name = value`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    DefinePrivateField { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Set a private method of a class constructor by it's name.
    ///
    /// Like `#name() {}`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPrivateMethod { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Set a private setter property of a class constructor by it's name.
    ///
    /// Like `set #name() {}`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPrivateSetter { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Set a private getter property of a class constructor by it's name.
    ///
    /// Like `get #name() {}`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object, value **=>**
    SetPrivateGetter { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Get a private property by name from an object an push it on the stack.
    ///
    /// Like `object.#name`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object **=>** value
    GetPrivateField { dst: VaryingOperand, object: VaryingOperand, index: VaryingOperand },

    /// Push a field to a class.
    ///
    /// Operands: is_anonymous_function: `bool`
    ///
    /// Stack: class, name, function **=>**
    PushClassField { class: VaryingOperand, name: VaryingOperand, function: VaryingOperand, is_anonymous_function: bool },

    /// Push a private field to the class.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, field_function **=>**
    PushClassFieldPrivate { class: VaryingOperand, function: VaryingOperand, index: VaryingOperand },

    /// Push a private getter to the class.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, getter **=>**
    PushClassPrivateGetter { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Push a private setter to the class.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, setter **=>**
    PushClassPrivateSetter { object: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Push a private method to the class.
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: class, class_proto, method **=>**
    PushClassPrivateMethod { object: VaryingOperand, proto: VaryingOperand, value: VaryingOperand, index: VaryingOperand },

    /// Deletes a property by name of an object.
    ///
    /// Like `delete object.key`
    ///
    /// Operands: index: `u32`
    ///
    /// Stack: object **=>** deleted
    DeletePropertyByName { object: VaryingOperand, index: VaryingOperand },

    /// Deletes a property by value of an object.
    ///
    /// Like `delete object[key]`
    ///
    /// Operands:
    ///
    /// Stack: object, key **=>** deleted
    DeletePropertyByValue { object: VaryingOperand, key: VaryingOperand },

    /// Throws an error when trying to delete a property of `super`
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    DeleteSuperThrow,

    /// Copy all properties of one object to another object.
    ///
    /// Operands: excluded_key_count: `VaryingOperand`, excluded_key_count_computed: `VaryingOperand`
    ///
    /// Stack: excluded_key_computed_0 ... excluded_key_computed_n, source, value, excluded_key_0 ... excluded_key_n **=>** value
    CopyDataProperties { object: VaryingOperand, source: VaryingOperand, excluded_key_count: VaryingOperand },

    /// Call ToPropertyKey on the value on the stack.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** key
    ToPropertyKey { value: VaryingOperand, dst: VaryingOperand },

    /// Unconditional jump to address.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: **=>**
    Jump { address: u32 },

    /// Conditional jump to address.
    ///
    /// If the value popped is [`truthy`][truthy] then jump to `address`.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: cond **=>**
    ///
    /// [truthy]: https://developer.mozilla.org/en-US/docs/Glossary/Truthy
    JumpIfTrue { address: u32, value: VaryingOperand },

    /// Conditional jump to address.
    ///
    /// If the value popped is [`falsy`][falsy] then jump to `address`.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: cond **=>**
    ///
    /// [falsy]: https://developer.mozilla.org/en-US/docs/Glossary/Falsy
    JumpIfFalse { address: u32, value: VaryingOperand },

    /// Conditional jump to address.
    ///
    /// If the value popped is not undefined jump to `address`.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: value **=>** value
    JumpIfNotUndefined { address: u32, value: VaryingOperand },

    /// Conditional jump to address.
    ///
    /// If the value popped is undefined jump to `address`.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: value **=>** value
    JumpIfNullOrUndefined { address: u32, value: VaryingOperand },

    /// Jump table that jumps depending on top value of the stack.
    ///
    /// This is used to handle special cases when we call `continue`, `break` or `return` in a try block,
    /// that has finally block.
    ///
    /// Operands: default: `u32`, count: `u32`, address: `u32` * count
    ///
    /// Stack: value: [`i32`] **=>**
    JumpTable { default: u32, addresses: ThinVec<u32> },

    /// Throw exception.
    ///
    /// This sets pending exception and searches for an exception handler.
    ///
    /// Operands:
    ///
    /// Stack: value **=>**
    Throw { value: VaryingOperand },

    /// Rethrow thrown exception.
    ///
    /// This is also used to handle generator `return()` call, we throw an empty exception, by setting pending exception to [`None`],
    /// propagating it and calling finally code until there is no exception handler left, in that case we consume the empty exception and return
    /// from the generator.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    ReThrow,

    /// Get the thrown pending exception (if it's set) and push on the stack.
    ///
    /// If there is no pending exception, which can happend if we are handling `return()` call on generator,
    /// then we rethrow the empty exception. See [`Opcode::ReThrow`].
    ///
    /// Operands:
    ///
    /// Stack: **=>** exception
    Exception { dst: VaryingOperand },

    /// Get the thrown pending exception if it's set and push `true`, otherwise push only `false`.
    ///
    /// Operands:
    ///
    /// Stack: **=>** (`true`, exception) or `false`
    MaybeException { has_exception: VaryingOperand, exception: VaryingOperand },

    /// Throw a new `TypeError` exception
    ///
    /// Operands: message: u32
    ///
    /// Stack: **=>**
    ThrowNewTypeError { message: VaryingOperand },

    /// Throw a new `SyntaxError` exception
    ///
    /// Operands: message: u32
    ///
    /// Stack: **=>**
    ThrowNewSyntaxError { message: VaryingOperand },

    /// Pushes `this` value
    ///
    /// Operands:
    ///
    /// Stack: **=>** this
    This { dst: VaryingOperand },

    /// Pushes `this` value that is related to the object environment of the given binding
    ///
    /// Operands: index: `VaryingOperand`
    ///
    /// Stack: **=>** value
    ThisForObjectEnvironmentName { dst: VaryingOperand, index: VaryingOperand },

    /// Pushes the current `super` value to the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** super
    Super { dst: VaryingOperand },

    /// Get the super constructor and the new target of the current environment.
    ///
    /// Operands:
    ///
    /// Stack: **=>** super_constructor
    SuperCallPrepare { dst: VaryingOperand },

    /// Execute the `super()` method.
    ///
    /// Operands: argument_count: `u32`
    ///
    /// Stack: super_constructor, argument_1, ... argument_n **=>**
    SuperCall { argument_count: VaryingOperand },

    /// Execute the `super()` method where the arguments contain spreads.
    ///
    /// Operands:
    ///
    /// Stack: super_constructor, arguments_array **=>**
    SuperCallSpread,

    /// Execute the `super()` method when no constructor of the class is defined.
    ///
    /// Operands:
    ///
    /// Stack: argument_n, ... argument_1 **=>**
    SuperCallDerived,

    /// Binds `this` value and initializes the instance elements.
    ///
    /// Performs steps 7-12 of [`SuperCall: super Arguments`][spec]
    ///
    /// Operands:
    ///
    /// Stack: result **=>** result
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-super-keyword-runtime-semantics-evaluation
    BindThisValue { value: VaryingOperand },

    /// Dynamically import a module.
    ///
    /// Operands:
    ///
    /// Stack: specifier **=>** promise
    ImportCall { value: VaryingOperand },

    /// Pop the two values of the stack, strict equal compares the two values,
    /// if true jumps to address, otherwise push the second pop'ed value.
    ///
    /// Operands: address: `u32`
    ///
    /// Stack: value, cond **=>** cond (if `cond !== value`).
    Case { address: u32, value: VaryingOperand, condition: VaryingOperand },

    /// Get function from the pre-compiled inner functions.
    ///
    /// Operands: index: `VaryingOperand`
    ///
    /// Stack: **=>** func
    GetFunction { dst: VaryingOperand, index: VaryingOperand },

    /// Call a function named "eval".
    ///
    /// Operands: argument_count: `VaryingOperand`, scope_index: `VaryingOperand`
    ///
    /// Stack: this, func, argument_1, ... argument_n **=>** result
    CallEval { argument_count: VaryingOperand, scope_index: VaryingOperand },

    /// Call a function named "eval" where the arguments contain spreads.
    ///
    /// Operands:
    ///
    /// Stack: this, func, arguments_array **=>** result
    CallEvalSpread { index: VaryingOperand },

    /// Call a function.
    ///
    /// Operands: argument_count: `u32`
    ///
    /// Stack: this, func, argument_1, ... argument_n **=>** result
    Call { argument_count: VaryingOperand },

    /// Call a function where the arguments contain spreads.
    ///
    /// Operands:
    ///
    /// Stack: this, func, arguments_array **=>** result
    CallSpread,

    /// Call construct on a function.
    ///
    /// Operands: argument_count: `u32`
    ///
    /// Stack: func, argument_1, ... argument_n **=>** result
    New { argument_count: VaryingOperand },

    /// Call construct on a function where the arguments contain spreads.
    ///
    /// Operands:
    ///
    /// Stack: arguments_array, func **=>** result
    NewSpread,

    /// Check return from a function.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    CheckReturn,

    /// Return from a function.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    Return,

    /// Close an async generator function.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    AsyncGeneratorClose,

    /// Creates the generator object and yields.
    ///
    /// Operands: async: `u8`
    ///
    /// Stack: **=>** resume_kind
    Generator { r#async: bool },

    /// Set return value of a function.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    SetAccumulator { register: VaryingOperand },

    // Set return value of a function.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    SetRegisterFromAccumulator { register: VaryingOperand },

    /// Move value of operand `src` to register `dst`.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    Move { dst: VaryingOperand, src: VaryingOperand },

    /// Pop value from the stack and push to register `dst`
    ///
    /// Operands:
    ///
    /// Stack: value **=>**
    PopIntoRegister { dst: VaryingOperand },

    /// Copy value at register `src` and push it into the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** value
    PushFromRegister { src: VaryingOperand },

    /// Pop value from the stack and push to a local binding register `dst`.
    ///
    /// Operands:
    ///
    /// Stack: value **=>**
    PopIntoLocal { src: VaryingOperand, dst: VaryingOperand },

    /// Copy value at local binding register `src` and push it into the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** value
    PushFromLocal { src: VaryingOperand, dst: VaryingOperand },

    /// Push a declarative environment.
    ///
    /// Operands: index: `VaryingOperand`
    ///
    /// Stack: **=>**
    PushScope { index: VaryingOperand },

    /// Push an object environment.
    ///
    /// Operands:
    ///
    /// Stack: object **=>**
    PushObjectEnvironment { object: VaryingOperand },

    /// Pop the current environment.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    PopEnvironment,

    /// Increment loop iteration count.
    ///
    /// Used for limiting the loop iteration.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    IncrementLoopIteration,

    /// Creates the ForInIterator of an object.
    ///
    /// Stack: object **=>**
    ///
    /// Iterator Stack: `iterator`
    CreateForInIterator { value: VaryingOperand },

    /// Gets the iterator of an object.
    ///
    /// Operands:
    ///
    /// Stack: object **=>**
    ///
    /// Iterator Stack: `iterator`
    GetIterator { value: VaryingOperand },

    /// Gets the async iterator of an object.
    ///
    /// Operands:
    ///
    /// Stack: object **=>**
    ///
    /// Iterator Stack: `iterator`
    GetAsyncIterator { value: VaryingOperand },

    /// Calls the `next` method of `iterator`, updating its record with the next value.
    ///
    /// Operands:
    ///
    /// Iterator Stack: `iterator` **=>** `iterator`
    IteratorNext,

    /// Returns `true` if the current iterator is done, or `false` otherwise
    ///
    /// Stack: **=>** done
    ///
    /// Iterator Stack: `iterator` **=>** `iterator`
    IteratorDone { done: VaryingOperand },

    /// Finishes the call to `Opcode::IteratorNext` within a `for await` loop by setting the current
    /// result of the current iterator.
    ///
    /// Operands:
    ///
    /// Stack: `next_result`, `resume_kind` **=>** `resume_kind`
    ///
    /// Iterator Stack: iterator **=>** iterator
    IteratorFinishAsyncNext { resume_kind: VaryingOperand, value: VaryingOperand },

    /// Gets the `value` property of the current iterator record.
    ///
    /// Stack: **=>** `value`
    ///
    /// Iterator Stack: `iterator` **=>** `iterator`
    IteratorValue { value: VaryingOperand },

    /// Gets the last iteration result of the current iterator record.
    ///
    /// Stack: **=>** `result`
    ///
    /// Iterator Stack: `iterator` **=>** `iterator`
    IteratorResult { value: VaryingOperand },

    /// Consume the iterator and construct and array with all the values.
    ///
    /// Operands:
    ///
    /// Stack: **=>** array
    ///
    /// Iterator Stack: `iterator` **=>** `iterator`
    IteratorToArray { array: VaryingOperand },

    /// Pushes `true` to the stack if the iterator stack is empty.
    ///
    /// Stack:
    /// - **=>** `is_empty`
    ///
    /// Iterator Stack:
    /// - **=>**
    IteratorStackEmpty { empty: VaryingOperand },

    /// Creates a new iterator result object.
    ///
    /// Operands:
    /// - done: bool (codified as u8 with `0` -> `false` and `!0` -> `true`)
    ///
    /// Stack:
    /// - value **=>**
    ///
    CreateIteratorResult { value: VaryingOperand, done: bool },

    /// Calls `return` on the current iterator and returns the result.
    ///
    /// Stack: **=>** return_val (if return is a method), is_return_method
    ///
    /// Iterator Stack: `iterator` **=>**
    IteratorReturn { value: VaryingOperand, called: VaryingOperand },

    /// Concat multiple stack objects into a string.
    ///
    /// Operands: value_count: `u32`
    ///
    /// Stack: `value_1`,...`value_n` **=>** `string`
    ConcatToString { string: VaryingOperand, value_count: VaryingOperand },

    /// Require the stack value to be neither null nor undefined.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** value
    ValueNotNullOrUndefined { value: VaryingOperand },

    /// Initialize the rest parameter value of a function from the remaining arguments.
    ///
    /// Operands:
    ///
    /// Stack: `argument_1` .. `argument_n` **=>** `array`
    RestParameterInit { dst: VaryingOperand },

    /// Yields from the current generator execution.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** resume_kind, received
    GeneratorYield { value: VaryingOperand },

    /// Resumes the current generator function.
    ///
    /// If the `resume_kind` is `Throw`, then the value is poped and thrown, otherwise if `Return`
    /// we pop the value, set it as the return value and throw and empty exception. See [`Opcode::ReThrow`].
    ///
    /// Operands:
    ///
    /// Stack: `resume_kind`, value **=>** value
    GeneratorNext { resume_kind: VaryingOperand, value: VaryingOperand },

    /// Yields from the current async generator execution.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** received
    AsyncGeneratorYield { value: VaryingOperand },

    /// Create a promise capacity for an async function, if not already set.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    CreatePromiseCapability,

    /// Resolves or rejects the promise capability of an async function.
    ///
    /// If the pending exception is set, reject and rethrow the exception, otherwise resolve.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    CompletePromiseCapability,

    /// Jumps to the specified address if the resume kind is not equal.
    ///
    /// Operands: `exit`: `u32`, `resume_kind`: `u8`.
    ///
    /// Stack: `resume_kind` **=>** `resume_kind`
    JumpIfNotResumeKind { exit: u32, resume_kind: GeneratorResumeKind, value: VaryingOperand },

    /// Delegates the current async generator function to another iterator.
    ///
    /// Operands: throw_method_undefined: `u32`, return_method_undefined: `u32`
    ///
    /// Stack: received **=>** result
    GeneratorDelegateNext {
        throw_method_undefined: u32,
        return_method_undefined: u32,
        value: VaryingOperand,
        resume_kind: VaryingOperand,
        is_return: VaryingOperand
    },

    /// Resume the async generator with yield delegate logic after it awaits a value.
    ///
    /// Operands: return: `u32`, exit: `u32`
    ///
    /// Stack: is_return, received **=>** value
    GeneratorDelegateResume {
        r#return: u32,
        exit: u32,
        value: VaryingOperand,
        resume_kind: VaryingOperand,
        is_return: VaryingOperand
    },

    /// Stops the current async function and schedules it to resume later.
    ///
    /// Operands:
    ///
    /// Stack: promise **=>** received
    Await { value: VaryingOperand },

    /// Push the current new target to the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `new.target`
    NewTarget { dst: VaryingOperand },

    /// Push the current `import.meta` to the stack.
    ///
    /// Operands:
    ///
    /// Stack: **=>** `import.meta`
    ImportMeta { dst: VaryingOperand },

    /// Pushes `true` to the stack if the top stack value is an object, or `false` otherwise.
    ///
    /// Operands:
    ///
    /// Stack: value **=>** is_object
    IsObject { value: VaryingOperand },

    /// Lookup if a tagged template object is cached and skip the creation if it is.
    ///
    /// Operands: exit: `u32`, site: `u64`
    ///
    /// Stack: **=>** template (if cached)
    TemplateLookup { exit: u32, site: u64, dst: VaryingOperand },

    /// Create a new tagged template object and cache it.
    ///
    /// Operands: count: `VaryingOperand`, site: `u64`
    ///
    /// Stack: count * (cooked_value, raw_value) **=>** template
    TemplateCreate { site: u64, dst: VaryingOperand, count: VaryingOperand },

    /// Push a private environment.
    ///
    /// Operands: count: `u32`, count * name_index: `u32`
    ///
    /// Stack: class **=>** class
    PushPrivateEnvironment { class: VaryingOperand, name_indices: ThinVec<u32> },

    /// Pop a private environment.
    ///
    /// Operands:
    ///
    /// Stack: **=>**
    PopPrivateEnvironment,

    /// Creates a mapped `arguments` object.
    ///
    /// Performs [`10.4.4.7 CreateMappedArgumentsObject ( func, formals, argumentsList, env )`]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createunmappedargumentsobject
    ///
    /// Operands:
    ///
    /// Stack: **=>** `arguments`
    CreateMappedArgumentsObject { dst: VaryingOperand },

    /// Creates an unmapped `arguments` object.
    ///
    /// Performs: [`10.4.4.6 CreateUnmappedArgumentsObject ( argumentsList )`]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createmappedargumentsobject
    ///
    /// Stack: **=>** `arguments`
    CreateUnmappedArgumentsObject { dst: VaryingOperand },

    /// Performs [`HasRestrictedGlobalProperty ( N )`][spec]
    ///
    /// Operands: `index`: u32
    ///
    /// Stack: **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-hasrestrictedglobalproperty
    HasRestrictedGlobalProperty { dst: VaryingOperand, index: VaryingOperand },

    /// Performs [`CanDeclareGlobalFunction ( N )`][spec]
    ///
    /// Operands: `index`: u32
    ///
    /// Stack: **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-candeclareglobalfunction
    CanDeclareGlobalFunction { dst: VaryingOperand, index: VaryingOperand },

    /// Performs [`CanDeclareGlobalVar ( N )`][spec]
    ///
    /// Operands: `index`: u32
    ///
    /// Stack: **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-candeclareglobalvar
    CanDeclareGlobalVar { dst: VaryingOperand, index: VaryingOperand },

    /// Performs [`CreateGlobalFunctionBinding ( N, V, D )`][spec]
    ///
    /// Operands: configurable: `bool`, `index`: `VaryingOperand`
    ///
    /// Stack: `function` **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createglobalfunctionbinding
    CreateGlobalFunctionBinding { function: VaryingOperand, configurable: bool, index: VaryingOperand },

    /// Performs [`CreateGlobalVarBinding ( N, V, D )`][spec]
    ///
    /// Operands: configurable: `bool`, `index`: `VaryingOperand`
    ///
    /// Stack: **=>**
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createglobalvarbinding
    CreateGlobalVarBinding { configurable: bool, index: VaryingOperand },

    /// Opcode prefix modifier, makes all [`VaryingOperand`]s of an instruction [`u16`] sized.
    ///
    /// Operands: opcode (operands if any).
    ///
    /// Stack: The stack changes based on the opcode that is being prefixed.
    U16Operands,

    /// Opcode prefix modifier, [`Opcode`] prefix operand modifier, makes all [`VaryingOperand`]s of an instruction [`u32`] sized.
    ///
    /// Operands: opcode (operands if any).
    ///
    /// Stack: The stack changes based on the opcode that is being prefixed.
    U32Operands,

    /// Reserved [`Opcode`].
    Reserved1 => Reserved,
    /// Reserved [`Opcode`].
    Reserved2 => Reserved,
    /// Reserved [`Opcode`].
    Reserved3 => Reserved,
    /// Reserved [`Opcode`].
    Reserved4 => Reserved,
    /// Reserved [`Opcode`].
    Reserved5 => Reserved,
    /// Reserved [`Opcode`].
    Reserved6 => Reserved,
    /// Reserved [`Opcode`].
    Reserved7 => Reserved,
    /// Reserved [`Opcode`].
    Reserved8 => Reserved,
    /// Reserved [`Opcode`].
    Reserved9 => Reserved,
    /// Reserved [`Opcode`].
    Reserved10 => Reserved,
    /// Reserved [`Opcode`].
    Reserved11 => Reserved,
    /// Reserved [`Opcode`].
    Reserved12 => Reserved,
    /// Reserved [`Opcode`].
    Reserved13 => Reserved,
    /// Reserved [`Opcode`].
    Reserved14 => Reserved,
    /// Reserved [`Opcode`].
    Reserved15 => Reserved,
    /// Reserved [`Opcode`].
    Reserved16 => Reserved,
    /// Reserved [`Opcode`].
    Reserved17 => Reserved,
    /// Reserved [`Opcode`].
    Reserved18 => Reserved,
    /// Reserved [`Opcode`].
    Reserved19 => Reserved,
    /// Reserved [`Opcode`].
    Reserved20 => Reserved,
    /// Reserved [`Opcode`].
    Reserved21 => Reserved,
    /// Reserved [`Opcode`].
    Reserved22 => Reserved,
    /// Reserved [`Opcode`].
    Reserved23 => Reserved,
    /// Reserved [`Opcode`].
    Reserved24 => Reserved,
    /// Reserved [`Opcode`].
    Reserved25 => Reserved,
    /// Reserved [`Opcode`].
    Reserved26 => Reserved,
    /// Reserved [`Opcode`].
    Reserved27 => Reserved,
    /// Reserved [`Opcode`].
    Reserved28 => Reserved,
    /// Reserved [`Opcode`].
    Reserved29 => Reserved,
    /// Reserved [`Opcode`].
    Reserved30 => Reserved,
    /// Reserved [`Opcode`].
    Reserved31 => Reserved,
    /// Reserved [`Opcode`].
    Reserved32 => Reserved,
    /// Reserved [`Opcode`].
    Reserved33 => Reserved,
    /// Reserved [`Opcode`].
    Reserved34 => Reserved,
    /// Reserved [`Opcode`].
    Reserved35 => Reserved,
    /// Reserved [`Opcode`].
    Reserved36 => Reserved,
    /// Reserved [`Opcode`].
    Reserved37 => Reserved,
    /// Reserved [`Opcode`].
    Reserved38 => Reserved,
    /// Reserved [`Opcode`].
    Reserved39 => Reserved,
    /// Reserved [`Opcode`].
    Reserved40 => Reserved,
    /// Reserved [`Opcode`].
    Reserved41 => Reserved,
    /// Reserved [`Opcode`].
    Reserved42 => Reserved,
    /// Reserved [`Opcode`].
    Reserved43 => Reserved,
    /// Reserved [`Opcode`].
    Reserved44 => Reserved,
    /// Reserved [`Opcode`].
    Reserved45 => Reserved,
    /// Reserved [`Opcode`].
    Reserved46 => Reserved,
    /// Reserved [`Opcode`].
    Reserved47 => Reserved,
    /// Reserved [`Opcode`].
    Reserved48 => Reserved,
    /// Reserved [`Opcode`].
    Reserved49 => Reserved,
    /// Reserved [`Opcode`].
    Reserved50 => Reserved,
    /// Reserved [`Opcode`].
    Reserved51 => Reserved,
    /// Reserved [`Opcode`].
    Reserved52 => Reserved,
    /// Reserved [`Opcode`].
    Reserved53 => Reserved,
    /// Reserved [`Opcode`].
    Reserved54 => Reserved,
    /// Reserved [`Opcode`].
    Reserved55 => Reserved,
    /// Reserved [`Opcode`].
    Reserved56 => Reserved,
    /// Reserved [`Opcode`].
    Reserved57 => Reserved,
    /// Reserved [`Opcode`].
    Reserved58 => Reserved,
    /// Reserved [`Opcode`].
    Reserved59 => Reserved,
    /// Reserved [`Opcode`].
    Reserved60 => Reserved,
    /// Reserved [`Opcode`].
    Reserved61 => Reserved,
}

/// Specific opcodes for bindings.
///
/// This separate enum exists to make matching exhaustive where needed.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BindingOpcode {
    Var,
    InitVar,
    InitLexical,
    SetName,
}

/// Iterator over the instructions in the compact bytecode.
#[derive(Debug, Clone)]
pub(crate) struct InstructionIterator<'bytecode> {
    bytes: &'bytecode [u8],
    pc: usize,
}

// TODO: see if this can be exposed on all features.
#[allow(unused)]
impl<'bytecode> InstructionIterator<'bytecode> {
    /// Create a new [`InstructionIterator`] from bytecode array.
    #[inline]
    #[must_use]
    pub(crate) const fn new(bytes: &'bytecode [u8]) -> Self {
        Self { bytes, pc: 0 }
    }

    /// Create a new [`InstructionIterator`] from bytecode array at pc.
    #[inline]
    #[must_use]
    pub(crate) const fn with_pc(bytes: &'bytecode [u8], pc: usize) -> Self {
        Self { bytes, pc }
    }

    /// Return the current program counter.
    #[must_use]
    pub(crate) const fn pc(&self) -> usize {
        self.pc
    }
}

impl Iterator for InstructionIterator<'_> {
    type Item = (usize, VaryingOperandKind, Instruction);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let start_pc = self.pc;
        if self.pc >= self.bytes.len() {
            return None;
        }

        let instruction =
            Instruction::from_bytecode(self.bytes, &mut self.pc, VaryingOperandKind::U8);

        if instruction == Instruction::U16Operands {
            return Some((
                start_pc,
                VaryingOperandKind::U16,
                Instruction::from_bytecode(self.bytes, &mut self.pc, VaryingOperandKind::U16),
            ));
        } else if instruction == Instruction::U32Operands {
            return Some((
                start_pc,
                VaryingOperandKind::U32,
                Instruction::from_bytecode(self.bytes, &mut self.pc, VaryingOperandKind::U32),
            ));
        }

        Some((start_pc, VaryingOperandKind::U8, instruction))
    }
}

impl FusedIterator for InstructionIterator<'_> {}
