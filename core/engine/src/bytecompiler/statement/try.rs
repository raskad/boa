use crate::{
    bytecompiler::{jump_control::JumpControlInfoFlags, ByteCompiler, Register, ToJsString},
    vm::{BindingOpcode, Opcode},
};
use boa_ast::{
    declaration::Binding,
    statement::{Block, Catch, Finally, Try},
    Statement, StatementListItem,
};

use super::Operand2;

impl ByteCompiler<'_> {
    /// Compile try statement.
    pub(crate) fn compile_try(&mut self, t: &Try, use_expr: bool) {
        let has_catch = t.catch().is_some();
        let has_finally = t.finally().is_some();

        let finally_re_throw = if t.finally().is_some() {
            let finally_re_throw = self.register_allocator.alloc();
            self.push_try_with_finally_control_info(&finally_re_throw, use_expr);
            Some(finally_re_throw)
        } else {
            None
        };

        let try_handler = self.push_handler();

        // Compile try block
        self.compile_block(t.block(), use_expr);

        if let Some(finally_re_throw) = &finally_re_throw {
            self.push_false(finally_re_throw);
        }

        let finally = self.jump();

        self.patch_handler(try_handler);

        // If it has a finally but no catch and we are in a generator, then we still need it
        // to handle `return()` call on generators.
        let catch_handler = if has_finally && (self.is_generator() || has_catch) {
            Some(self.push_handler())
        } else {
            None
        };

        let error = self.register_allocator.alloc();
        self.emit2(Opcode::Exception, &[Operand2::Register(&error)]);

        let mut re_throw_generator = None;

        if let Some(catch) = t.catch() {
            self.compile_catch_stmt(catch, &error, use_expr);
        } else {
            // Note: implicit !has_catch
            if self.is_generator() && has_finally {
                // Is this a generator `return()` empty exception?
                //
                // This is false because when the `Exception` opcode is executed,
                // it rethrows the empty exception, so if we reached this section,
                // that means it's not an `return()` generator exception.
                let value = self.register_allocator.alloc();
                self.push_false(&value);
                re_throw_generator = Some(value);
            }

            // Should we rethrow the exception?
            self.push_true(finally_re_throw.as_ref().expect("must exist"));
        }

        if let Some(finally_re_throw) = &finally_re_throw {
            if has_catch {
                self.push_false(finally_re_throw);
            }

            let exit = self.jump();

            if let Some(catch_handler) = catch_handler {
                self.patch_handler(catch_handler);
            }

            // Note: implicit has_finally
            if let Some(re_throw_generator) = &re_throw_generator {
                // Is this a generator `return()` empty exception?
                self.push_true(re_throw_generator);
            }

            // Should we rethrow the exception?
            self.push_true(&finally_re_throw);

            self.patch_jump(exit);
        }

        self.patch_jump(finally);

        let finally_start = self.next_opcode_location();
        if let Some(finally) = t.finally() {
            self.jump_info
                .last_mut()
                .expect("there should be a try block")
                .flags |= JumpControlInfoFlags::IN_FINALLY;

            // Compile finally statement body
            self.compile_finally_stmt(
                finally,
                has_catch,
                finally_re_throw.as_ref().expect("must exist"),
                re_throw_generator.as_ref(),
                &error,
            );
        }

        self.register_allocator.dealloc(error);

        if let Some(re_throw_generator) = re_throw_generator {
            self.register_allocator.dealloc(re_throw_generator);
        }

        if let Some(finally_re_throw) = finally_re_throw {
            self.pop_try_with_finally_control_info(finally_start);
            self.register_allocator.dealloc(finally_re_throw);
        }
    }

    pub(crate) fn compile_catch_stmt(&mut self, catch: &Catch, error: &Register, use_expr: bool) {
        let outer_scope = self.lexical_scope.clone();
        let scope_index = self.push_scope(catch.scope());
        self.emit_with_varying_operand(Opcode::PushScope, scope_index);

        if let Some(binding) = catch.parameter() {
            match binding {
                Binding::Identifier(ident) => {
                    let ident = ident.to_js_string(self.interner());
                    self.emit_binding(BindingOpcode::InitLexical, ident, &error);
                }
                Binding::Pattern(pattern) => {
                    self.compile_declaration_pattern(pattern, BindingOpcode::InitLexical, &error);
                }
            }
        }

        self.compile_catch_finally_block(catch.block(), use_expr);

        self.pop_scope();
        self.lexical_scope = outer_scope;
        self.emit_opcode(Opcode::PopEnvironment);
    }

    pub(crate) fn compile_finally_stmt(
        &mut self,
        finally: &Finally,
        has_catch: bool,
        re_throw: &Register,
        re_throw_generator: Option<&Register>,
        error: &Register,
    ) {
        // TODO: We could probably remove the Get/SetAccumulatorFromStack if we check that there is no break/continues statements.
        let value = self.register_allocator.alloc();
        self.emit2(
            Opcode::SetRegisterFromAccumulator,
            &[Operand2::Register(&value)],
        );
        self.compile_catch_finally_block(finally.block(), false);
        self.emit2(Opcode::SetAccumulator, &[Operand2::Register(&value)]);
        self.register_allocator.dealloc(value);

        // Rethrow error if error happened!
        let do_not_throw_exit = self.jump_if_false(&re_throw);

        if has_catch {
            self.emit_opcode(Opcode::ReThrow);
        } else if self.is_generator() {
            if let Some(re_throw_generator) = re_throw_generator {
                let is_generator_exit = self.jump_if_true(re_throw_generator);
                self.emit2(Opcode::Throw, &[Operand2::Register(&error)]);
                self.patch_jump(is_generator_exit);
            }

            self.emit_opcode(Opcode::ReThrow);
        } else {
            self.emit2(Opcode::Throw, &[Operand2::Register(&error)]);
        }

        self.patch_jump(do_not_throw_exit);
    }

    /// Compile a catch or finally block.
    ///
    /// If the block contains a break or continue as the first statement,
    /// the return value is set to undefined.
    /// See the [ECMAScript reference][spec] for more information.
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-try-statement-runtime-semantics-evaluation
    fn compile_catch_finally_block(&mut self, block: &Block, use_expr: bool) {
        let mut b = block;

        loop {
            match b.statement_list().first() {
                Some(StatementListItem::Statement(
                    Statement::Break(_) | Statement::Continue(_),
                )) => {
                    let value = self.register_allocator.alloc();
                    self.push_undefined(&value);
                    self.emit2(Opcode::SetAccumulator, &[Operand2::Register(&value)]);
                    self.register_allocator.dealloc(value);
                    break;
                }
                Some(StatementListItem::Statement(Statement::Block(block))) => {
                    b = block;
                }
                _ => {
                    break;
                }
            }
        }

        self.compile_block(block, use_expr);
    }
}
