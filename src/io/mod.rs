mod completion;
use std::collections::VecDeque;
use std::io;
use std::os::unix::io::AsRawFd;

use io_uring::squeue::Entry;
use io_uring::{IoUring, opcode, types::Fd};

use completion::{
    CompletionArena,
    Completion,    
}
;  // CompletionError, CompletionHandle, CompletionState, OwnerHandle, ReadOp};




pub trait Io {
    type Buf;                                   // handle de buffer alinhado do slab
    fn pread (&mut self, fd: Fd, off: u64, buf: Self::Buf, c: &mut CompletionHandler);
    fn pwrite(&mut self, fd: Fd, off: u64, buf: Self::Buf, c: &mut CompletionHandler);
    fn fsync (&mut self, fd: Fd, c: &mut CompletionHandler);
    fn step  (&mut self);                       // submete e colhe completions
}

struct WalWriter {}

pub enum Progress { Idle, Working, Done }

impl WalWriter {
    pub fn step<I: Io>(&mut self, io: &mut I) -> Progress { todo!() }
}