use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

macro_rules! implement_push_numbers_with_conversion {
    ($name:ident, $num_type:ty, $doc_string:literal) => {
        #[doc= concat!("`", stringify!($name), "` implements the OpCode Operation for `Opcode::", stringify!($name), "`\n")]
        #[doc= "\n"]
        #[doc="Operation:\n"]
        #[doc= concat!(" - ", $doc_string)]
        #[derive(Debug, Clone, Copy)]
        pub(crate) struct $name;

        impl $name {
            fn operation(dst: u32, value: $num_type, context: &mut Context) -> JsResult<CompletionType> {
                let rp = context.vm.frame().rp;
                context.vm.stack[(rp + dst) as usize] = i32::from(value).into();
                Ok(CompletionType::Normal)
            }
        }

        impl Operation for $name {
            const NAME: &'static str = stringify!($name);
            const INSTRUCTION: &'static str = stringify!("INST - " + $name);
            const COST: u8 = 1;

            fn execute(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u8>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }

            fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u16>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }

            fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u32>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }
        }
    };
}

macro_rules! implement_push_numbers_no_conversion {
    ($name:ident, $num_type:ty, $doc_string:literal) => {
        #[doc= concat!("`", stringify!($name), "` implements the OpCode Operation for `Opcode::", stringify!($name), "`\n")]
        #[doc= "\n"]
        #[doc="Operation:\n"]
        #[doc= concat!(" - ", $doc_string)]
        #[derive(Debug, Clone, Copy)]
        pub(crate) struct $name;

        impl $name {
            fn operation(dst: u32, value: $num_type, context: &mut Context) -> JsResult<CompletionType> {
                let rp = context.vm.frame().rp;
                context.vm.stack[(rp + dst) as usize] = value.into();
                Ok(CompletionType::Normal)
            }
        }

        impl Operation for $name {
            const NAME: &'static str = stringify!($name);
            const INSTRUCTION: &'static str = stringify!("INST - " + $name);
            const COST: u8 = 1;

            fn execute(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u8>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }

            fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u16>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }

            fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
                let dst = context.vm.read::<u32>().into();
                let value = context.vm.read::<$num_type>();
                Self::operation(dst, value, context)
            }
        }
    };
}

implement_push_numbers_with_conversion!(PushInt8, i8, "Push `i8` value on the stack");
implement_push_numbers_with_conversion!(PushInt16, i16, "Push `i16` value on the stack");

implement_push_numbers_no_conversion!(PushInt32, i32, "Push `i32` value on the stack");
implement_push_numbers_no_conversion!(PushFloat, f32, "Push `f64` value on the stack");
implement_push_numbers_no_conversion!(PushDouble, f64, "Push `f64` value on the stack");
