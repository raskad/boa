use crate::{
    builtins::OrdinaryObject,
    vm::{
        opcode::{Operation, VaryingOperand},
        Registers,
    },
    Context,
};

/// `PushEmptyObject` implements the Opcode Operation for `Opcode::PushEmptyObject`
///
/// Operation:
///  - Push empty object `{}` value on the stack.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PushEmptyObject;

impl PushEmptyObject {
    #[inline(always)]
    pub(crate) fn operation(dst: VaryingOperand, registers: &mut Registers, context: &mut Context) {
        let o = context
            .intrinsics()
            .templates()
            .ordinary_object()
            .create(OrdinaryObject, Vec::default());
        
        dbg!("Pushed empty object");
        dbg!("Pushed empty object: ", o.as_erased());
        dbg!(o.is_marked());
        registers.set(dst.into(), o.into());
        context.force_gc();
        let value = registers.get(dst.into());
        if let Some(o) = value.as_object() {
            dbg!("Pushed empty object AFTER GC", o.as_erased());
            dbg!(o.is_marked());
        }
    }
}

impl Operation for PushEmptyObject {
    const NAME: &'static str = "PushEmptyObject";
    const INSTRUCTION: &'static str = "INST - PushEmptyObject";
    const COST: u8 = 1;
}
