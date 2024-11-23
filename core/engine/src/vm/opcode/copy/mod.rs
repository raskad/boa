use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `CopyDataProperties` implements the Opcode Operation for `Opcode::CopyDataProperties`
///
/// Operation:
///  - Copy all properties of one object to another object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CopyDataProperties;

impl CopyDataProperties {
    fn operation(
        object: u32,
        source: u32,
        keys: &[u32],
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object) as usize].clone();
        let source = context.vm.stack[(rp + source) as usize].clone();
        let mut excluded_keys = Vec::with_capacity(keys.len());
        for key in keys {
            let key = context.vm.stack[(rp + *key) as usize].clone();
            excluded_keys.push(
                key.to_property_key(context)
                    .expect("key must be property key"),
            );
        }
        let object = object.as_object().expect("not an object");
        object.copy_data_properties(&source, excluded_keys, context)?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for CopyDataProperties {
    const NAME: &'static str = "CopyDataProperties";
    const INSTRUCTION: &'static str = "INST - CopyDataProperties";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let source = context.vm.read::<u8>().into();
        let key_count = context.vm.read::<u8>() as usize;
        let mut keys = Vec::with_capacity(key_count);
        for _ in 0..key_count {
            keys.push(context.vm.read::<u8>().into());
        }
        Self::operation(object, source, &keys, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let source = context.vm.read::<u16>().into();
        let key_count = context.vm.read::<u16>() as usize;
        let mut keys = Vec::with_capacity(key_count);
        for _ in 0..key_count {
            keys.push(context.vm.read::<u16>().into());
        }
        Self::operation(object, source, &keys, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>();
        let source = context.vm.read::<u32>();
        let key_count = context.vm.read::<u32>() as usize;
        let mut keys = Vec::with_capacity(key_count);
        for _ in 0..key_count {
            keys.push(context.vm.read::<u32>());
        }
        Self::operation(object, source, &keys, context)
    }
}
