mod assign;
mod binary;
mod object_literal;
mod unary;
mod update;

use std::ops::Deref;

use super::{Access, Callable, InstructionOperand, NodeKind, Operand, Operand2, ToJsString};
use crate::{
    bytecompiler::{ByteCompiler, Literal},
    vm::{GeneratorResumeKind, Opcode},
};
use boa_ast::{
    expression::{
        access::{PropertyAccess, PropertyAccessField},
        literal::{Literal as AstLiteral, TemplateElement, TemplateLiteral},
        operator::Conditional,
        Identifier,
    },
    Expression,
};

impl ByteCompiler<'_> {
    fn compile_literal(&mut self, lit: &AstLiteral, use_expr: bool) {
        match lit {
            AstLiteral::String(v) => {
                self.emit_push_literal(Literal::String(v.to_js_string(self.interner())));
            }
            AstLiteral::Int(v) => self.emit_push_integer(*v),
            AstLiteral::Num(v) => self.emit_push_rational(*v),
            AstLiteral::BigInt(v) => {
                self.emit_push_literal(Literal::BigInt(v.clone().into()));
            }
            AstLiteral::Bool(true) => self.emit(Opcode::PushTrue, &[]),
            AstLiteral::Bool(false) => self.emit(Opcode::PushFalse, &[]),
            AstLiteral::Null => self.emit(Opcode::PushNull, &[]),
            AstLiteral::Undefined => self.emit(Opcode::PushUndefined, &[]),
        }

        if !use_expr {
            self.emit_opcode(Opcode::Pop);
        }
    }

    fn compile_conditional(&mut self, op: &Conditional, use_expr: bool) {
        let dst = self.register_allocator.alloc();
        self.compile_expr2(op.condition(), &dst);
        let jelse = self.jump_if_false(&dst);
        self.compile_expr2(op.if_true(), &dst);
        let exit = self.jump();
        self.patch_jump(jelse);
        self.compile_expr2(op.if_false(), &dst);
        self.patch_jump(exit);

        if use_expr {
            self.push_from_register(&dst);
        };

        self.register_allocator.dealloc(dst);
    }

    fn compile_template_literal(&mut self, template_literal: &TemplateLiteral, use_expr: bool) {
        for element in template_literal.elements() {
            match element {
                TemplateElement::String(s) => {
                    self.emit_push_literal(Literal::String(s.to_js_string(self.interner())));
                }
                TemplateElement::Expr(expr) => {
                    self.compile_expr(expr, true);
                }
            }
        }

        self.emit_with_varying_operand(
            Opcode::ConcatToString,
            template_literal.elements().len() as u32,
        );

        if !use_expr {
            self.emit_opcode(Opcode::Pop);
        }
    }

    pub(crate) fn compile_expr_impl(&mut self, expr: &Expression, use_expr: bool) {
        match expr {
            Expression::Literal(lit) => self.compile_literal(lit, use_expr),
            Expression::RegExpLiteral(regexp) => {
                let pattern_index = self.get_or_insert_name(Identifier::new(regexp.pattern()));
                let flags_index = self.get_or_insert_name(Identifier::new(regexp.flags()));
                self.emit(
                    Opcode::PushRegExp,
                    &[
                        Operand::Varying(pattern_index),
                        Operand::Varying(flags_index),
                    ],
                );
            }
            Expression::Unary(unary) => self.compile_unary(unary, use_expr),
            Expression::Update(update) => self.compile_update(update, use_expr),
            Expression::Binary(binary) => {
                let reg = self.register_allocator.alloc();
                self.compile_binary(binary, &reg);
                if use_expr {
                    self.push_from_register(&reg);
                }
                self.register_allocator.dealloc(reg);
            }
            Expression::BinaryInPrivate(binary) => {
                let reg = self.register_allocator.alloc();
                self.compile_binary_in_private(binary, &reg);
                if use_expr {
                    self.push_from_register(&reg);
                }
                self.register_allocator.dealloc(reg);
            }
            Expression::Assign(assign) => self.compile_assign(assign, use_expr),
            Expression::ObjectLiteral(object) => {
                self.compile_object_literal(object, use_expr);
            }
            Expression::Identifier(name) => {
                self.access_get(Access::Variable { name: *name }, use_expr);
            }
            Expression::PropertyAccess(access) => {
                self.access_get(Access::Property { access }, use_expr);
            }
            Expression::Conditional(op) => self.compile_conditional(op, use_expr),
            Expression::ArrayLiteral(literal) => {
                let array = self.register_allocator.alloc();
                let value = self.register_allocator.alloc();

                self.emit2(Opcode::PushNewArray, &[Operand2::Register(&array)]);

                for element in literal.as_ref() {
                    if let Some(element) = element {
                        self.compile_expr2(element, &value);
                        if let Expression::Spread(_) = element {
                            self.emit2(Opcode::GetIterator, &[Operand2::Register(&value)]);
                            self.emit2(Opcode::PushIteratorToArray, &[Operand2::Register(&array)]);
                        } else {
                            self.emit2(
                                Opcode::PushValueToArray,
                                &[Operand2::Register(&value), Operand2::Register(&array)],
                            );
                        }
                    } else {
                        self.emit2(Opcode::PushElisionToArray, &[Operand2::Register(&array)]);
                    }
                }

                if use_expr {
                    self.push_from_register(&array);
                }

                self.register_allocator.dealloc(array);
                self.register_allocator.dealloc(value);
            }
            Expression::This => {
                self.access_get(Access::This, use_expr);
            }
            Expression::Spread(spread) => self.compile_expr(spread.target(), true),
            Expression::FunctionExpression(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::ArrowFunction(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::AsyncArrowFunction(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::GeneratorExpression(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::AsyncFunctionExpression(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::AsyncGeneratorExpression(function) => {
                self.function_with_binding(function.into(), NodeKind::Expression, use_expr);
            }
            Expression::Call(call) => self.call(Callable::Call(call), use_expr),
            Expression::New(new) => self.call(Callable::New(new), use_expr),
            Expression::TemplateLiteral(template_literal) => {
                self.compile_template_literal(template_literal, use_expr);
            }
            Expression::Await(expr) => {
                self.compile_expr(expr.target(), true);
                self.emit_opcode(Opcode::Await);
                self.emit_opcode(Opcode::GeneratorNext);
                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            Expression::Yield(r#yield) => {
                // stack:
                if let Some(expr) = r#yield.target() {
                    self.compile_expr(expr, true);
                } else {
                    self.emit_opcode(Opcode::PushUndefined);
                }

                // stack: value

                if r#yield.delegate() {
                    let object = self.register_allocator.alloc();
                    self.pop_into_register(&object);

                    if self.is_async() {
                        self.emit2(Opcode::GetAsyncIterator, &[Operand2::Register(&object)]);
                    } else {
                        self.emit2(Opcode::GetIterator, &[Operand2::Register(&object)]);
                    }

                    self.register_allocator.dealloc(object);

                    // stack:
                    self.emit_opcode(Opcode::PushUndefined);

                    // stack: undefined
                    self.emit_resume_kind(GeneratorResumeKind::Normal);

                    // stack: resume_kind, undefined
                    let start_address = self.next_opcode_location();
                    let (throw_method_undefined, return_method_undefined) =
                        self.emit_opcode_with_two_operands(Opcode::GeneratorDelegateNext);

                    if self.is_async() {
                        self.emit_opcode(Opcode::Pop);
                        self.emit_opcode(Opcode::Await);
                    }

                    let (return_gen, exit) =
                        self.emit_opcode_with_two_operands(Opcode::GeneratorDelegateResume);
                    if self.is_async() {
                        self.emit_opcode(Opcode::IteratorValue);
                        self.async_generator_yield();
                    } else {
                        self.emit_opcode(Opcode::IteratorResult);
                        self.emit_opcode(Opcode::GeneratorYield);
                    }
                    self.emit(Opcode::Jump, &[Operand::U32(start_address)]);

                    self.patch_jump(return_gen);
                    self.patch_jump(return_method_undefined);
                    if self.is_async() {
                        self.emit_opcode(Opcode::Await);
                        self.emit_opcode(Opcode::Pop);
                    }
                    self.close_active_iterators();

                    self.r#return(true);

                    self.patch_jump(throw_method_undefined);
                    self.iterator_close(self.is_async());
                    self.emit_opcode(Opcode::Throw);

                    self.patch_jump(exit);
                } else {
                    self.r#yield();
                }

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            Expression::TaggedTemplate(template) => {
                let this = self.register_allocator.alloc();
                let function = self.register_allocator.alloc();

                match template.tag() {
                    Expression::PropertyAccess(PropertyAccess::Simple(access)) => {
                        self.compile_expr2(access.target(), &this);
                        match access.field() {
                            PropertyAccessField::Const(ident) => {
                                self.emit_get_property_by_name(&function, &this, &this, *ident);
                            }
                            PropertyAccessField::Expr(field) => {
                                let key = self.register_allocator.alloc();
                                self.compile_expr2(field, &key);
                                self.emit2(
                                    Opcode::GetPropertyByValue,
                                    &[
                                        Operand2::Register(&function),
                                        Operand2::Operand(InstructionOperand::Register(&key)),
                                        Operand2::Operand(InstructionOperand::Register(&this)),
                                        Operand2::Operand(InstructionOperand::Register(&this)),
                                    ],
                                );
                                self.register_allocator.dealloc(key);
                            }
                        }
                    }
                    Expression::PropertyAccess(PropertyAccess::Private(access)) => {
                        let index = self.get_or_insert_private_name(access.field());
                        self.compile_expr2(access.target(), &this);
                        self.emit2(
                            Opcode::GetPrivateField,
                            &[
                                Operand2::Register(&function),
                                Operand2::Operand(InstructionOperand::Register(&this)),
                                Operand2::Varying(index),
                            ],
                        );
                    }
                    expr => {
                        self.emit_opcode(Opcode::PushUndefined);
                        self.pop_into_register(&this);
                        self.compile_expr2(expr, &function);
                    }
                }

                self.push_from_register(&this);
                self.push_from_register(&function);

                self.register_allocator.dealloc(this);
                self.register_allocator.dealloc(function);

                let site = template.identifier();
                let count = template.cookeds().len() as u32;

                let jump_label = self.emit_opcode_with_operand(Opcode::TemplateLookup);
                self.emit_u64(site);

                for (cooked, raw) in template.cookeds().iter().zip(template.raws()) {
                    if let Some(cooked) = cooked {
                        self.emit_push_literal(Literal::String(
                            cooked.to_js_string(self.interner()),
                        ));
                    } else {
                        self.emit_opcode(Opcode::PushUndefined);
                    }
                    self.emit_push_literal(Literal::String(raw.to_js_string(self.interner())));
                }

                self.emit(
                    Opcode::TemplateCreate,
                    &[Operand::Varying(count), Operand::U64(site)],
                );

                self.patch_jump(jump_label);

                for expr in template.exprs() {
                    self.compile_expr(expr, true);
                }

                self.emit_with_varying_operand(Opcode::Call, template.exprs().len() as u32 + 1);
            }
            Expression::ClassExpression(class) => self.class(class.deref().into(), true),
            Expression::SuperCall(super_call) => {
                self.emit_opcode(Opcode::SuperCallPrepare);

                let contains_spread = super_call
                    .arguments()
                    .iter()
                    .any(|arg| matches!(arg, Expression::Spread(_)));

                if contains_spread {
                    let array = self.register_allocator.alloc();
                    let value = self.register_allocator.alloc();

                    self.emit2(Opcode::PushNewArray, &[Operand2::Register(&array)]);

                    for arg in super_call.arguments() {
                        self.compile_expr2(arg, &value);
                        if let Expression::Spread(_) = arg {
                            self.emit2(Opcode::GetIterator, &[Operand2::Register(&value)]);
                            self.emit2(Opcode::PushIteratorToArray, &[Operand2::Register(&array)]);
                        } else {
                            self.emit2(
                                Opcode::PushValueToArray,
                                &[Operand2::Register(&value), Operand2::Register(&array)],
                            );
                        }
                    }

                    self.push_from_register(&array);

                    self.register_allocator.dealloc(value);
                    self.register_allocator.dealloc(array);
                } else {
                    for arg in super_call.arguments() {
                        self.compile_expr(arg, true);
                    }
                }

                if contains_spread {
                    self.emit_opcode(Opcode::SuperCallSpread);
                } else {
                    self.emit_with_varying_operand(
                        Opcode::SuperCall,
                        super_call.arguments().len() as u32,
                    );
                }
                self.emit_opcode(Opcode::BindThisValue);

                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            Expression::ImportCall(import) => {
                self.compile_expr(import.argument(), true);
                self.emit_opcode(Opcode::ImportCall);
                if !use_expr {
                    self.emit_opcode(Opcode::Pop);
                }
            }
            Expression::NewTarget => {
                if use_expr {
                    self.emit_opcode(Opcode::NewTarget);
                }
            }
            Expression::ImportMeta => {
                if use_expr {
                    self.emit_opcode(Opcode::ImportMeta);
                }
            }
            Expression::Optional(opt) => {
                let this = self.register_allocator.alloc();
                let dst = self.register_allocator.alloc();
                self.compile_optional_preserve_this(opt, &this, &dst);

                if use_expr {
                    self.push_from_register(&dst);
                }

                self.register_allocator.dealloc(this);
                self.register_allocator.dealloc(dst);
            }
            Expression::Parenthesized(parenthesized) => {
                self.compile_expr(parenthesized.expression(), use_expr);
            }
            // TODO: try to remove this variant somehow
            Expression::FormalParameterList(_) => unreachable!(),
            Expression::Debugger => (),
        }
    }
}
