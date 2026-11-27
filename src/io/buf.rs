use std::ops::{Deref, DerefMut};

pub const BLOCK_SIZE: usize = 4096;

#[derive(Clone, Debug)]
#[repr(C, align(4096))]
struct Block([u8; BLOCK_SIZE]);

/// Owned handle to one block of the pool.
///
/// # Leaks
///
/// A `Buf` goes back to the pool only through [`BufferPool::release`].
/// Dropping one without releasing it is safe, but the block is lost for the
/// rest of the program: the pool never gets it back. Nothing checks this,
/// neither the compiler nor a runtime assertion, so returning every `Buf` is
/// the caller's responsibility.
#[derive(Debug)]
pub struct Buf(&'static mut Block);

impl Deref for Buf {
    type Target = [u8];
    fn deref(&self) -> &[u8] { &self.0.0 }
}
impl DerefMut for Buf {
    fn deref_mut(&mut self) -> &mut [u8] { &mut self.0.0 }
}

pub struct BufferPool {
    free: Vec<Buf>,
}

impl BufferPool {
    pub fn new(n_blocks: usize) -> Self {
        let blocks = vec![Block([0; BLOCK_SIZE]); n_blocks].into_boxed_slice();
        let blocks: &'static mut [Block] = Box::leak(blocks);
        Self { free: blocks.iter_mut().map(Buf).collect() }
    }

    pub fn acquire(&mut self) -> Option<Buf> { self.free.pop() }

    pub fn release(&mut self, buf: Buf) { self.free.push(buf); }
}