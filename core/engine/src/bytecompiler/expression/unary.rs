use boa_ast::{
    expression::operator::{unary::UnaryOp, Unary},
    Expression,
};

use crate::{
    bytecompiler::{Access, ByteCompiler, Operand2, ToJsString},
    vm::Opcode,
};

impl ByteCompiler<'_> {
    pub(crate) fn compile_unary(&mut self, unary: &Unary, use_expr: bool) {
        let dst = self.register_allocator.alloc();

        let opcode = match unary.op() {
            UnaryOp::Delete => {
                if let Some(access) = Access::from_expression(unary.target()) {
                    self.access_delete(access, &dst);
                } else {
                    self.compile_expr(unary.target(), false);
                    self.emit_opcode(Opcode::PushTrue);
                    self.pop_into_register(&dst);
                }
                None
            }
            UnaryOp::Minus => Some(Opcode::Neg),
            UnaryOp::Plus => Some(Opcode::Pos),
            UnaryOp::Not => Some(Opcode::LogicalNot),
            UnaryOp::Tilde => Some(Opcode::BitNot),
            UnaryOp::TypeOf => {
                match unary.target().flatten() {
                    Expression::Identifier(identifier) => {
                        let identifier = identifier.to_js_string(self.interner());
                        let binding = self.lexical_scope.get_identifier_reference(identifier);
                        let index = self.get_or_insert_binding(binding);
                        self.emit_binding_access(Opcode::GetNameOrUndefined, &index);
                        self.pop_into_register(&dst);
                    }
                    expr => self.compile_expr2(expr, &dst),
                }
                self.emit2(Opcode::TypeOf, &[Operand2::Register(&dst)]);
                None
            }
            UnaryOp::Void => {
                self.compile_expr2(unary.target(), &dst);
                self.emit_opcode(Opcode::PushUndefined);
                self.pop_into_register(&dst);
                None
            }
        };

        if let Some(opcode) = opcode {
            self.compile_expr2(unary.target(), &dst);
            self.emit2(opcode, &[Operand2::Register(&dst)]);
        }

        if use_expr {
            self.push_from_register(&dst);
        }

        self.register_allocator.dealloc(dst);
    }
}
