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

## Limits and backpressure

Not allocating at runtime means everything has a fixed limit. The number of IO
operations in flight is the size of the completion arena. The number of buffers
in use is the size of the buffer pool. What happens at a limit depends on who
hit it:

- **Client requests are rejected at the edge.** A new `put` or `get` that
  cannot be admitted is refused before it consumes anything. Queueing it would
  only hide the overload.
- **Internal IO is never dropped.** A WAL write or a compaction read that
  cannot get a slot stays in its current state and tries again on the next
  `step()`.
- **Each component has a fixed budget.** The WAL may have at most N operations
  in flight, compaction K, reads R, and the arena holds N + K + R slots. A
  component that stays inside its budget always finds a free slot, so a full
  arena is a bug to assert on, and compaction cannot starve the WAL.

Buffers follow the same rules. An operation that gets a buffer but no slot (or
the reverse) gives back what it took.

## The LSM

The starting point is a classic leveled LSM tree, with three decisions made up
front so it fits the IO model:

1. **Fixed-size blocks and bounded table sizes.** One block size matches the
   buffer pool, and bounded tables make the IO a compaction needs predictable.
2. **Internal keys carry a sequence number from day one**: `(user_key, seq,
   kind)`. MVCC then becomes keeping old versions and reading at a snapshot,
   not a format change.
3. **Compaction is incremental and driven by `step()`**, with a work budget per
   step, instead of a background thread.

Two things follow from the limits above:

- **The memtable is a skiplist over a fixed-size arena.** A full arena is the
  flush trigger.
- **Write stalls are backpressure.** When the memtable is full and the flush
  has not finished, or level 0 has too many tables, writes are rejected.

## Ideas to try once the base works

| Idea | What changes | Why it fits |
| --- | --- | --- |
| WiscKey (FAST '16) | Values go to a separate log; the LSM keeps keys and pointers | Scans become many parallel random reads, which is where io_uring helps most |
| Monkey (SIGMOD '17) | Bloom filters get a different size per level | Small, isolated change |
| Dostoevsky (SIGMOD '18) | Lazy leveling: tiering in the upper levels, leveling in the last | Only the compaction policy changes |

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
- TigerBeetle. [*Tiger Style*](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md),
  for static allocation and putting a limit on everything.
- Peter Mbanugo. [*Why queues don't fix overload, and what to do instead*](https://pmbanugo.me/blog/why-queues-dont-fix-overload-and-what-to-do-instead).
- Lu et al. *WiscKey: Separating Keys from Values in SSD-conscious Storage*, FAST 2016.
- Dayan, Athanassoulis, Idreos. *Monkey: Optimal Navigable Key-Value Store*, SIGMOD 2017.
- Dayan, Idreos. *Dostoevsky: Better Space-Time Trade-Offs for LSM-Tree Based Key-Value Stores via Adaptive Removal of Superfluous Merging*, SIGMOD 2018.
