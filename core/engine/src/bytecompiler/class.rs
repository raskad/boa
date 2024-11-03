use super::{ByteCompiler, InstructionOperand, Literal, Operand2, Register, ToJsString};
use crate::{
    js_string,
    vm::{BindingOpcode, CodeBlock, CodeBlockFlags, Opcode},
};
use boa_ast::{
    expression::Identifier,
    function::{
        ClassDeclaration, ClassElement, ClassElementName, ClassExpression, FormalParameterList,
        FunctionExpression,
    },
    property::{MethodDefinitionKind, PropertyName},
    scope::Scope,
    Expression,
};
use boa_gc::Gc;
use boa_interner::Sym;

// Static class elements that are initialized at a later time in the class creation.
enum StaticElement {
    // A static class block with it's function code.
    StaticBlock(Gc<CodeBlock>),

    // A static class field with it's function code, an optional name index and the information if the function is an anonymous function.
    StaticField((Gc<CodeBlock>, StaticFieldName, bool)),
}

enum StaticFieldName {
    Index(u32),
    Register(Register),
}

/// Describes the complete specification of a class.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ClassSpec<'a> {
    name: Option<Identifier>,
    super_ref: Option<&'a Expression>,
    constructor: Option<&'a FunctionExpression>,
    elements: &'a [ClassElement],
    has_binding_identifier: bool,
    name_scope: Option<&'a Scope>,
}

impl<'a> From<&'a ClassDeclaration> for ClassSpec<'a> {
    fn from(class: &'a ClassDeclaration) -> Self {
        Self {
            name: Some(class.name()),
            super_ref: class.super_ref(),
            constructor: class.constructor(),
            elements: class.elements(),
            has_binding_identifier: true,
            name_scope: Some(class.name_scope()),
        }
    }
}

impl<'a> From<&'a ClassExpression> for ClassSpec<'a> {
    fn from(class: &'a ClassExpression) -> Self {
        Self {
            name: class.name(),
            super_ref: class.super_ref(),
            constructor: class.constructor(),
            elements: class.elements(),
            has_binding_identifier: class.name().is_some(),
            name_scope: class.name_scope(),
        }
    }
}

