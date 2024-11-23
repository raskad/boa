use boa_macros::js_str;

use crate::{
    builtins::function::{set_function_name, OrdinaryFunction},
    object::internal_methods::InternalMethodContext,
    property::PropertyDescriptor,
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `DefineClassStaticGetterByName` implements the Opcode Operation for `Opcode::DefineClassStaticGetterByName`
///
/// Operation:
///  - Defines a class getter by name.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DefineClassStaticGetterByName;

impl DefineClassStaticGetterByName {
    fn operation(
        class: u32,
        function: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let function = context.vm.stack[(rp + function) as usize].clone();
        let class = context.vm.stack[(rp + class) as usize].clone();
        let class = class.as_object().expect("class must be object");
        let key = context
            .vm
            .frame()
            .code_block()
            .constant_string(index)
            .into();
        {
            let function_object = function
                .as_object()
                .expect("method must be function object");
            set_function_name(function_object, &key, Some(js_str!("get")), context);
            function_object
                .downcast_mut::<OrdinaryFunction>()
                .expect("method must be function object")
                .set_home_object(class.clone());
        }
        let set = class
            .__get_own_property__(&key, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        class.__define_own_property__(
            &key,
            PropertyDescriptor::builder()
                .maybe_get(Some(function))
                .maybe_set(set)
                .enumerable(false)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for DefineClassStaticGetterByName {
    const NAME: &'static str = "DefineClassStaticGetterByName";
    const INSTRUCTION: &'static str = "INST - DefineClassStaticGetterByName";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u8>().into();
        let class = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(class, function, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u16>().into();
        let class = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(class, function, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u32>();
        let class = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(class, function, index, context)
    }
}

/// `DefineClassGetterByName` implements the Opcode Operation for `Opcode::DefineClassGetterByName`
///
/// Operation:
///  - Defines a class getter by name.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DefineClassGetterByName;

impl DefineClassGetterByName {
    fn operation(
        class_proto: u32,
        function: u32,
        index: usize,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let function = context.vm.stack[(rp + function) as usize].clone();
        let class_proto = context.vm.stack[(rp + class_proto) as usize].clone();
        let class_proto = class_proto.as_object().expect("class must be object");
        let key = context
            .vm
            .frame()
            .code_block()
            .constant_string(index)
            .into();
        {
            let function_object = function
                .as_object()
                .expect("method must be function object");
            set_function_name(function_object, &key, Some(js_str!("get")), context);
            function_object
                .downcast_mut::<OrdinaryFunction>()
                .expect("method must be function object")
                .set_home_object(class_proto.clone());
        }
        let set = class_proto
            .__get_own_property__(&key, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        class_proto.__define_own_property__(
            &key,
            PropertyDescriptor::builder()
                .maybe_get(Some(function))
                .maybe_set(set)
                .enumerable(false)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for DefineClassGetterByName {
    const NAME: &'static str = "DefineClassGetterByName";
    const INSTRUCTION: &'static str = "INST - DefineClassGetterByName";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u8>().into();
        let class_proto = context.vm.read::<u8>().into();
        let index = context.vm.read::<u8>() as usize;
        Self::operation(class_proto, function, index, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u16>().into();
        let class_proto = context.vm.read::<u16>().into();
        let index = context.vm.read::<u16>() as usize;
        Self::operation(class_proto, function, index, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u32>();
        let class_proto = context.vm.read::<u32>();
        let index = context.vm.read::<u32>() as usize;
        Self::operation(class_proto, function, index, context)
    }
}

/// `DefineClassStaticGetterByValue` implements the Opcode Operation for `Opcode::DefineClassStaticGetterByValue`
///
/// Operation:
///  - Defines a class getter by value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DefineClassStaticGetterByValue;

impl DefineClassStaticGetterByValue {
    fn operation(
        function: u32,
        key: u32,
        class: u32,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let function = context.vm.stack[(rp + function) as usize].clone();
        let key = context.vm.stack[(rp + key) as usize].clone();
        let class = context.vm.stack[(rp + class) as usize].clone();
        let class = class.as_object().expect("class must be object");
        let key = key
            .to_property_key(context)
            .expect("property key must already be valid");
        {
            let function_object = function
                .as_object()
                .expect("method must be function object");
            set_function_name(function_object, &key, Some(js_str!("get")), context);
            function_object
                .downcast_mut::<OrdinaryFunction>()
                .expect("method must be function object")
                .set_home_object(class.clone());
        }

        let set = class
            .__get_own_property__(&key, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        class.define_property_or_throw(
            key,
            PropertyDescriptor::builder()
                .maybe_get(Some(function))
                .maybe_set(set)
                .enumerable(false)
                .configurable(true)
                .build(),
            context,
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for DefineClassStaticGetterByValue {
    const NAME: &'static str = "DefineClassStaticGetterByValue";
    const INSTRUCTION: &'static str = "INST - DefineClassStaticGetterByValue";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        let class = context.vm.read::<u8>().into();
        Self::operation(function, key, class, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        let class = context.vm.read::<u16>().into();
        Self::operation(function, key, class, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        let class = context.vm.read::<u32>();
        Self::operation(function, key, class, context)
    }
}

/// `DefineClassGetterByValue` implements the Opcode Operation for `Opcode::DefineClassGetterByValue`
///
/// Operation:
///  - Defines a class getter by value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DefineClassGetterByValue;

impl DefineClassGetterByValue {
    fn operation(
        function: u32,
        key: u32,
        class_proto: u32,
        context: &mut Context,
    ) -> JsResult<CompletionType> {
        let rp = context.vm.frame().rp;
        let function = context.vm.stack[(rp + function) as usize].clone();
        let key = context.vm.stack[(rp + key) as usize].clone();
        let class_proto = context.vm.stack[(rp + class_proto) as usize].clone();
        let class_proto = class_proto.as_object().expect("class must be object");
        let key = key
            .to_property_key(context)
            .expect("property key must already be valid");
        {
            let function_object = function
                .as_object()
                .expect("method must be function object");
            set_function_name(function_object, &key, Some(js_str!("get")), context);
            function_object
                .downcast_mut::<OrdinaryFunction>()
                .expect("method must be function object")
                .set_home_object(class_proto.clone());
        }
        let set = class_proto
            .__get_own_property__(&key, &mut InternalMethodContext::new(context))?
            .as_ref()
            .and_then(PropertyDescriptor::set)
            .cloned();
        class_proto.__define_own_property__(
            &key,
            PropertyDescriptor::builder()
                .maybe_get(Some(function))
                .maybe_set(set)
                .enumerable(false)
                .configurable(true)
                .build(),
            &mut InternalMethodContext::new(context),
        )?;
        Ok(CompletionType::Normal)
    }
}

impl Operation for DefineClassGetterByValue {
    const NAME: &'static str = "DefineClassGetterByValue";
    const INSTRUCTION: &'static str = "INST - DefineClassGetterByValue";
    const COST: u8 = 6;

    fn execute(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u8>().into();
        let key = context.vm.read::<u8>().into();
        let class_proto = context.vm.read::<u8>().into();
        Self::operation(function, key, class_proto, context)
    }

    fn execute_with_u16_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u16>().into();
        let key = context.vm.read::<u16>().into();
        let class_proto = context.vm.read::<u16>().into();
        Self::operation(function, key, class_proto, context)
    }

    fn execute_with_u32_operands(context: &mut Context) -> JsResult<CompletionType> {
        let function = context.vm.read::<u32>();
        let key = context.vm.read::<u32>();
        let class_proto = context.vm.read::<u32>();
        Self::operation(function, key, class_proto, context)
    }
}
