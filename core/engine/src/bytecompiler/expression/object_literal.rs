use crate::{
    bytecompiler::{Access, ByteCompiler, FunctionSpec, MethodKind, Operand, Operand2, Register},
    vm::Opcode,
};
use boa_ast::{
    expression::literal::{ObjectLiteral, PropertyDefinition},
    property::{MethodDefinitionKind, PropertyName},
    Expression,
};
use boa_interner::Sym;

impl ByteCompiler<'_> {
    pub(crate) fn compile_object_literal(&mut self, literal: &ObjectLiteral, use_expr: bool) {
        self.emit_opcode(Opcode::PushEmptyObject);
        let object = self.register_allocator.alloc();
        self.pop_into_register(&object);

        for property in literal.properties() {
            match property {
                PropertyDefinition::IdentifierReference(ident) => {
                    self.push_from_register(&object);
                    let index = self.get_or_insert_name(*ident);
                    self.access_get(Access::Variable { name: *ident }, true);
                    self.emit_with_varying_operand(Opcode::DefineOwnPropertyByName, index);
                }
                PropertyDefinition::Property(name, expr) => match name {
                    PropertyName::Literal(name) => {
                        self.push_from_register(&object);
                        self.compile_expr(expr, true);
                        let index = self.get_or_insert_name((*name).into());
                        if *name == Sym::__PROTO__ && !self.json_parse {
                            self.emit_opcode(Opcode::SetPrototype);
                        } else {
                            self.emit_with_varying_operand(Opcode::DefineOwnPropertyByName, index);
                        }
                    }
                    PropertyName::Computed(name_node) => {
                        let key = self.register_allocator.alloc();
                        self.compile_expr2(name_node, &key);
                        self.emit2(
                            Opcode::ToPropertyKey,
                            &[Operand2::Register(&key), Operand2::Register(&key)],
                        );
                        let function = self.register_allocator.alloc();
                        self.compile_expr2(expr, &function);
                        if expr.is_anonymous_function_definition() {
                            self.emit2(Opcode::SetFunctionName, &[
                                Operand2::Register(&function),
                                Operand2::Register(&key),
                                Operand2::U8(0),
                            ]);
                        }
                        self.emit2(Opcode::DefineOwnPropertyByValue, &[
                            Operand2::Register(&function),
                            Operand2::Register(&key),
                            Operand2::Register(&object),
                        ]);
                        self.register_allocator.dealloc(key);
                        self.register_allocator.dealloc(function);
                    }
                },
                PropertyDefinition::MethodDefinition(m) => {
                    let kind = match m.kind() {
                        MethodDefinitionKind::Get => MethodKind::Get,
                        MethodDefinitionKind::Set => MethodKind::Set,
                        _ => MethodKind::Ordinary,
                    };
                    match m.name() {
                        PropertyName::Literal(name) => {
                            let method = self.object_method(m.into(), kind);
                            self.emit2(Opcode::SetHomeObject, &[
                                Operand2::Register(&method),
                                Operand2::Register(&object),
                            ]);
                            let index = self.get_or_insert_name((*name).into());
                            let opcode = match kind {
                                MethodKind::Get => Opcode::SetPropertyGetterByName,
                                MethodKind::Set => Opcode::SetPropertySetterByName,
                                MethodKind::Ordinary => Opcode::DefineOwnPropertyByName,
                            };
                            self.push_from_register(&object);
                            self.push_from_register(&method);
                            self.emit_with_varying_operand(opcode, index);

                            self.register_allocator.dealloc(method);
                        }
                        PropertyName::Computed(name_node) => {
                            self.compile_object_literal_computed_method(
                                name_node,
                                m.into(),
                                kind,
                                &object,
                            );
                        }
                    }
                }
                PropertyDefinition::SpreadObject(expr) => {
                    self.compile_expr(expr, true);
                    self.push_from_register(&object);
                    self.emit(
                        Opcode::CopyDataProperties,
                        &[Operand::Varying(0), Operand::Varying(0)],
                    );
                    self.emit_opcode(Opcode::Pop);
                }
                PropertyDefinition::CoverInitializedName(_, _) => {
                    unreachable!("invalid assignment pattern in object literal")
                }
            }
        }

        if use_expr {
            self.push_from_register(&object);
        }

        self.register_allocator.dealloc(object);
    }

    fn compile_object_literal_computed_method(
        &mut self,
        expr: &Expression,
        function: FunctionSpec<'_>,
        kind: MethodKind,
        object: &Register,
    ) {
        let key = self.register_allocator.alloc();
        self.compile_expr2(expr, &key);

        self.emit2(
            Opcode::ToPropertyKey,
            &[Operand2::Register(&key), Operand2::Register(&key)],
        );

        let method = self.object_method(function, kind);
        let value = match kind {
            MethodKind::Get => 1,
            MethodKind::Set => 2,
            MethodKind::Ordinary => 0,
        };

        self.emit2(Opcode::SetFunctionName, &[
            Operand2::Register(&method),
            Operand2::Register(&key),
            Operand2::U8(value),
        ]);

        self.emit2(Opcode::SetHomeObject, &[
            Operand2::Register(&method),
            Operand2::Register(&object),
        ]);

        let operands = &[
            Operand2::Register(&method),
            Operand2::Register(&key),
            Operand2::Register(&object),
        ];
        match kind {
            MethodKind::Get => self.emit2(Opcode::SetPropertyGetterByValue, operands),
            MethodKind::Set => self.emit2(Opcode::SetPropertySetterByValue, operands),
            MethodKind::Ordinary => self.emit2(Opcode::DefineOwnPropertyByValue, operands),
        }

        self.register_allocator.dealloc(key);
        self.register_allocator.dealloc(method);
    }
}
