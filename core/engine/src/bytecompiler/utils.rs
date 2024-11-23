use crate::{
    js_string,
    vm::{GeneratorResumeKind, Opcode},
};

use super::{ByteCompiler, Literal, Operand, Operand2, Register};

impl ByteCompiler<'_> {
    /// Closes an iterator
    ///
    /// This is equivalent to the [`IteratorClose`][iter] and [`AsyncIteratorClose`][async]
    /// operations.
    ///
    /// Iterator Stack:
    ///  - iterator **=>** \<empty\>
    ///
    /// [iter]: https://tc39.es/ecma262/#sec-iteratorclose
    /// [async]: https://tc39.es/ecma262/#sec-asynciteratorclose
    pub(super) fn iterator_close(&mut self, async_: bool) {
        let value = self.register_allocator.alloc();
        let called = self.register_allocator.alloc();

        self.emit2(
            Opcode::IteratorReturn,
            &[Operand2::Register(&value), Operand2::Register(&called)],
        );

        // `iterator` didn't have a `return` method, is already done or is not on the iterator stack.
        let early_exit = self.jump_if_false(&called);
        self.register_allocator.dealloc(called);

        if async_ {
            self.push_from_register(&value);
            self.emit_opcode(Opcode::Await);
            let resume_kind = self.register_allocator.alloc();
            self.pop_into_register(&resume_kind);
            self.pop_into_register(&value);
            self.emit2(
                Opcode::GeneratorNext,
                &[Operand2::Register(&resume_kind), Operand2::Register(&value)],
            );
            self.register_allocator.dealloc(resume_kind);
        }

        self.emit2(Opcode::IsObject, &[Operand2::Register(&value)]);
        let skip_throw = self.jump_if_true(&value);

        self.register_allocator.dealloc(value);

        let error_msg = self.get_or_insert_literal(Literal::String(js_string!(
            "inner result was not an object"
        )));
        self.emit_with_varying_operand(Opcode::ThrowNewTypeError, error_msg);

        self.patch_jump(skip_throw);
        self.patch_jump(early_exit);
    }

    /// Closes all active iterators in the current [`CallFrame`][crate::vm::CallFrame].
    pub(super) fn close_active_iterators(&mut self) {
        let start = self.next_opcode_location();

        let empty = self.register_allocator.alloc();
        self.emit2(Opcode::IteratorStackEmpty, &[Operand2::Register(&empty)]);
        let exit = self.jump_if_true(&empty);
        self.register_allocator.dealloc(empty);

        self.iterator_close(self.is_async_generator());
        self.emit(Opcode::Jump, &[Operand::U32(start)]);
        self.patch_jump(exit);
    }

    /// Yields from the current generator.
    ///
    /// This is equivalent to the [`Yield ( value )`][yield] operation from the spec.
    ///
    /// stack:
    /// - value **=>** received
    ///
    /// [yield]: https://tc39.es/ecma262/#sec-yield
    pub(super) fn r#yield(&mut self, value: &Register) {
        let resume_kind = self.register_allocator.alloc();

        // 1. Let generatorKind be GetGeneratorKind().
        if self.is_async() {
            // 2. If generatorKind is async, return ? AsyncGeneratorYield(? Await(value)).
            self.push_from_register(value);
            self.emit_opcode(Opcode::Await);
            self.pop_into_register(&resume_kind);
            self.pop_into_register(value);
            self.emit2(
                Opcode::GeneratorNext,
                &[Operand2::Register(&resume_kind), Operand2::Register(value)],
            );
            self.async_generator_yield(value, &resume_kind);
        } else {
            // 3. Otherwise, return ? GeneratorYield(CreateIterResultObject(value, false)).
            self.emit2(
                Opcode::CreateIteratorResult,
                &[Operand2::Register(value), Operand2::Bool(false)],
            );
            self.push_from_register(value);
            self.emit_opcode(Opcode::GeneratorYield);
            self.pop_into_register(&resume_kind);
            self.pop_into_register(value);
        }

        self.emit2(
            Opcode::GeneratorNext,
            &[Operand2::Register(&resume_kind), Operand2::Register(value)],
        );
        self.register_allocator.dealloc(resume_kind);
    }

    /// Yields from the current async generator.
    ///
    /// This is equivalent to the [`AsyncGeneratorYield ( value )`][async_yield] operation from the spec.
    ///
    /// stack:
    /// - value **=>** `resume_kind`, received
    ///
    /// [async_yield]: https://tc39.es/ecma262/#sec-asyncgeneratoryield
    pub(super) fn async_generator_yield(&mut self, value: &Register, resume_kind: &Register) {
        self.push_from_register(value);
        self.emit_opcode(Opcode::AsyncGeneratorYield);
        self.pop_into_register(resume_kind);
        self.pop_into_register(value);

        let non_return_resume =
            self.jump_if_not_resume_kind(GeneratorResumeKind::Return, resume_kind);

        self.push_from_register(value);
        self.emit_opcode(Opcode::Await);
        self.pop_into_register(resume_kind);
        self.pop_into_register(value);

        let non_normal_resume =
            self.jump_if_not_resume_kind(GeneratorResumeKind::Normal, resume_kind);

        self.emit_resume_kind(GeneratorResumeKind::Return, resume_kind);

        self.patch_jump(non_normal_resume);
        self.patch_jump(non_return_resume);
    }
}
