use std::fs::File;


#[derive(Debug)]
pub struct CompletionArena {
    slots: Vec<Slot>,
    free_list: Vec<u32>,
}

#[derive(Debug)]
struct FsyncOp {
    file: File,
}

#[derive(Debug)]
struct PreadOp {
    file: File,
    buf: Vec<u8>, // Maybe a better type for this
    offset: u64,
}

#[derive(Debug)]
struct PwriteOp {
    file: File,
    buf: Vec<u8>, // Maybe a better type for this
    offset: u64,
}


#[derive(Debug)]
enum Op{
    pread(PreadOp),
    pwrite(PwriteOp),
    fsync(FsyncOp),
}

#[derive(Debug, PartialEq, Eq)]
enum SloteState {
    Free,
    InFlight,
    Completed(i32),
    Aborted,
}
#[derive(Debug)] // No copy or clone
struct OwnerHandle {
    id: u32,
}

// fn try_take(&mut self, h: CompletionHandle)
//     -> Result<Output, CompletionHandle>;                                // pronto: consome; senão devolve


#[derive(Debug)]
enum CompletionState {
    Free,
    InFlight,
    Completed(i32),
    Aborted,
}

#[derive(Debug)]
struct Slot {
    state: CompletionState,
    owner: Option<OwnerHandle>,
    op: Option<Op>
}

