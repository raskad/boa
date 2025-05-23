use crate::Trace;

use super::{vtable_of, DropFn, GcHeader, RunFinalizerFn, TraceFn, VTable};

/// A garbage collected allocation.
#[derive(Debug)]
#[repr(C)]
pub struct GcBox<T: Trace + ?Sized + 'static> {
    pub(crate) header: GcHeader,
    pub(crate) vtable: &'static VTable,
    value: T,
}

impl<T: Trace> GcBox<T> {
    /// Returns a new `GcBox` with a rooted `GcBoxHeader`.
    pub(crate) fn new(value: T) -> Self {
        let vtable = vtable_of::<T>();
        Self {
            header: GcHeader::new(),
            vtable,
            value,
        }
    }
}

impl<T: Trace + ?Sized> GcBox<T> {
    /// Returns a reference to the `GcBox`'s value.
    pub(crate) const fn value(&self) -> &T {
        &self.value
    }

    /// Returns `true` if the header is marked.
    pub fn is_marked(&self) -> bool {
        self.header.is_marked()
    }

    pub fn trace_fn(&self) -> TraceFn {
        self.vtable.trace_fn()
    }

    pub(crate) fn run_finalizer_fn(&self) -> RunFinalizerFn {
        self.vtable.run_finalizer_fn()
    }

    pub(crate) fn drop_fn(&self) -> DropFn {
        self.vtable.drop_fn()
    }

    pub(crate) fn size(&self) -> usize {
        self.vtable.size()
    }
}
