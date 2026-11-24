use std::ptr::NonNull;

/// Fixed set of aligned blocks, allocated once at startup.
#[derive(Debug)]
pub struct BufferPool {
    /// One allocation of `n_blocks * BLOCK_SIZE` bytes, aligned to 4096.
    base: NonNull<u8>,
    /// Indices of free blocks. Preallocated with `n_blocks`, never grows.
    free: Vec<u32>,
}

/// Owned handle to one block of the pool.
///
/// No `Clone` or `Copy`: a block has exactly one owner, and it goes back to
/// the pool through `BufferPool::release`.
#[derive(Debug)]
pub struct Buf {
    ptr: NonNull<u8>,
    /// Which block this is, so it can be returned to the pool.
    idx: u32,
    /// Valid bytes, `<= BLOCK_SIZE`.
    len: u32,
}

// TODO:
// impl BufferPool {
//     fn new(n_blocks: u32) -> Self;
//     fn acquire(&mut self) -> Option<Buf>;   // None = backpressure
//     fn release(&mut self, buf: Buf);        // consumes the handle
// }
