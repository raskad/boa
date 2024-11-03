use crate::{
    bytecompiler::{
        Access, ByteCompiler, InstructionOperand, Literal, Operand2, Register, ToJsString,
    },
    vm::{BindingOpcode, Opcode},
};
use boa_ast::{
    pattern::{ArrayPatternElement, ObjectPatternElement, Pattern},
    property::PropertyName,
};

impl ByteCompiler<'_> {
    pub(crate) fn compile_declaration_pattern_impl(
        &mut self,
        pattern: &Pattern,
        def: BindingOpcode,
        object: &Register,
    ) {
        match pattern {
            Pattern::Object(pattern) => {
                self.emit2(
                    Opcode::ValueNotNullOrUndefined,
                    &[Operand2::Register(&object)],
                );

                let mut excluded_keys_registers = Vec::new();
                let rest_exits = pattern.has_rest();

                for binding in pattern.bindings() {
                    use ObjectPatternElement::{
                        AssignmentPropertyAccess, AssignmentRestPropertyAccess, Pattern,
                        RestProperty, SingleName,
                    };

                    match binding {
                        //  SingleNameBinding : BindingIdentifier Initializer[opt]
                        SingleName {
                            ident,
                            name,
                            default_init,
                        } => {
                            let dst = self.register_allocator.alloc();

                            match name {
                                PropertyName::Literal(ident) => {
                                    self.emit_get_property_by_name(&dst, &object, &object, *ident);
                                    let key = self.register_allocator.alloc();
                                    self.emit_push_literal(
                                        Literal::String(
                                            self.interner()
                                                .resolve_expect(*ident)
                                                .into_common(false),
                                        ),
                                        &key,
                                    );
                                    excluded_keys_registers.push(key);
                                }
                                PropertyName::Computed(node) => {
                                    let key = self.register_allocator.alloc();
                                    self.compile_expr(node, &key);
                                    if rest_exits {
                                        self.emit2(
                                            Opcode::GetPropertyByValuePush,
                                            &[
                                                Operand2::Register(&dst),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &key,
                                                )),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &object,
                                                )),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &object,
                                                )),
                                            ],
                                        );
                                        excluded_keys_registers.push(key);
                                    } else {
                                        self.emit2(
                                            Opcode::GetPropertyByValue,
                                            &[
                                                Operand2::Register(&dst),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &key,
                                                )),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &object,
                                                )),
                                                Operand2::Operand(InstructionOperand::Register(
                                                    &object,
                                                )),
                                            ],
                                        );
                                        self.register_allocator.dealloc(key);
                                    }
                                }
                            }

                            if let Some(init) = default_init {
                                let skip = self.emit_jump_if_not_undefined(&dst);
                                self.compile_expr(init, &dst);
                                self.patch_jump(skip);
                            }

                            self.push_from_register(&dst);
                            self.register_allocator.dealloc(dst);

                            self.emit_binding(def, ident.to_js_string(self.interner()));
                        }
                        //  BindingRestProperty : ... BindingIdentifier
                        RestProperty { ident } => {
                            let value = self.register_allocator.alloc();
                            self.emit2(Opcode::PushEmptyObject, &[Operand2::Register(&value)]);
                            let mut args = Vec::from([
                                Operand2::Register(&value),
                                Operand2::Register(&object),
                                Operand2::Varying(excluded_keys_registers.len() as u32),
                            ]);
                            for r in &excluded_keys_registers {
                                args.push(Operand2::Register(&r));
                            }
                            self.emit2(Opcode::CopyDataProperties, &args);
                            while let Some(r) = excluded_keys_registers.pop() {
                                self.register_allocator.dealloc(r);
                            }
                            self.push_from_register(&value);
                            self.register_allocator.dealloc(value);
                            self.emit_binding(def, ident.to_js_string(self.interner()));
                        }
                        AssignmentRestPropertyAccess { access } => {
                            let value = self.register_allocator.alloc();
                            self.emit2(Opcode::PushEmptyObject, &[Operand2::Register(&value)]);
                            let mut args = Vec::from([
                                Operand2::Register(&value),
                                Operand2::Register(&object),
                                Operand2::Varying(excluded_keys_registers.len() as u32),
                            ]);
                            for r in &excluded_keys_registers {
                                args.push(Operand2::Register(&r));
                            }
                            self.emit2(Opcode::CopyDataProperties, &args);
                            while let Some(r) = excluded_keys_registers.pop() {
                                self.register_allocator.dealloc(r);
                            }
                            self.access_set(Access::Property { access }, |_| return &value);
                            self.register_allocator.dealloc(value);
                        }
                        AssignmentPropertyAccess {
                            name,
                            access,
                            default_init,
                        } => {
                            let key = self.register_allocator.alloc();
                            match &name {
                                PropertyName::Literal(ident) => {
                                    let key = self.register_allocator.alloc();
                                    self.emit_push_literal(
                                        Literal::String(
                                            self.interner()
                                                .resolve_expect(*ident)
                                                .into_common(false),
                                        ),
                                        &key,
                                    );
                                    excluded_keys_registers.push(key);
                                }
                                PropertyName::Computed(node) => {
                                    self.compile_expr(node, &key);
                                    self.emit2(
                                        Opcode::ToPropertyKey,
                                        &[Operand2::Register(&key), Operand2::Register(&key)],
                                    );
                                }
                            }

                            let dst = self.register_allocator.alloc();
                            self.access_set(
                                Access::Property { access },
                                |compiler: &mut ByteCompiler<'_>| {
                                    match name {
                                        PropertyName::Literal(ident) => {
                                            compiler.emit_get_property_by_name(
                                                &dst, &object, &object, *ident,
                                            );
                                            compiler.register_allocator.dealloc(key);
                                        }
                                        PropertyName::Computed(_) => {
                                            if rest_exits {
                                                compiler.emit2(
                                                    Opcode::GetPropertyByValuePush,
                                                    &[
                                                        Operand2::Register(&dst),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&key),
                                                        ),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&object),
                                                        ),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&object),
                                                        ),
                                                    ],
                                                );
                                                excluded_keys_registers.push(key);
                                            } else {
                                                compiler.emit2(
                                                    Opcode::GetPropertyByValue,
                                                    &[
                                                        Operand2::Register(&dst),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&key),
                                                        ),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&object),
                                                        ),
                                                        Operand2::Operand(
                                                            InstructionOperand::Register(&object),
                                                        ),
                                                    ],
                                                );
                                                compiler.register_allocator.dealloc(key);
                                            }
                                        }
                                    }

                                    if let Some(init) = default_init {
                                        let skip = compiler.emit_jump_if_not_undefined(&dst);
                                        compiler.compile_expr(init, &dst);
                                        compiler.patch_jump(skip);
                                    }

                                    return &dst;
                                },
                            );
                            self.register_allocator.dealloc(dst);
                        }
                        Pattern {
                            name,
                            pattern,
                            default_init,
                        } => {
                            let dst = self.register_allocator.alloc();

                            match name {
                                PropertyName::Literal(ident) => {
                                    self.emit_get_property_by_name(&dst, &object, &object, *ident);
                                }
                                PropertyName::Computed(node) => {
                                    let key = self.register_allocator.alloc();
                                    self.compile_expr(node, &key);
                                    self.emit2(
                                        Opcode::GetPropertyByValue,
                                        &[
                                            Operand2::Register(&dst),
                                            Operand2::Operand(InstructionOperand::Register(&key)),
                                            Operand2::Operand(InstructionOperand::Register(
                                                &object,
                                            )),
                                            Operand2::Operand(InstructionOperand::Register(
                                                &object,
                                            )),
                                        ],
                                    );
                                    self.register_allocator.dealloc(key);
                                }
                            }

                            if let Some(init) = default_init {
                                let skip = self.emit_jump_if_not_undefined(&dst);
                                self.compile_expr(init, &dst);
                                self.patch_jump(skip);
                            }
                            self.compile_declaration_pattern(pattern, def, &dst);
                            self.register_allocator.dealloc(dst);
                        }
                    }
                }

                while let Some(r) = excluded_keys_registers.pop() {
                    self.register_allocator.dealloc(r);
                }
            }
            Pattern::Array(pattern) => {
                self.emit2(
                    Opcode::ValueNotNullOrUndefined,
                    &[Operand2::Register(&object)],
                );
                self.emit2(Opcode::GetIterator, &[Operand2::Register(&object)]);

                let handler_index = self.push_handler();
                for element in pattern.bindings() {
                    self.compile_array_pattern_element(element, def);
                }

                let no_exception_thrown = self.jump();
                self.patch_handler(handler_index);
                self.emit_opcode(Opcode::MaybeException);

                // stack: hasPending, exception?

                self.current_stack_value_count += 2;
                let iterator_close_handler = self.push_handler();
                self.iterator_close(false);
                self.patch_handler(iterator_close_handler);
                self.current_stack_value_count -= 2;

                let value = self.register_allocator.alloc();
                self.pop_into_register(&value);
                let jump = self.jump_if_false(&value);
                self.register_allocator.dealloc(value);

                self.emit_opcode(Opcode::Throw);
                self.patch_jump(jump);
                self.emit_opcode(Opcode::ReThrow);

                self.patch_jump(no_exception_thrown);

                self.iterator_close(false);
            }
        }
    }

    fn compile_array_pattern_element(&mut self, element: &ArrayPatternElement, def: BindingOpcode) {
        use ArrayPatternElement::{
            Elision, Pattern, PatternRest, PropertyAccess, PropertyAccessRest, SingleName,
            SingleNameRest,
        };

        match element {
            // ArrayBindingPattern : [ Elision ]
            Elision => {
                self.emit_opcode(Opcode::IteratorNext);
            }
            // SingleNameBinding : BindingIdentifier Initializer[opt]
            SingleName {
                ident,
                default_init,
            } => {
                self.emit_opcode(Opcode::IteratorNext);
                let value = self.register_allocator.alloc();
                self.emit2(Opcode::IteratorDone, &[Operand2::Register(&value)]);
                let done = self.jump_if_true(&value);
                self.emit2(Opcode::IteratorValue, &[Operand2::Register(&value)]);
                let skip_push = self.jump();
                self.patch_jump(done);
                self.push_undefined(&value);
                self.patch_jump(skip_push);

                if let Some(init) = default_init {
                    let skip = self.emit_jump_if_not_undefined(&value);
                    self.compile_expr(init, &value);
                    self.patch_jump(skip);
                }

                self.push_from_register(&value);
                self.register_allocator.dealloc(value);

                self.emit_binding(def, ident.to_js_string(self.interner()));
            }
            PropertyAccess {
                access,
                default_init,
            } => {
                let value = self.register_allocator.alloc();
                self.access_set(Access::Property { access }, |compiler| {
                    compiler.emit_opcode(Opcode::IteratorNext);
                    compiler.emit2(Opcode::IteratorDone, &[Operand2::Register(&value)]);
                    let done = compiler.jump_if_true(&value);
                    compiler.emit2(Opcode::IteratorValue, &[Operand2::Register(&value)]);
                    let skip_push = compiler.jump();
                    compiler.patch_jump(done);
                    compiler.push_undefined(&value);
                    compiler.patch_jump(skip_push);

                    if let Some(init) = default_init {
                        let skip = compiler.emit_jump_if_not_undefined(&value);
                        compiler.compile_expr(init, &value);
                        compiler.patch_jump(skip);
                    }

                    return &value;
                });
                self.register_allocator.dealloc(value);
            }
            // BindingElement : BindingPattern Initializer[opt]
            Pattern {
                pattern,
                default_init,
            } => {
                self.emit_opcode(Opcode::IteratorNext);
                let value = self.register_allocator.alloc();
                self.emit2(Opcode::IteratorDone, &[Operand2::Register(&value)]);
                let done = self.jump_if_true(&value);
                self.emit2(Opcode::IteratorValue, &[Operand2::Register(&value)]);
                let skip_push = self.jump();
                self.patch_jump(done);
                self.push_undefined(&value);
                self.patch_jump(skip_push);

                if let Some(init) = default_init {
                    let skip = self.emit_jump_if_not_undefined(&value);
                    self.compile_expr(init, &value);
                    self.patch_jump(skip);
                }
                self.compile_declaration_pattern(pattern, def, &value);
                self.register_allocator.dealloc(value);
            }
            // BindingRestElement : ... BindingIdentifier
            SingleNameRest { ident } => {
                let value = self.register_allocator.alloc();
                self.emit2(Opcode::IteratorToArray, &[Operand2::Register(&value)]);
                self.push_from_register(&value);
                self.register_allocator.dealloc(value);
                self.emit_binding(def, ident.to_js_string(self.interner()));
            }
            PropertyAccessRest { access } => {
                let value = self.register_allocator.alloc();
                self.access_set(Access::Property { access }, |compiler| {
                    compiler.emit2(Opcode::IteratorToArray, &[Operand2::Register(&value)]);
                    return &value;
                });
                self.register_allocator.dealloc(value);
            }
            // BindingRestElement : ... BindingPattern
            PatternRest { pattern } => {
                let value = self.register_allocator.alloc();
                self.emit2(Opcode::IteratorToArray, &[Operand2::Register(&value)]);
                self.compile_declaration_pattern(pattern, def, &value);
                self.register_allocator.dealloc(value);
            }
        }
    }
}
