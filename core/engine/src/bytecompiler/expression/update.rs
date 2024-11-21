use crate::{
    bytecompiler::{Access, ByteCompiler, InstructionOperand, Operand2, Register, ToJsString},
    vm::Opcode,
};
use boa_ast::{
    expression::{
        access::{PropertyAccess, PropertyAccessField},
        operator::{update::UpdateOp, Update},
    },
    scope::BindingLocatorError,
};

impl ByteCompiler<'_> {
    pub(crate) fn compile_update(&mut self, update: &Update, dst: &Register) {
        let opcode = match update.op() {
            UpdateOp::IncrementPost | UpdateOp::IncrementPre => Opcode::Inc,
            UpdateOp::DecrementPre | UpdateOp::DecrementPost => Opcode::Dec,
        };
        let post = matches!(
            update.op(),
            UpdateOp::IncrementPost | UpdateOp::DecrementPost
        );

        match Access::from_update_target(update.target()) {
            Access::Variable { name } => {
                let name = name.to_js_string(self.interner());
                let binding = self.lexical_scope.get_identifier_reference(name.clone());
                let is_lexical = binding.is_lexical();
                let index = self.get_or_insert_binding(binding);

                let src = self.register_allocator.alloc();
                if is_lexical {
                    self.emit_binding_access(Opcode::GetName, &index, &src);
                } else {
                    self.emit_binding_access(Opcode::GetNameAndLocator, &index, &src);
                }

                self.emit2(
                    Opcode::ToNumeric,
                    &[
                        Operand2::Register(&dst),
                        Operand2::Operand(InstructionOperand::Register(&src)),
                    ],
                );
                self.emit2(
                    opcode,
                    &[
                        Operand2::Register(&src),
                        Operand2::Operand(InstructionOperand::Register(&dst)),
                    ],
                );
                if is_lexical {
                    match self.lexical_scope.set_mutable_binding(name.clone()) {
                        Ok(binding) => {
                            let index = self.get_or_insert_binding(binding);
                            self.emit_binding_access(Opcode::SetName, &index, &src);
                        }
                        Err(BindingLocatorError::MutateImmutable) => {
                            let index = self.get_or_insert_string(name);
                            self.emit_with_varying_operand(Opcode::ThrowMutateImmutable, index);
                        }
                        Err(BindingLocatorError::Silent) => {}
                    }
                } else {
                    self.emit_binding_access(Opcode::SetNameByLocator, &index, &src);
                }
                if !post {
                    self.emit_move(dst, InstructionOperand::Register(&src));
                }

                self.register_allocator.dealloc(src);
            }
            Access::Property { access } => match access {
                PropertyAccess::Simple(access) => {
                    let object = self.register_allocator.alloc();
                    self.compile_expr(access.target(), &object);

                    match access.field() {
                        PropertyAccessField::Const(ident) => {
                            self.emit_get_property_by_name(&dst, &object, &object, *ident);

                            let dst_numeric = self.register_allocator.alloc();

                            self.emit2(
                                Opcode::ToNumeric,
                                &[
                                    Operand2::Register(&dst_numeric),
                                    Operand2::Operand(InstructionOperand::Register(&dst)),
                                ],
                            );
                            self.emit2(
                                opcode,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Operand(InstructionOperand::Register(&dst_numeric)),
                                ],
                            );

                            self.emit_set_property_by_name(&dst, &object, &object, *ident);

                            if post {
                                self.emit_move(dst, InstructionOperand::Register(&dst_numeric));
                            }

                            self.register_allocator.dealloc(object);
                            self.register_allocator.dealloc(dst_numeric);
                        }
                        PropertyAccessField::Expr(expr) => {
                            let key = self.register_allocator.alloc();
                            self.compile_expr(expr, &key);

                            self.emit2(
                                Opcode::GetPropertyByValuePush,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                ],
                            );

                            let dst_numeric = self.register_allocator.alloc();

                            self.emit2(
                                Opcode::ToNumeric,
                                &[
                                    Operand2::Register(&dst_numeric),
                                    Operand2::Operand(InstructionOperand::Register(&dst)),
                                ],
                            );
                            self.emit2(
                                opcode,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Operand(InstructionOperand::Register(&dst_numeric)),
                                ],
                            );

                            self.emit2(
                                Opcode::SetPropertyByValue,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Register(&key),
                                    Operand2::Register(&object),
                                    Operand2::Register(&object),
                                ],
                            );

                            self.register_allocator.dealloc(key);
                            self.register_allocator.dealloc(object);

                            if post {
                                self.emit_move(dst, InstructionOperand::Register(&dst_numeric));
                            }

                            self.register_allocator.dealloc(dst_numeric);
                        }
                    }
                }
                PropertyAccess::Private(access) => {
                    let index = self.get_or_insert_private_name(access.field());

                    let object = self.register_allocator.alloc();
                    self.compile_expr(access.target(), &object);

                    self.emit2(
                        Opcode::GetPrivateField,
                        &[
                            Operand2::Register(&dst),
                            Operand2::Operand(InstructionOperand::Register(&object)),
                            Operand2::Varying(index),
                        ],
                    );

                    let dst_numeric = self.register_allocator.alloc();

                    self.emit2(
                        Opcode::ToNumeric,
                        &[
                            Operand2::Register(&dst_numeric),
                            Operand2::Operand(InstructionOperand::Register(&dst)),
                        ],
                    );
                    self.emit2(
                        opcode,
                        &[
                            Operand2::Register(&dst),
                            Operand2::Operand(InstructionOperand::Register(&dst_numeric)),
                        ],
                    );

                    self.emit2(
                        Opcode::SetPrivateField,
                        &[
                            Operand2::Operand(InstructionOperand::Register(&dst)),
                            Operand2::Operand(InstructionOperand::Register(&object)),
                            Operand2::Varying(index),
                        ],
                    );

                    self.register_allocator.dealloc(object);

                    if post {
                        self.emit_move(dst, InstructionOperand::Register(&dst_numeric));
                    }

                    self.register_allocator.dealloc(dst_numeric);
                }
                PropertyAccess::Super(access) => match access.field() {
                    PropertyAccessField::Const(ident) => {
                        let object = self.register_allocator.alloc();
                        let receiver = self.register_allocator.alloc();
                        self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                        self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                        self.emit_get_property_by_name(&dst, &receiver, &object, *ident);

                        let dst_numeric = self.register_allocator.alloc();
                        self.emit2(
                            Opcode::ToNumeric,
                            &[
                                Operand2::Register(&dst_numeric),
                                Operand2::Operand(InstructionOperand::Register(&dst)),
                            ],
                        );
                        self.emit2(
                            opcode,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Operand(InstructionOperand::Register(&dst_numeric)),
                            ],
                        );

                        self.emit_set_property_by_name(&dst, &receiver, &object, *ident);
                        if post {
                            self.emit_move(dst, InstructionOperand::Register(&dst_numeric));
                        }

                        self.register_allocator.dealloc(receiver);
                        self.register_allocator.dealloc(object);
                        self.register_allocator.dealloc(dst_numeric);
                    }
                    PropertyAccessField::Expr(expr) => {
                        let object = self.register_allocator.alloc();
                        let receiver = self.register_allocator.alloc();
                        self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                        self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                        let key = self.register_allocator.alloc();
                        self.compile_expr(expr, &key);

                        self.emit2(
                            Opcode::GetPropertyByValue,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Register(&key),
                                Operand2::Register(&receiver),
                                Operand2::Register(&object),
                            ],
                        );

                        self.emit2(
                            Opcode::ToNumeric,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Operand(InstructionOperand::Register(&dst)),
                            ],
                        );
                        self.emit2(
                            opcode,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Operand(InstructionOperand::Register(&dst)),
                            ],
                        );

                        self.emit2(
                            Opcode::SetPropertyByValue,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Register(&key),
                                Operand2::Register(&receiver),
                                Operand2::Register(&object),
                            ],
                        );

                        self.register_allocator.dealloc(receiver);
                        self.register_allocator.dealloc(object);
                        self.register_allocator.dealloc(key);
                    }
                },
            },
            Access::This => unreachable!(),
        }
    }
}
