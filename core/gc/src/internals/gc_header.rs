use core::{cell::Cell, fmt};

const MARK_MASK: u32 = 1 << (u32::BITS - 1);

/// The `GcHeader` contains the `GcBox`'s and `EphemeronBox`'s current state for the `Collector`'s
/// Mark/Sweep as well as a pointer to the next node in the heap.
pub(crate) struct GcHeader {
    mark: Cell<u32>,
}

impl GcHeader {
    /// Creates a new [`GcHeader`] with mark bit unset.
    pub(crate) fn new() -> Self {
        Self {
            mark: Cell::new(0),
        }
    }

    /// Returns a bool for whether [`GcHeader`]'s mark bit is 1.
    pub(crate) fn is_marked(&self) -> bool {
        self.mark.get() & MARK_MASK != 0
    }

    /// Sets [`GcHeader`]'s mark bit to 1.
    pub(crate) fn mark(&self) {
        self.mark.set(self.mark.get() | MARK_MASK);
    }

    /// Sets [`GcHeader`]'s mark bit to 0.
    pub(crate) fn unmark(&self) {
        self.mark.set(self.mark.get() & !MARK_MASK);
    }
}

impl fmt::Debug for GcHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GcHeader")
            .field("marked", &self.is_marked())
            .finish()
    }
}
