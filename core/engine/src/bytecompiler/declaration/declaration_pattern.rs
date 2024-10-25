use crate::{
    bytecompiler::{
        Access, ByteCompiler, InstructionOperand, Literal, Operand, Operand2, ToJsString,
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
    ) {
        match pattern {
            Pattern::Object(pattern) => {
                self.emit_opcode(Opcode::ValueNotNullOrUndefined);
                self.emit_opcode(Opcode::RequireObjectCoercible);
                let object = self.register_allocator.alloc();
                self.pop_into_register(&object);

                let mut excluded_keys_registers= Vec::new();
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
                                    self.emit_push_literal(Literal::String(
                                        self.interner().resolve_expect(*ident).into_common(false),
                                    ));
                                    let key = self.register_allocator.alloc();
                                    self.pop_into_register(&key);
                                    excluded_keys_registers.push(key);
                                }
                                PropertyName::Computed(node) => {
                                    let key = self.register_allocator.alloc();
                                    self.compile_expr2(node, &key);
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

                            self.push_from_register(&dst);
                            self.register_allocator.dealloc(dst);

                            if let Some(init) = default_init {
                                let skip =
                                    self.emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                                self.compile_expr(init, true);
                                self.patch_jump(skip);
                            }
                            self.emit_binding(def, ident.to_js_string(self.interner()));
                        }
                        //  BindingRestProperty : ... BindingIdentifier
                        RestProperty { ident } => {
                            let new_object = self.register_allocator.alloc();
                            self.emit2(Opcode::PushEmptyObject, &[Operand2::Register(&new_object)]);
                            self.push_from_register(&object);
                            self.push_from_register(&new_object);
                            self.register_allocator.dealloc(new_object);

                            let len = excluded_keys_registers.len() as u32;
                            while let Some(r) = excluded_keys_registers.pop() {
                                self.push_from_register(&r);
                                self.register_allocator.dealloc(r);
                            }

                            self.emit(
                                Opcode::CopyDataProperties,
                                &[
                                    Operand::Varying(len),
                                    Operand::Varying(0),
                                ],
                            );
                            self.emit_binding(def, ident.to_js_string(self.interner()));
                        }
                        AssignmentRestPropertyAccess { access } => {
                            let new_object = self.register_allocator.alloc();
                            self.emit2(Opcode::PushEmptyObject, &[Operand2::Register(&new_object)]);
                            self.push_from_register(&object);
                            self.push_from_register(&new_object);
                            self.register_allocator.dealloc(new_object);

                            let len = excluded_keys_registers.len() as u32;
                            while let Some(r) = excluded_keys_registers.pop() {
                                self.push_from_register(&r);
                                self.register_allocator.dealloc(r);
                            }

                            self.emit(
                                Opcode::CopyDataProperties,
                                &[
                                    Operand::Varying(len),
                                    Operand::Varying(0),
                                ],
                            );
                            let value = self.register_allocator.alloc();
                            self.pop_into_register(&value);
                            self.access_set(Access::Property { access }, false, |_| {
                                return &value;
                            });
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
                                    self.emit_push_literal(Literal::String(
                                        self.interner().resolve_expect(*ident).into_common(false),
                                    ));
                                    let key = self.register_allocator.alloc();
                                    self.pop_into_register(&key);
                                    excluded_keys_registers.push(key);
                                }
                                PropertyName::Computed(node) => {
                                    self.compile_expr2(node, &key);
                                    self.emit2(
                                        Opcode::ToPropertyKey,
                                        &[Operand2::Register(&key), Operand2::Register(&key)],
                                    );
                                }
                            }

                            let dst = self.register_allocator.alloc();
                            self.access_set(
                                Access::Property { access },
                                false,
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
                                        compiler.push_from_register(&dst);
                                        let skip = compiler
                                            .emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                                        compiler.compile_expr2(init, &dst);
                                        let skip2 = compiler.jump();
                                        compiler.patch_jump(skip);
                                        compiler.pop_into_register(&dst);
                                        compiler.patch_jump(skip2);
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
                                    self.compile_expr2(node, &key);
                                    self.emit2(
                                        Opcode::GetPropertyByValue,
                                        &[
                                            Operand2::Register(&dst),
                                            Operand2::Operand(InstructionOperand::Register(&key)),
                                            Operand2::Operand(InstructionOperand::Register(&object)),
                                            Operand2::Operand(InstructionOperand::Register(&object)),
                                        ],
                                    );
                                    self.register_allocator.dealloc(key);
                                }
                            }

                            self.push_from_register(&dst);
                            self.register_allocator.dealloc(dst);

                            if let Some(init) = default_init {
                                let skip =
                                    self.emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                                self.compile_expr(init, true);
                                self.patch_jump(skip);
                            }

                            self.compile_declaration_pattern(pattern, def);
                        }
                    }
                }

                self.register_allocator.dealloc(object);
                while let Some(r) = excluded_keys_registers.pop() {
                    self.register_allocator.dealloc(r);
                }
            }
            Pattern::Array(pattern) => {
                self.emit_opcode(Opcode::ValueNotNullOrUndefined);
                self.emit_opcode(Opcode::GetIterator);

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

                let jump = self.jump_if_false();
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
                self.emit_opcode(Opcode::IteratorNextWithoutPop);
            }
            // SingleNameBinding : BindingIdentifier Initializer[opt]
            SingleName {
                ident,
                default_init,
            } => {
                self.emit_opcode(Opcode::IteratorNextWithoutPop);
                self.emit_opcode(Opcode::IteratorValueWithoutPop);
                if let Some(init) = default_init {
                    let skip = self.emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                    self.compile_expr(init, true);
                    self.patch_jump(skip);
                }
                self.emit_binding(def, ident.to_js_string(self.interner()));
            }
            PropertyAccess {
                access,
                default_init,
            } => {
                let value = self.register_allocator.alloc();
                self.access_set(Access::Property { access }, false, |compiler| {
                    compiler.emit_opcode(Opcode::IteratorNextWithoutPop);
                    compiler.emit_opcode(Opcode::IteratorValueWithoutPop);

                    if let Some(init) = default_init {
                        let skip = compiler.emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                        compiler.compile_expr2(init, &value);
                        let skip2 = compiler.jump();
                        compiler.patch_jump(skip);
                        compiler.pop_into_register(&value);
                        compiler.patch_jump(skip2);
                    } else {
                        compiler.pop_into_register(&value);
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
                self.emit_opcode(Opcode::IteratorNextWithoutPop);
                self.emit_opcode(Opcode::IteratorValueWithoutPop);

                if let Some(init) = default_init {
                    let skip = self.emit_opcode_with_operand(Opcode::JumpIfNotUndefined);
                    self.compile_expr(init, true);
                    self.patch_jump(skip);
                }

                self.compile_declaration_pattern(pattern, def);
            }
            // BindingRestElement : ... BindingIdentifier
            SingleNameRest { ident } => {
                self.emit_opcode(Opcode::IteratorToArray);
                self.emit_binding(def, ident.to_js_string(self.interner()));
            }
            PropertyAccessRest { access } => {
                let value = self.register_allocator.alloc();
                self.access_set(Access::Property { access }, false, |compiler| {
                    compiler.emit_opcode(Opcode::IteratorToArray);
                    compiler.pop_into_register(&value);
                    return &value;
                });
                self.register_allocator.dealloc(value);
            }
            // BindingRestElement : ... BindingPattern
            PatternRest { pattern } => {
                self.emit_opcode(Opcode::IteratorToArray);
                self.compile_declaration_pattern(pattern, def);
            }
        }
    }
}
