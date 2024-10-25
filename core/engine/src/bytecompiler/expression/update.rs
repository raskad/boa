use crate::{
    bytecompiler::{Access, ByteCompiler, InstructionOperand, Operand2, ToJsString},
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
    pub(crate) fn compile_update(&mut self, update: &Update, use_expr: bool) {
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

                if is_lexical {
                    self.emit_binding_access(Opcode::GetName, &index);
                } else {
                    self.emit_binding_access(Opcode::GetNameAndLocator, &index);
                }

                let src = self.register_allocator.alloc();
                let dst = self.register_allocator.alloc();

                self.pop_into_register(&src);

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
                self.push_from_register(&src);

                if is_lexical {
                    match self.lexical_scope.set_mutable_binding(name.clone()) {
                        Ok(binding) => {
                            let index = self.get_or_insert_binding(binding);
                            self.emit_binding_access(Opcode::SetName, &index);
                        }
                        Err(BindingLocatorError::MutateImmutable) => {
                            let index = self.get_or_insert_string(name);
                            self.emit_with_varying_operand(Opcode::ThrowMutateImmutable, index);
                        }
                        Err(BindingLocatorError::Silent) => {
                            self.emit_opcode(Opcode::Pop);
                        }
                    }
                } else {
                    self.emit_binding_access(Opcode::SetNameByLocator, &index);
                }
                if post {
                    self.push_from_register(&dst);
                } else {
                    self.push_from_register(&src);
                }

                self.register_allocator.dealloc(src);
                self.register_allocator.dealloc(dst);
            }
            Access::Property { access } => match access {
                PropertyAccess::Simple(access) => {
                    let object = self.register_allocator.alloc();
                    self.compile_expr2(access.target(), &object);

                    match access.field() {
                        PropertyAccessField::Const(ident) => {
                            let dst = self.register_allocator.alloc();
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
                                self.push_from_register(&dst_numeric);
                            } else {
                                self.push_from_register(&dst);
                            }

                            self.register_allocator.dealloc(object);
                            self.register_allocator.dealloc(dst);
                            self.register_allocator.dealloc(dst_numeric);
                        }
                        PropertyAccessField::Expr(expr) => {
                            let key = self.register_allocator.alloc();
                            self.compile_expr2(expr, &key);

                            let value = self.register_allocator.alloc();

                            self.emit2(
                                Opcode::GetPropertyByValuePush,
                                &[
                                    Operand2::Register(&value),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                ],
                            );

                            let value_numeric = self.register_allocator.alloc();

                            self.emit2(
                                Opcode::ToNumeric,
                                &[
                                    Operand2::Register(&value_numeric),
                                    Operand2::Operand(InstructionOperand::Register(&value)),
                                ],
                            );
                            self.emit2(
                                opcode,
                                &[
                                    Operand2::Register(&value),
                                    Operand2::Operand(InstructionOperand::Register(&value_numeric)),
                                ],
                            );

                            self.emit2(
                                Opcode::SetPropertyByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&value)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                ],
                            );

                            self.register_allocator.dealloc(key);
                            self.register_allocator.dealloc(object);

                            if post {
                                self.push_from_register(&value_numeric);
                            } else {
                                self.push_from_register(&value);
                            }

                            self.register_allocator.dealloc(value);
                            self.register_allocator.dealloc(value_numeric);
                        }
                    }
                }
                PropertyAccess::Private(access) => {
                    let index = self.get_or_insert_private_name(access.field());

                    let object = self.register_allocator.alloc();
                    self.compile_expr2(access.target(), &object);

                    let value = self.register_allocator.alloc();

                    self.emit2(
                        Opcode::GetPrivateField,
                        &[
                            Operand2::Register(&value),
                            Operand2::Operand(InstructionOperand::Register(&object)),
                            Operand2::Varying(index),
                        ],
                    );

                    let value_numeric = self.register_allocator.alloc();

                    self.emit2(
                        Opcode::ToNumeric,
                        &[
                            Operand2::Register(&value_numeric),
                            Operand2::Operand(InstructionOperand::Register(&value)),
                        ],
                    );
                    self.emit2(
                        opcode,
                        &[
                            Operand2::Register(&value),
                            Operand2::Operand(InstructionOperand::Register(&value_numeric)),
                        ],
                    );

                    self.emit2(
                        Opcode::SetPrivateField,
                        &[
                            Operand2::Operand(InstructionOperand::Register(&value)),
                            Operand2::Operand(InstructionOperand::Register(&object)),
                            Operand2::Varying(index),
                        ],
                    );

                    self.register_allocator.dealloc(object);

                    if post {
                        self.push_from_register(&value_numeric);
                    } else {
                        self.push_from_register(&value);
                    }

                    self.register_allocator.dealloc(value);
                    self.register_allocator.dealloc(value_numeric);
                }
                PropertyAccess::Super(access) => match access.field() {
                    PropertyAccessField::Const(ident) => {
                        let object = self.register_allocator.alloc();
                        let receiver = self.register_allocator.alloc();
                        self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                        self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                        let dst = self.register_allocator.alloc();

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
                            self.push_from_register(&dst_numeric);
                        } else {
                            self.push_from_register(&dst);
                        }

                        self.register_allocator.dealloc(receiver);
                        self.register_allocator.dealloc(object);
                        self.register_allocator.dealloc(dst);
                        self.register_allocator.dealloc(dst_numeric);
                    }
                    PropertyAccessField::Expr(expr) => {
                        let object = self.register_allocator.alloc();
                        let receiver = self.register_allocator.alloc();
                        self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                        self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                        let key = self.register_allocator.alloc();
                        self.compile_expr2(expr, &key);

                        let value = self.register_allocator.alloc();

                        self.emit2(
                            Opcode::GetPropertyByValue,
                            &[
                                Operand2::Register(&value),
                                Operand2::Operand(InstructionOperand::Register(&key)),
                                Operand2::Operand(InstructionOperand::Register(&receiver)),
                                Operand2::Operand(InstructionOperand::Register(&object)),
                            ],
                        );

                        self.emit2(
                            Opcode::ToNumeric,
                            &[
                                Operand2::Register(&value),
                                Operand2::Operand(InstructionOperand::Register(&value)),
                            ],
                        );
                        self.emit2(
                            opcode,
                            &[
                                Operand2::Register(&value),
                                Operand2::Operand(InstructionOperand::Register(&value)),
                            ],
                        );

                        self.emit2(
                            Opcode::SetPropertyByValue,
                            &[
                                Operand2::Operand(InstructionOperand::Register(&value)),
                                Operand2::Operand(InstructionOperand::Register(&key)),
                                Operand2::Operand(InstructionOperand::Register(&receiver)),
                                Operand2::Operand(InstructionOperand::Register(&object)),
                            ],
                        );

                        self.register_allocator.dealloc(receiver);
                        self.register_allocator.dealloc(object);
                        self.register_allocator.dealloc(key);

                        if post {
                            self.push_from_register(&value);
                        } else {
                            self.push_from_register(&value);
                        }

                        self.register_allocator.dealloc(value);
                    }
                },
            },
            Access::This => unreachable!(),
        }

        if !use_expr {
            self.emit_opcode(Opcode::Pop);
        }
    }
}
