use crate::{
    bytecompiler::{Access, ByteCompiler, InstructionOperand, Operand2, Register, ToJsString},
    vm::{BindingOpcode, Opcode},
};
use boa_ast::{
    expression::{
        access::{PropertyAccess, PropertyAccessField},
        operator::{assign::AssignOp, Assign},
    },
    scope::BindingLocatorError,
};

impl ByteCompiler<'_> {
    pub(crate) fn compile_assign(&mut self, assign: &Assign, dst: &Register) {
        if assign.op() == AssignOp::Assign {
            match Access::from_assign_target(assign.lhs()) {
                Ok(access) => {
                    self.access_set(access, |compiler| {
                        compiler.compile_expr(assign.rhs(), &dst);
                        return &dst;
                    });
                }
                Err(pattern) => {
                    self.compile_expr(assign.rhs(), &dst);
                    self.compile_declaration_pattern(pattern, BindingOpcode::SetName, &dst);
                }
            }
        } else {
            let access = Access::from_assign_target(assign.lhs())
                .expect("patterns should throw early errors on complex assignment operators");

            let opcode = match assign.op() {
                AssignOp::Assign => unreachable!(),
                AssignOp::Add => Opcode::Add,
                AssignOp::Sub => Opcode::Sub,
                AssignOp::Mul => Opcode::Mul,
                AssignOp::Div => Opcode::Div,
                AssignOp::Mod => Opcode::Mod,
                AssignOp::Exp => Opcode::Pow,
                AssignOp::And => Opcode::BitAnd,
                AssignOp::Or => Opcode::BitOr,
                AssignOp::Xor => Opcode::BitXor,
                AssignOp::Shl => Opcode::ShiftLeft,
                AssignOp::Shr => Opcode::ShiftRight,
                AssignOp::Ushr => Opcode::UnsignedShiftRight,
                AssignOp::BoolAnd => Opcode::LogicalAnd,
                AssignOp::BoolOr => Opcode::LogicalOr,
                AssignOp::Coalesce => Opcode::Coalesce,
            };

            let short_circuit = matches!(
                assign.op(),
                AssignOp::BoolAnd | AssignOp::BoolOr | AssignOp::Coalesce
            );
            let mut early_exit = None;

            match access {
                Access::Variable { name } => {
                    let name = name.to_js_string(self.interner());

                    let binding = self.lexical_scope.get_identifier_reference(name.clone());
                    let is_lexical = binding.is_lexical();
                    let index = self.get_or_insert_binding(binding);

                    if is_lexical {
                        self.emit_binding_access(Opcode::GetName, &index, &dst);
                    } else {
                        self.emit_binding_access(Opcode::GetNameAndLocator, &index, &dst);
                    }

                    if short_circuit {
                        early_exit =
                            Some(self.emit_opcode_with_operand2(
                                opcode,
                                InstructionOperand::Register(&dst),
                            ));

                        self.compile_expr(assign.rhs(), &dst);
                    } else {
                        let rhs = self.register_allocator.alloc();
                        self.compile_expr(assign.rhs(), &rhs);
                        self.emit2(
                            opcode,
                            &[
                                Operand2::Register(&dst),
                                Operand2::Operand(InstructionOperand::Register(&dst)),
                                Operand2::Operand(InstructionOperand::Register(&rhs)),
                            ],
                        );
                        self.register_allocator.dealloc(rhs);
                    }

                    if is_lexical {
                        match self.lexical_scope.set_mutable_binding(name.clone()) {
                            Ok(binding) => {
                                let index = self.get_or_insert_binding(binding);
                                self.emit_binding_access(Opcode::SetName, &index, &dst);
                            }
                            Err(BindingLocatorError::MutateImmutable) => {
                                let index = self.get_or_insert_string(name);
                                self.emit_with_varying_operand(Opcode::ThrowMutateImmutable, index);
                            }
                            Err(BindingLocatorError::Silent) => {}
                        }
                    } else {
                        self.emit_binding_access(Opcode::SetNameByLocator, &index, &dst);
                    }
                }
                Access::Property { access } => match access {
                    PropertyAccess::Simple(access) => match access.field() {
                        PropertyAccessField::Const(name) => {
                            let object = self.register_allocator.alloc();
                            self.compile_expr(access.target(), &object);

                            self.emit_get_property_by_name(&dst, &object, &object, *name);

                            if short_circuit {
                                early_exit = Some(self.emit_opcode_with_operand2(
                                    opcode,
                                    InstructionOperand::Register(&dst),
                                ));
                                self.compile_expr(assign.rhs(), &dst);
                            } else {
                                let rhs = self.register_allocator.alloc();
                                self.compile_expr(assign.rhs(), &rhs);
                                self.emit2(
                                    opcode,
                                    &[
                                        Operand2::Register(&dst),
                                        Operand2::Operand(InstructionOperand::Register(&dst)),
                                        Operand2::Operand(InstructionOperand::Register(&rhs)),
                                    ],
                                );
                                self.register_allocator.dealloc(rhs);
                            }

                            self.emit_set_property_by_name(&dst, &object, &object, *name);

                            self.register_allocator.dealloc(object);
                        }
                        PropertyAccessField::Expr(expr) => {
                            let object = self.register_allocator.alloc();
                            self.compile_expr(access.target(), &object);

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

                            if short_circuit {
                                early_exit = Some(self.emit_opcode_with_operand2(
                                    opcode,
                                    InstructionOperand::Register(&dst),
                                ));
                                self.compile_expr(assign.rhs(), &dst);
                            } else {
                                let rhs = self.register_allocator.alloc();
                                self.compile_expr(assign.rhs(), &rhs);
                                self.emit2(
                                    opcode,
                                    &[
                                        Operand2::Register(&dst),
                                        Operand2::Operand(InstructionOperand::Register(&dst)),
                                        Operand2::Operand(InstructionOperand::Register(&rhs)),
                                    ],
                                );
                                self.register_allocator.dealloc(rhs);
                            }

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
                        }
                    },
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

                        if short_circuit {
                            early_exit = Some(self.emit_opcode_with_operand2(
                                opcode,
                                InstructionOperand::Register(&dst),
                            ));
                            self.compile_expr(assign.rhs(), &dst);
                        } else {
                            let rhs = self.register_allocator.alloc();
                            self.compile_expr(assign.rhs(), &rhs);

                            self.emit2(
                                opcode,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Operand(InstructionOperand::Register(&dst)),
                                    Operand2::Operand(InstructionOperand::Register(&rhs)),
                                ],
                            );
                            self.register_allocator.dealloc(rhs);
                        }

                        self.emit2(
                            Opcode::SetPrivateField,
                            &[
                                Operand2::Operand(InstructionOperand::Register(&dst)),
                                Operand2::Operand(InstructionOperand::Register(&object)),
                                Operand2::Varying(index),
                            ],
                        );

                        self.register_allocator.dealloc(object);
                    }
                    PropertyAccess::Super(access) => match access.field() {
                        PropertyAccessField::Const(name) => {
                            let object = self.register_allocator.alloc();
                            let receiver = self.register_allocator.alloc();
                            self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                            self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                            self.emit_get_property_by_name(&dst, &receiver, &object, *name);

                            if short_circuit {
                                early_exit = Some(self.emit_opcode_with_operand2(
                                    opcode,
                                    InstructionOperand::Register(&dst),
                                ));
                                self.compile_expr(assign.rhs(), &dst);
                            } else {
                                let rhs = self.register_allocator.alloc();
                                self.compile_expr(assign.rhs(), &rhs);
                                self.emit2(
                                    opcode,
                                    &[
                                        Operand2::Register(&dst),
                                        Operand2::Operand(InstructionOperand::Register(&dst)),
                                        Operand2::Operand(InstructionOperand::Register(&rhs)),
                                    ],
                                );
                                self.register_allocator.dealloc(rhs);
                            }

                            self.emit_set_property_by_name(&dst, &receiver, &object, *name);

                            self.register_allocator.dealloc(receiver);
                            self.register_allocator.dealloc(object);
                        }
                        PropertyAccessField::Expr(expr) => {
                            let object = self.register_allocator.alloc();
                            let receiver = self.register_allocator.alloc();
                            self.emit2(Opcode::Super, &[Operand2::Register(&object)]);
                            self.emit2(Opcode::This, &[Operand2::Register(&receiver)]);

                            let key = self.register_allocator.alloc();
                            self.compile_expr(expr, &key);

                            self.emit2(
                                Opcode::GetPropertyByValuePush,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(&receiver)),
                                    Operand2::Operand(InstructionOperand::Register(&object)),
                                ],
                            );

                            if short_circuit {
                                early_exit = Some(self.emit_opcode_with_operand2(
                                    opcode,
                                    InstructionOperand::Register(&dst),
                                ));
                                self.compile_expr(assign.rhs(), &dst);
                            } else {
                                let rhs = self.register_allocator.alloc();
                                self.compile_expr(assign.rhs(), &rhs);
                                self.emit2(
                                    opcode,
                                    &[
                                        Operand2::Register(&dst),
                                        Operand2::Operand(InstructionOperand::Register(&dst)),
                                        Operand2::Operand(InstructionOperand::Register(&rhs)),
                                    ],
                                );
                                self.register_allocator.dealloc(rhs);
                            }

                            self.emit2(
                                Opcode::SetPropertyByValue,
                                &[
                                    Operand2::Register(&dst),
                                    Operand2::Register(&key),
                                    Operand2::Register(&receiver),
                                    Operand2::Register(&object),
                                ],
                            );

                            self.register_allocator.dealloc(key);
                            self.register_allocator.dealloc(receiver);
                            self.register_allocator.dealloc(object);
                        }
                    },
                },
                Access::This => unreachable!(),
            }

            if let Some(early_exit) = early_exit {
                let exit = self.emit_opcode_with_operand(Opcode::Jump);
                self.patch_jump(early_exit);
                self.patch_jump(exit);
            }
        }
    }
}
