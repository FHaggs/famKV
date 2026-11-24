pub mod buf;
pub mod completion;

pub use buf::{Buf, BufferPool};
pub use completion::{CompletionArena, CompletionHandle, Output};

/// Identifies an open file. An index into the file table owned by the `Io`
/// implementation, not a raw fd, so a simulated backend can hand them out too.
///
/// `Copy` on purpose: several operations may target the same file at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileId(u32);

pub trait Io {
    /// On a full arena the buffer is handed back in `Err`.
    fn read(&mut self, fd: FileId, offset: u64, buf: Buf) -> Result<CompletionHandle, Buf>;
    /// On a full arena the buffer is handed back in `Err`.
    fn write(&mut self, fd: FileId, offset: u64, buf: Buf) -> Result<CompletionHandle, Buf>;
    /// `None` when the arena is full.
    fn fsync(&mut self, fd: FileId) -> Option<CompletionHandle>;
    /// Consumes the handle when the operation is done; hands it back otherwise.
    fn try_take(&mut self, h: CompletionHandle) -> Result<Output, CompletionHandle>;
    /// Submits queued operations and reaps completions.
    fn step(&mut self);
}