impl ByteCompiler<'_> {
    /// This function compiles a class declaration or expression.
    ///
    /// The compilation of a class declaration and expression is mostly equal.
    /// A class declaration binds the resulting class object to it's identifier.
    /// A class expression leaves the resulting class object on the stack for following operations.
    pub(crate) fn compile_class(&mut self, class: ClassSpec<'_>, expression: bool) {
        // 11.2.2 Strict Mode Code - <https://tc39.es/ecma262/#sec-strict-mode-code>
        //  - All parts of a ClassDeclaration or a ClassExpression are strict mode code.
        let strict = self.strict();
        self.code_block_flags |= CodeBlockFlags::STRICT;

        let class_name = class
            .name
            .map_or(Sym::EMPTY_STRING, Identifier::sym)
            .to_js_string(self.interner());

        let outer_scope = if let Some(name_scope) = class.name_scope {
            let outer_scope = self.lexical_scope.clone();
            let scope_index = self.push_scope(name_scope);
            self.emit_with_varying_operand(Opcode::PushScope, scope_index);
            Some(outer_scope)
        } else {
            None
        };

        let mut compiler = ByteCompiler::new(
            class_name.clone(),
            true,
            self.json_parse,
            self.variable_scope.clone(),
            self.lexical_scope.clone(),
            false,
            false,
            self.interner,
            self.in_with,
        );

        compiler.code_block_flags |= CodeBlockFlags::IS_CLASS_CONSTRUCTOR;

        if let Some(expr) = &class.constructor {
            let _ = compiler.push_scope(expr.scopes().function_scope());

            compiler.length = expr.parameters().length();
            compiler.params = expr.parameters().clone();

            compiler.function_declaration_instantiation(
                expr.body(),
                expr.parameters(),
                false,
                true,
                false,
                expr.scopes(),
            );

            compiler.compile_statement_list(expr.body().statement_list(), false, false);

            let value = compiler.register_allocator.alloc();
            compiler.push_undefined(&value);
            compiler.push_from_register(&value);
            compiler.register_allocator.dealloc(value);
        } else if class.super_ref.is_some() {
            // We push an empty, unused function scope since the compiler expects a function scope.
            let _ = compiler.push_scope(&Scope::new(compiler.lexical_scope.clone(), true));
            compiler.emit_opcode(Opcode::SuperCallDerived);
            let value = compiler.register_allocator.alloc();
            compiler.pop_into_register(&value);
            compiler.emit2(Opcode::BindThisValue, &[Operand2::Register(&value)]);
            compiler.push_from_register(&value);
            compiler.register_allocator.dealloc(value);
        } else {
            // We push an empty, unused function scope since the compiler expects a function scope.
            let _ = compiler.push_scope(&Scope::new(compiler.lexical_scope.clone(), true));
            let value = compiler.register_allocator.alloc();
            compiler.push_undefined(&value);
            compiler.push_from_register(&value);
            compiler.register_allocator.dealloc(value);
        }
        compiler.emit_opcode(Opcode::SetAccumulatorFromStack);

        // 17. If ClassHeritageopt is present, set F.[[ConstructorKind]] to derived.
        compiler.code_block_flags.set(
            CodeBlockFlags::IS_DERIVED_CONSTRUCTOR,
            class.super_ref.is_some(),
        );

        let code = Gc::new(compiler.finish());
        let index = self.push_function_to_constants(code);

        let class_register = self.register_allocator.alloc();
        self.emit_get_function(&class_register, index);

        let prototype_register = self.register_allocator.alloc();

        if let Some(node) = class.super_ref {
            self.compile_expr(node, &prototype_register);

            self.emit2(
                Opcode::PushClassPrototype,
                &[
                    Operand2::Register(&prototype_register),
                    Operand2::Operand(InstructionOperand::Register(&class_register)),
                    Operand2::Operand(InstructionOperand::Register(&prototype_register)),
                ],
            );
        } else {
            self.push_undefined(&prototype_register);
        }

        let proto_register = self.register_allocator.alloc();

        self.emit2(
            Opcode::SetClassPrototype,
            &[
                Operand2::Register(&proto_register),
                Operand2::Operand(InstructionOperand::Register(&prototype_register)),
                Operand2::Operand(InstructionOperand::Register(&class_register)),
            ],
        );
        self.register_allocator.dealloc(prototype_register);

        let count_label =
            self.emit_push_private_environment(InstructionOperand::Register(&class_register));
        let mut count = 0;
        for element in class.elements {
            match element {
                ClassElement::MethodDefinition(m) => {
                    if let ClassElementName::PrivateName(name) = m.name() {
                        count += 1;
                        let index = self.get_or_insert_private_name(*name);
                        self.emit_u32(index);
                    }
                }
                ClassElement::PrivateFieldDefinition(field) => {
                    count += 1;
                    let index = self.get_or_insert_private_name(*field.name());
                    self.emit_u32(index);
                }
                ClassElement::PrivateStaticFieldDefinition(name, _) => {
                    count += 1;
                    let index = self.get_or_insert_private_name(*name);
                    self.emit_u32(index);
                }
                _ => {}
            }
        }
        self.patch_jump_with_target(count_label, count);

        let mut static_elements = Vec::new();

        if outer_scope.is_some() {
            self.push_from_register(&class_register);
            self.emit_binding(BindingOpcode::InitLexical, class_name.clone());
        }

        for element in class.elements {
            match element {
                ClassElement::MethodDefinition(m) => match m.name() {
                    ClassElementName::PropertyName(PropertyName::Literal(name)) => {
                        let index = self.get_or_insert_name((*name).into());
                        let method = self.method(m.into());

                        match (m.is_static(), m.kind()) {
                            (true, MethodDefinitionKind::Get) => self.emit2(
                                Opcode::DefineClassStaticGetterByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                            (true, MethodDefinitionKind::Set) => self.emit2(
                                Opcode::DefineClassStaticSetterByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                            (true, _) => self.emit2(
                                Opcode::DefineClassStaticMethodByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                            (false, MethodDefinitionKind::Get) => self.emit2(
                                Opcode::DefineClassGetterByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                            (false, MethodDefinitionKind::Set) => self.emit2(
                                Opcode::DefineClassSetterByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                            (false, _) => self.emit2(
                                Opcode::DefineClassMethodByName,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                    Operand2::Varying(index),
                                ],
                            ),
                        }

                        self.register_allocator.dealloc(method);
                    }
                    ClassElementName::PropertyName(PropertyName::Computed(name)) => {
                        let key = self.register_allocator.alloc();
                        self.compile_expr(name, &key);
                        self.emit2(
                            Opcode::ToPropertyKey,
                            &[Operand2::Register(&key), Operand2::Register(&key)],
                        );
                        let method = self.method(m.into());
                        match (m.is_static(), m.kind()) {
                            (true, MethodDefinitionKind::Get) => self.emit2(
                                Opcode::DefineClassStaticGetterByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                ],
                            ),
                            (true, MethodDefinitionKind::Set) => self.emit2(
                                Opcode::DefineClassStaticSetterByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                ],
                            ),
                            (true, _) => self.emit2(
                                Opcode::DefineClassStaticMethodByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &class_register,
                                    )),
                                ],
                            ),
                            (false, MethodDefinitionKind::Get) => self.emit2(
                                Opcode::DefineClassGetterByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                ],
                            ),
                            (false, MethodDefinitionKind::Set) => self.emit2(
                                Opcode::DefineClassSetterByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                ],
                            ),
                            (false, _) => self.emit2(
                                Opcode::DefineClassMethodByValue,
                                &[
                                    Operand2::Operand(InstructionOperand::Register(&method)),
                                    Operand2::Operand(InstructionOperand::Register(&key)),
                                    Operand2::Operand(InstructionOperand::Register(
                                        &proto_register,
                                    )),
                                ],
                            ),
                        }
                        self.register_allocator.dealloc(method);
                        self.register_allocator.dealloc(key);
                    }
                    ClassElementName::PrivateName(name) => {
                        let index = self.get_or_insert_private_name(*name);
                        let method = self.method(m.into());
                        match (m.is_static(), m.kind()) {
                            (true, MethodDefinitionKind::Get) => {
                                self.emit2(
                                    Opcode::SetPrivateGetter,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                            (true, MethodDefinitionKind::Set) => {
                                self.emit2(
                                    Opcode::SetPrivateSetter,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                            (true, _) => {
                                self.emit2(
                                    Opcode::SetPrivateMethod,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                            (false, MethodDefinitionKind::Get) => {
                                self.emit2(
                                    Opcode::PushClassPrivateGetter,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                            (false, MethodDefinitionKind::Set) => {
                                self.emit2(
                                    Opcode::PushClassPrivateSetter,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                            (false, _) => {
                                self.emit2(
                                    Opcode::PushClassPrivateMethod,
                                    &[
                                        Operand2::Register(&class_register),
                                        Operand2::Register(&proto_register),
                                        Operand2::Register(&method),
                                        Operand2::Varying(index),
                                    ],
                                );
                            }
                        }
                        self.register_allocator.dealloc(method);
                    }
                },
                ClassElement::FieldDefinition(field) => {
                    let name = self.register_allocator.alloc();
                    match field.name() {
                        PropertyName::Literal(ident) => {
                            self.emit_push_literal(
                                Literal::String(
                                    self.interner().resolve_expect(*ident).into_common(false),
                                ),
                                &name,
                            );
                        }
                        PropertyName::Computed(expr) => {
                            self.compile_expr(expr, &name);
                        }
                    }
                    let mut field_compiler = ByteCompiler::new(
                        js_string!(),
                        true,
                        self.json_parse,
                        self.variable_scope.clone(),
                        self.lexical_scope.clone(),
                        false,
                        false,
                        self.interner,
                        self.in_with,
                    );

                    // Function environment
                    let _ = field_compiler.push_scope(field.scope());
                    let is_anonymous_function = if let Some(node) = &field.field() {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.compile_expr(node, &value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                        node.is_anonymous_function_definition()
                    } else {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.push_undefined(&value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                        false
                    };

                    field_compiler.emit_opcode(Opcode::SetAccumulatorFromStack);

                    field_compiler.code_block_flags |= CodeBlockFlags::IN_CLASS_FIELD_INITIALIZER;

                    let code = Gc::new(field_compiler.finish());
                    let index = self.push_function_to_constants(code);

                    let dst = self.register_allocator.alloc();
                    self.emit_get_function(&dst, index);

                    self.emit2(
                        Opcode::PushClassField,
                        &[
                            Operand2::Register(&class_register),
                            Operand2::Register(&name),
                            Operand2::Register(&dst),
                            Operand2::Bool(is_anonymous_function),
                        ],
                    );

                    self.register_allocator.dealloc(name);
                    self.register_allocator.dealloc(dst);
                }
                ClassElement::PrivateFieldDefinition(field) => {
                    let name_index = self.get_or_insert_private_name(*field.name());
                    let mut field_compiler = ByteCompiler::new(
                        class_name.clone(),
                        true,
                        self.json_parse,
                        self.variable_scope.clone(),
                        self.lexical_scope.clone(),
                        false,
                        false,
                        self.interner,
                        self.in_with,
                    );
                    let _ = field_compiler.push_scope(field.scope());
                    if let Some(node) = field.field() {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.compile_expr(node, &value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                    } else {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.push_undefined(&value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                    }
                    field_compiler.emit_opcode(Opcode::SetAccumulatorFromStack);

                    field_compiler.code_block_flags |= CodeBlockFlags::IN_CLASS_FIELD_INITIALIZER;

                    let code = Gc::new(field_compiler.finish());
                    let index = self.push_function_to_constants(code);
                    let dst = self.register_allocator.alloc();
                    self.emit_get_function(&dst, index);

                    self.emit2(
                        Opcode::PushClassFieldPrivate,
                        &[
                            Operand2::Register(&class_register),
                            Operand2::Register(&dst),
                            Operand2::Varying(name_index),
                        ],
                    );

                    self.register_allocator.dealloc(dst);
                }
                ClassElement::StaticFieldDefinition(field) => {
                    let name_index = match field.name() {
                        PropertyName::Literal(name) => {
                            StaticFieldName::Index(self.get_or_insert_name((*name).into()))
                        }
                        PropertyName::Computed(name) => {
                            let name_register = self.register_allocator.alloc();
                            self.compile_expr(name, &name_register);
                            StaticFieldName::Register(name_register)
                        }
                    };
                    let mut field_compiler = ByteCompiler::new(
                        class_name.clone(),
                        true,
                        self.json_parse,
                        self.variable_scope.clone(),
                        self.lexical_scope.clone(),
                        false,
                        false,
                        self.interner,
                        self.in_with,
                    );
                    let _ = field_compiler.push_scope(field.scope());
                    let is_anonymous_function = if let Some(node) = &field.field() {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.compile_expr(node, &value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                        node.is_anonymous_function_definition()
                    } else {
                        let value = field_compiler.register_allocator.alloc();
                        field_compiler.push_undefined(&value);
                        field_compiler.push_from_register(&value);
                        field_compiler.register_allocator.dealloc(value);
                        false
                    };

                    field_compiler.emit_opcode(Opcode::SetAccumulatorFromStack);

                    field_compiler.code_block_flags |= CodeBlockFlags::IN_CLASS_FIELD_INITIALIZER;

                    let code = field_compiler.finish();
                    let code = Gc::new(code);

                    static_elements.push(StaticElement::StaticField((
                        code,
                        name_index,
                        is_anonymous_function,
                    )));
                }
                ClassElement::PrivateStaticFieldDefinition(name, field) => {
                    let value = self.register_allocator.alloc();
                    if let Some(expr) = &field {
                        self.compile_expr(expr, &value);
                    } else {
                        self.push_undefined(&value);
                    }
                    let index = self.get_or_insert_private_name(*name);
                    self.emit2(
                        Opcode::DefinePrivateField,
                        &[
                            Operand2::Register(&class_register),
                            Operand2::Register(&value),
                            Operand2::Varying(index),
                        ],
                    );
                    self.register_allocator.dealloc(value);
                }
                ClassElement::StaticBlock(block) => {
                    let mut compiler = ByteCompiler::new(
                        Sym::EMPTY_STRING.to_js_string(self.interner()),
                        true,
                        false,
                        self.variable_scope.clone(),
                        self.lexical_scope.clone(),
                        false,
                        false,
                        self.interner,
                        self.in_with,
                    );
                    let _ = compiler.push_scope(block.scopes().function_scope());

                    compiler.function_declaration_instantiation(
                        block.statements(),
                        &FormalParameterList::default(),
                        false,
                        true,
                        false,
                        block.scopes(),
                    );

                    compiler.compile_statement_list(
                        block.statements().statement_list(),
                        false,
                        false,
                    );

                    let code = Gc::new(compiler.finish());
                    static_elements.push(StaticElement::StaticBlock(code));
                }
            }
        }

        for element in static_elements {
            match element {
                StaticElement::StaticBlock(code) => {
                    let index = self.push_function_to_constants(code);
                    let function = self.register_allocator.alloc();
                    self.emit_get_function(&function, index);
                    self.emit2(
                        Opcode::SetHomeObject,
                        &[
                            Operand2::Register(&function),
                            Operand2::Register(&class_register),
                        ],
                    );
                    self.push_from_register(&class_register);
                    self.push_from_register(&function);
                    self.register_allocator.dealloc(function);
                    self.emit_with_varying_operand(Opcode::Call, 0);
                    self.emit_opcode(Opcode::Pop);
                }
                StaticElement::StaticField((code, name, is_anonymous_function)) => {
                    let index = self.push_function_to_constants(code);
                    let function = self.register_allocator.alloc();
                    self.emit_get_function(&function, index);
                    self.emit2(
                        Opcode::SetHomeObject,
                        &[
                            Operand2::Register(&function),
                            Operand2::Register(&class_register),
                        ],
                    );
                    self.push_from_register(&class_register);
                    self.push_from_register(&function);
                    self.register_allocator.dealloc(function);
                    self.emit_with_varying_operand(Opcode::Call, 0);
                    let value = self.register_allocator.alloc();
                    self.pop_into_register(&value);
                    match name {
                        StaticFieldName::Index(name) => {
                            self.emit2(
                                Opcode::DefineOwnPropertyByName,
                                &[
                                    Operand2::Register(&class_register),
                                    Operand2::Register(&value),
                                    Operand2::Varying(name),
                                ],
                            );
                        }
                        StaticFieldName::Register(key) => {
                            if is_anonymous_function {
                                self.emit2(
                                    Opcode::ToPropertyKey,
                                    &[Operand2::Register(&key), Operand2::Register(&key)],
                                );
                                self.emit2(
                                    Opcode::SetFunctionName,
                                    &[
                                        Operand2::Register(&value),
                                        Operand2::Register(&key),
                                        Operand2::U8(0),
                                    ],
                                );
                            }
                            self.emit2(
                                Opcode::DefineOwnPropertyByValue,
                                &[
                                    Operand2::Register(&value),
                                    Operand2::Register(&key),
                                    Operand2::Register(&class_register),
                                ],
                            );

                            self.register_allocator.dealloc(key);
                        }
                    }

                    self.register_allocator.dealloc(value);
                }
            }
        }

        self.register_allocator.dealloc(proto_register);

        if let Some(outer_scope) = outer_scope {
            self.pop_scope();
            self.lexical_scope = outer_scope;
            self.emit_opcode(Opcode::PopEnvironment);
        }

        self.emit_opcode(Opcode::PopPrivateEnvironment);

        self.push_from_register(&class_register);
        if !expression {
            self.emit_binding(BindingOpcode::InitVar, class_name);
        }

        self.register_allocator.dealloc(class_register);

        // NOTE: Reset strict mode to before class declaration/expression evalutation.
        self.code_block_flags.set(CodeBlockFlags::STRICT, strict);
    }
}
