use super::{Buf, FileId};


/// Fixed set of completion slots, allocated once at startup. Its capacity is
/// the limit on operations in flight.
#[derive(Debug)]
pub(crate) struct CompletionArena {
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
    /// Has owner; the kernel may be using the buffer.
    Pending(Op),
    /// Completed; waiting for the owner to take the result.
    Done { op: Op, result: i32 },
    /// The owner gave up, but the kernel has not answered yet. Keeps the
    /// buffer alive until the completion arrives.
    Abandoned(Op),
}

// TODO:
// impl CompletionArena {
//     fn new(capacity: u32) -> Self;
//     fn alloc(&mut self, op: Op) -> Result<CompletionHandle, Op>;      // Free -> Pending
//     fn complete(&mut self, idx: u32, result: i32) -> Option<Op>;      // Pending -> Done, Abandoned -> Free
//     fn try_take(&mut self, h: CompletionHandle)
//         -> Result<Output, CompletionHandle>;                          // Done -> Free
//     fn abandon(&mut self, h: CompletionHandle) -> Option<Op>;         // Pending -> Abandoned, Done -> Free
// }
impl CompletionArena {
    pub(crate) fn new(capacity: u32) -> Self {
        assert!(capacity > 0);
        let  slots: Vec<Slot> = (0..capacity)
            .map(|i| Slot::Free { next: (i + 1 < capacity).then_some(i + 1) }).collect();
        Self {slots, free_head: Some(0)}
    }
    pub(crate) fn alloc(&mut self, op: Op) -> Result<CompletionHandle, Op> {
        let Some(idx) = self.free_head else {return Err(op)};

        let slot = &mut self.slots[idx as usize];
        let Slot::Free{next} = *slot else {unreachable!("free_head points to a slot in use")}; 
        self.free_head = next;
        *slot = Slot::Pending(op);

        Ok(CompletionHandle { idx })
    }
    // Called by the IoEngine when the kernel has completed an operation
    pub(crate) fn complete(&mut self, idx: u32, result: i32) -> Option<Op> {
        let slot = &mut self.slots[idx as usize];

        match std::mem::replace(slot, Slot::Free { next: self.free_head }) {
            Slot::Pending(op) => {
                *slot = Slot::Done { op, result };
                None
            },
            Slot::Abandoned(op) => {
                self.free_head = Some(idx);
                Some(op)
            }
            other => unreachable!("kernel completed a slot that was not in flight. Slot state: {:?}", other),
        }
    }

    pub(crate) fn try_take(&mut self, handle: CompletionHandle) -> Result<
    


}