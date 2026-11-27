use crate::io::Io;

/// What a state machine reports after one `step()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    Idle,
    Working,
    Done,
}

#[derive(Debug)]
pub struct WalWriter {}

impl WalWriter {
    pub fn step<I: Io>(&mut self, io: &mut I) -> Progress {
        todo!()
    }
}
