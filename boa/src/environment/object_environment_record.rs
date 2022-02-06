//! # Object Records
//!
//! Each object Environment Record is associated with an object called its binding object.
//! An object Environment Record binds the set of string identifier names that directly
//! correspond to the property names of its binding object.
//! Property keys that are not strings in the form of an `IdentifierName` are not included in the set of bound identifiers.
//! More info:  [Object Records](https://tc39.es/ecma262/#sec-object-environment-records)

use crate::{
    gc::{Finalize, Trace},
    object::JsObject,
    symbol::WellKnownSymbols,
    Context, JsResult, JsValue,
};

#[derive(Debug, Trace, Finalize, Clone)]
pub struct ObjectEnvironmentRecord {
    pub bindings: JsObject,
    pub with_environment: bool,
}

impl ObjectEnvironmentRecord {
    /// `9.1.1.2.1 HasBinding ( N )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-object-environment-records-hasbinding-n
    #[inline]
    pub(crate) fn has_binding(&self, name: &str, context: &mut Context) -> JsResult<bool> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        // 2. Let foundBinding be ? HasProperty(bindingObject, N).
        // 3. If foundBinding is false, return false.
        if !self.bindings.has_property(name, context)? {
            return Ok(false);
        }

        // 4. If envRec.[[IsWithEnvironment]] is false, return true.
        if !self.with_environment {
            return Ok(true);
        }

        // 5. Let unscopables be ? Get(bindingObject, @@unscopables).
        // 6. If Type(unscopables) is Object, then
        if let Some(unscopables) = self
            .bindings
            .get(WellKnownSymbols::unscopables(), context)?
            .as_object()
        {
            // a. Let blocked be ! ToBoolean(? Get(unscopables, N)).
            // b. If blocked is true, return false.
            if unscopables.get(name, context)?.to_boolean() {
                return Ok(false);
            }
        }

        // 7. Return true.
        Ok(true)
    }

    /// `9.1.1.2.5 SetMutableBinding ( N, V, S )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-object-environment-records-setmutablebinding-n-v-s
    #[inline]
    pub(crate) fn set_mutable_binding(
        &self,
        name: &str,
        value: JsValue,
        strict: bool,
        context: &mut Context,
    ) -> JsResult<()> {
        // 1. Let bindingObject be envRec.[[BindingObject]].
        // 2. Let stillExists be ? HasProperty(bindingObject, N).
        let still_exists = self.bindings.has_property(name, context)?;

        // 3. If stillExists is false and S is true, throw a ReferenceError exception.
        if !still_exists && strict {
            return context.throw_reference_error("Binding already exists");
        }

        // 4. Return ? Set(bindingObject, N, V, S).
        self.bindings.set(name, value, strict, context)?;
        Ok(())
    }
}
