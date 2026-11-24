# famKV

A key-value store written in Rust, built to learn how storage engines work.

This is a learning project. It is not finished, not fast yet, and not something
you should put data in. The goal is to understand the ideas by implementing
them, including ideas from papers that look interesting along the way.

## The plan

Three pieces, built in this order:

1. **An explicit IO layer on io_uring.** No async runtime. Every read, write
   and fsync is submitted as an operation and collected later as a completion.
   The components above it are state machines that advance through a `step()`
   call.
2. **An LSM tree on top of that IO layer.** Write-ahead log, memtable, sorted
   files on disk, compaction.
3. **MVCC, later.** Multiple versions per key so readers get a consistent
   snapshot without blocking writers.

## Design rules

These are the constraints the code is written against:

- **No allocation on the hot path.** Completion slots and IO buffers come from
  pools sized at startup. When a pool is empty, the caller gets backpressure
  instead of a new allocation.
- **Let the compiler enforce correct use.** Completion handles and buffers
  cannot be copied or cloned, and the calls that finish an operation take them
  by value. Using a handle twice is a compile error, not a runtime bug.
- **IO behind a trait.** The engine talks to an `Io` trait, not to io_uring
  directly, so the same code can run against a simulated disk that injects
  failures.
- **Buffers stay put.** The kernel holds a pointer to a buffer while an
  operation is in flight, so buffers are fixed, aligned blocks owned by the
  operation until its completion arrives.

## Status

Early. The IO layer is being designed and the project does not build yet.

- [ ] Buffer pool
- [ ] Completion arena and handles
- [ ] io_uring backend (read, write, fsync)
- [ ] Simulated IO backend for tests (DST)
- [ ] Write-ahead log and recovery
- [ ] Memtable
- [ ] On-disk sorted files
- [ ] Compaction
- [ ] MVCC

## Layout

```
src/
  main.rs
  io/
    mod.rs          the Io trait
    completion.rs   completion slots and operations
```

This will grow as the pieces above land.

## Requirements

- Linux with io_uring support
- A recent Rust toolchain (the crate uses the 2024 edition)

```
cargo build
cargo test
```

## Background reading

- O'Neil, Cheng, Gawlick, O'Neil. *The Log-Structured Merge-Tree (LSM-Tree)*, 1996.
- Jens Axboe. *Efficient IO with io_uring*.
