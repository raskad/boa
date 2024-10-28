use boa_macros::js_str;

use crate::{
    builtins::function::set_function_name,
    object::{internal_methods::InternalMethodContext, shape::slot::SlotAttributes},
    property::{PropertyDescriptor, PropertyKey},
    vm::{opcode::Operation, CompletionType},
    Context, JsNativeError, JsResult, JsValue,
};

/// `SetPropertyByName` implements the Opcode Operation for `Opcode::SetPropertyByName`
///
/// Operation:
///  - Sets a property by name of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertyByName;

impl SetPropertyByName {
    fn operation(
        operand_types: u8,
        value: u32,
        receiver: u32,
        object: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let value = context
            .vm
            .frame()
            .read_value::<0>(operand_types, value, &context.vm);
        let receiver = context
            .vm
            .frame()
            .read_value::<1>(operand_types, receiver, &context.vm);
        let object = context
            .vm
            .frame()
            .read_value::<2>(operand_types, object, &context.vm);
        let object = if let Some(object) = object.as_object() {
            object.clone()
        } else {
            object.to_object(context)?
        };

        let ic = &context.vm.frame().code_block().ic[index];

        let object_borrowed = object.borrow();
        if let Some((shape, slot)) = ic.match_or_reset(object_borrowed.shape()) {
            let slot_index = slot.index as usize;

            if slot.attributes.is_accessor_descriptor() {
                let result = if slot.attributes.contains(SlotAttributes::PROTOTYPE) {
                    let prototype = shape.prototype().expect("prototype should have value");
                    let prototype = prototype.borrow();

                    prototype.properties().storage[slot_index + 1].clone()
                } else {
                    object_borrowed.properties().storage[slot_index + 1].clone()
                };

                drop(object_borrowed);
                if slot.attributes.has_set() && result.is_object() {
                    result.as_object().expect("should contain getter").call(
                        &receiver,
                        &[value.clone()],
                        context,
                    )?;
                }
            } else if slot.attributes.contains(SlotAttributes::PROTOTYPE) {
                let prototype = shape.prototype().expect("prototype should have value");
                let mut prototype = prototype.borrow_mut();

                prototype.properties_mut().storage[slot_index] = value.clone();
            } else {
                drop(object_borrowed);
                let mut object_borrowed = object.borrow_mut();
                object_borrowed.properties_mut().storage[slot_index] = value.clone();
            }
            return Ok(CompletionType::Normal);
        }
        drop(object_borrowed);

        let name: PropertyKey = ic.name.clone().into();

        let context = &mut InternalMethodContext::new(context);
        let succeeded = object.__set__(name.clone(), value.clone(), receiver, context)?;
        if !succeeded && context.vm.frame().code_block.strict() {
            return Err(JsNativeError::typ()
                .with_message(format!("cannot set non-writable property: {name}"))
                .into());
        }

        // Cache the property.
        let slot = *context.slot();
        if succeeded && slot.is_cachable() {
            let ic = &context.vm.frame().code_block.ic[index];
            let object_borrowed = object.borrow();
            let shape = object_borrowed.shape();
            ic.set(shape, slot);
        }

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertyByName {
    const NAME: &'static str = "SetPropertyByName";
    const INSTRUCTION: &'static str = "INST - SetPropertyByName";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u8>().into();
        let receiver = context.vm.read::<u8>().into();
        let object = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(operand_types, value, receiver, object, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u16>().into();
        let receiver = context.vm.read::<u16>().into();
        let object = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(operand_types, value, receiver, object, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u32>();
        let receiver = context.vm.read::<u32>();
        let object = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(operand_types, value, receiver, object, index, context)
    }
}

/// `SetPropertyByValue` implements the Opcode Operation for `Opcode::SetPropertyByValue`
///
/// Operation:
///  - Sets a property by value of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertyByValue;

impl SetPropertyByValue {
    fn operation(
        operand_types: u8,
        value: u32,
        key: u32,
        receiver: u32,
        object: u32,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let value = context
            .vm
            .frame()
            .read_value::<0>(operand_types, value, &context.vm);
        let key = context
            .vm
            .frame()
            .read_value::<1>(operand_types, key, &context.vm);
        let receiver = context
            .vm
            .frame()
            .read_value::<2>(operand_types, receiver, &context.vm);
        let object = context
            .vm
            .frame()
            .read_value::<3>(operand_types, object, &context.vm);

        let object = if let Some(object) = object.as_object() {
            object.clone()
        } else {
            object.to_object(context)?
        };

        let key = key.to_property_key(context)?;

        // Fast Path:
        'fast_path: {
            if object.is_array() {
                if let PropertyKey::Index(index) = &key {
                    let mut object_borrowed = object.borrow_mut();

                    // Cannot modify if not extensible.
                    if !object_borrowed.extensible {
                        break 'fast_path;
                    }

                    if object_borrowed
                        .properties_mut()
                        .set_dense_property(index.get(), &value)
                    {
                        return Ok(CompletionType::Normal);
                    }
                }
            }
        }

        // Slow path:
        let succeeded =
            object.__set__(key.clone(), value.clone(), receiver, &mut context.into())?;
        if !succeeded && context.vm.frame().code_block.strict() {
            return Err(JsNativeError::typ()
                .with_message(format!("cannot set non-writable property: {key}"))
                .into());
        }

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertyByValue {
    const NAME: &'static str = "SetPropertyByValue";
    const INSTRUCTION: &'static str = "INST - SetPropertyByValue";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        let receiver = context.vm.read::<u8>().into();
        let object = context.vm.read::<u8>().into();
        Self::operation(operand_types, value, key, receiver, object, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        let receiver = context.vm.read::<u16>().into();
        let object = context.vm.read::<u16>().into();
        Self::operation(operand_types, value, key, receiver, object, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let operand_types = context.vm.read::<u8>();
        let value = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        let receiver = context.vm.read::<u32>();
        let object = context.vm.read::<u32>();
        Self::operation(operand_types, value, key, receiver, object, context)
    }
}

/// `SetPropertyGetterByName` implements the Opcode Operation for `Opcode::SetPropertyGetterByName`
///
/// Operation:
///  - Sets a getter property by name of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertyGetterByName;

impl SetPropertyGetterByName {
    fn operation(
        object: u32,
        value: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object) as usize].clone();
        let value = context.vm.stack[(rp + value) as usize].clone();
        let name = context.vm.frame().code_block().constant_string(index).into();

        let object = object.to_object(context)?;
        let set = object
            .__get_own_property__(&name, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        object.__define_own_property__(
            &name,
            PropertyDescriptor::builder()
                .maybe_get(Some(value))
                .maybe_set(set)
                .enumerable(true)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertyGetterByName {
    const NAME: &'static str = "SetPropertyGetterByName";
    const INSTRUCTION: &'static str = "INST - SetPropertyGetterByName";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let value = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(object, value, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let value = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(object, value, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>().into();
        let value = context.vm.read::<u32>().into();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(object, value, index, context)
    }
}

/// `SetPropertyGetterByValue` implements the Opcode Operation for `Opcode::SetPropertyGetterByValue`
///
/// Operation:
///  - Sets a getter property by value of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertyGetterByValue;

impl SetPropertyGetterByValue {
    fn operation(value: u32, key: u32, object: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let key = context.vm.stack[(rp + key) as usize].clone();
        let object = context.vm.stack[(rp + object) as usize].clone();
        let object = object.to_object(context)?;
        let name = key.to_property_key(context)?;

        let set = object
            .__get_own_property__(&name, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        object.__define_own_property__(
            &name,
            PropertyDescriptor::builder()
                .maybe_get(Some(value))
                .maybe_set(set)
                .enumerable(true)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertyGetterByValue {
    const NAME: &'static str = "SetPropertyGetterByValue";
    const INSTRUCTION: &'static str = "INST - SetPropertyGetterByValue";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        let object = context.vm.read::<u8>().into();
        Self::operation(value, key, object,context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        let object = context.vm.read::<u16>().into();
        Self::operation(value, key, object,context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        let object = context.vm.read::<u32>();
        Self::operation(value, key, object,context)
    }
}

/// `SetPropertySetterByName` implements the Opcode Operation for `Opcode::SetPropertySetterByName`
///
/// Operation:
///  - Sets a setter property by name of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertySetterByName;

impl SetPropertySetterByName {
    fn operation(
        object: u32,
        value: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let object = context.vm.stack[(rp + object) as usize].clone();
        let value = context.vm.stack[(rp + value) as usize].clone();
        let name = context.vm.frame().code_block().constant_string(index).into();

        let object = object.to_object(context)?;

        let get = object
            .__get_own_property__(&name, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::get)
            .cloned();
        object.__define_own_property__(
            &name,
            PropertyDescriptor::builder()
                .maybe_set(Some(value))
                .maybe_get(get)
                .enumerable(true)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertySetterByName {
    const NAME: &'static str = "SetPropertySetterByName";
    const INSTRUCTION: &'static str = "INST - SetPropertySetterByName";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u8>().into();
        let value = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(object, value, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u16>().into();
        let value = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(object, value, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let object = context.vm.read::<u32>().into();
        let value = context.vm.read::<u32>().into();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(object, value, index, context)
    }
}

/// `SetPropertySetterByValue` implements the Opcode Operation for `Opcode::SetPropertySetterByValue`
///
/// Operation:
///  - Sets a setter property by value of an object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetPropertySetterByValue;

impl SetPropertySetterByValue {
    fn operation(value: u32, key: u32, object: u32, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let value = context.vm.stack[(rp + value) as usize].clone();
        let key = context.vm.stack[(rp + key) as usize].clone();
        let object = context.vm.stack[(rp + object) as usize].clone();
        let object = object.to_object(context)?;
        let name = key.to_property_key(context)?;

        let get = object
            .__get_own_property__(&name, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::get)
            .cloned();
        object.__define_own_property__(
            &name,
            PropertyDescriptor::builder()
                .maybe_set(Some(value))
                .maybe_get(get)
                .enumerable(true)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for SetPropertySetterByValue {
    const NAME: &'static str = "SetPropertySetterByValue";
    const INSTRUCTION: &'static str = "INST - SetPropertySetterByValue";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        let object = context.vm.read::<u8>().into();
        Self::operation(value, key, object,context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        let object = context.vm.read::<u16>().into();
        Self::operation(value, key, object,context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let value = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        let object = context.vm.read::<u32>();
        Self::operation(value, key, object,context)
    }
}

/// `SetFunctionName` implements the Opcode Operation for `Opcode::SetFunctionName`
///
/// Operation:
///  - Sets the name of a function object.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SetFunctionName;

impl SetFunctionName {
    fn operation(function: u32, name: u32, prefix: u8, context: &mut Context) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let function = context.vm.stack[(rp + function) as usize].clone();
        let name = context.vm.stack[(rp + name) as usize].clone();

        let name = match name {
            JsValue::String(name) => name.into(),
            JsValue::Symbol(name) => name.into(),
            _ => unreachable!(),
        };

        let prefix = match prefix {
            1 => Some(js_str!("get")),
            2 => Some(js_str!("set")),
            _ => None,
        };

        set_function_name(
            function.as_object().expect("function is not an object"),
            &name,
            prefix,
            context,
        );

        Ok(CompletionType::Normal)
    }
}

impl Operation for SetFunctionName {
    const NAME: &'static str = "SetFunctionName";
    const INSTRUCTION: &'static str = "INST - SetFunctionName";
    const COST: u8 = 4;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u8>().into();
        let name = context.vm.read::<u8>().into();
        let prefix = context.vm.read::<u8>();
        Self::operation(function, name, prefix, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u16>().into();
        let name = context.vm.read::<u16>().into();
        let prefix = context.vm.read::<u8>();
        Self::operation(function, name, prefix,context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u32>();
        let name = context.vm.read::<u32>();
        let prefix = context.vm.read::<u8>();
        Self::operation(function, name, prefix,context)
    }
}
