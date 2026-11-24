use super::{Buf, FileId};

/// Fixed set of completion slots, allocated once at startup. Its capacity is
/// the limit on operations in flight.
#[derive(Debug)]
pub struct CompletionArena {
    slots: Vec<Slot>,
    /// First free slot. `None` means the arena is full.
    free_head: Option<u32>,
}

/// Owned handle to one in-flight operation.
///
/// No `Clone` or `Copy`: the calls that finish an operation take it by value,
/// so a slot cannot be used after it has been released.
#[derive(Debug)]
#[must_use]
pub struct CompletionHandle {
    idx: u32,
}

/// What the kernel was asked to do. Holds everything that must stay alive
/// until the completion arrives.
#[derive(Debug)]
pub enum Op {
    Read { fd: FileId, offset: u64, buf: Buf },
    Write { fd: FileId, offset: u64, buf: Buf },
    Fsync { fd: FileId },
}

/// What the owner gets back from a finished operation.
#[derive(Debug)]
pub struct Output {
    pub op: Op,
    /// Raw result from the kernel: bytes transferred, or `-errno`.
    pub result: i32,
}

#[derive(Debug)]
enum Slot {
    /// Unused. `next` links the free slots into an intrusive list.
    Free { next: Option<u32> },
    /// Submitted; the kernel may be using the buffer.
    InFlight(Op),
    /// Completed; waiting for the owner to take the result.
    Done { op: Op, result: i32 },
    /// The owner gave up, but the kernel has not answered yet. Keeps the
    /// buffer alive until the completion arrives.
    Abandoned(Op),
}

// TODO:
// impl CompletionArena {
//     fn new(capacity: u32) -> Self;
//     fn alloc(&mut self, op: Op) -> Result<CompletionHandle, Op>;      // Free -> InFlight
//     fn complete(&mut self, idx: u32, result: i32) -> Option<Op>;      // InFlight -> Done, Abandoned -> Free
//     fn try_take(&mut self, h: CompletionHandle)
//         -> Result<Output, CompletionHandle>;                          // Done -> Free
//     fn abandon(&mut self, h: CompletionHandle) -> Option<Op>;         // InFlight -> Abandoned, Done -> Free
// }
