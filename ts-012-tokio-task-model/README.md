# Trading System

> Architecture Prototype — Trading-sys-001 ~ Trading-sys-010  
> Status: **Completed**

This repository records the architecture evolution of a Rust trading system from a basic TCP gateway to a single-owner event-driven engine with durable Journal, Replay, Snapshot Recovery, and Checkpoint Creation.

The current stage is intentionally an **architecture prototype**. The goal is to establish clear ownership, state-machine, persistence, recovery, and runtime boundaries before later phases add async I/O, richer OrderBook behavior, matching, market data, FIX, production hardening, and latency optimization.

---

## 1. Architecture Scope

The prototype currently focuses on the **Order path**.

```text
Client
  |
  v
Order Gateway
  |
  v
Protocol
  |
  v
OrderBook Engine
  |
  +----------------------+
  |                      |
  v                      v
Event / Sequencer     OrderBook
  |                  State Machine
  v
Journal
```

Future components remain in the workspace architecture map, but are not the implementation focus of this prototype:

```text
OrderBook / Event
      |
      v
Market Data Engine
      |
      v
Market Gateway
      |
      v
Client
```

### Design Principle

Order processing and market-data distribution are separated because they have different latency, consistency, throughput, and backpressure requirements.

---

## 2. Core Architecture

```text
                              Trading System

 Client
   |
   v
+----------------+
| order-gateway  |
+----------------+
        |
        v
+----------------+
|    protocol    |
+----------------+
        |
        v
+----------------------+
|   orderbook-engine   |
|                      |
|  Order State Machine |
+----------------------+
        |
        | state-changing command
        v
+----------------+
|     event      |
|   Sequencer    |
+----------------+
        |
        v
+----------------+
|    journal     |
| durable events |
+----------------+

Recovery side:

    Snapshot -------------------+
       |                        |
       v                        |
  restore State@N               |
       |                        |
       +---- Journal Tail ------+
                 |
                 v
          Recovered State
                 |
          ownership handoff
                 |
                 v
            Engine Loop
```

---

## 3. Fundamental Design Rules

### 3.1 OrderBook is the State Machine

The `OrderBook` is the current business state.

```text
State(N)
   +
Event(N+1)
   |
   v
OrderBook::apply()
   |
   v
State(N+1)
```

The state machine does not need to know about TCP, threads, channels, Journal files, Snapshot files, or Replay orchestration.

### 3.2 Recovery and Runtime are Separate Phases

Recovery handles the **old world**.

```text
Historical persistence
        |
        v
recover_engine_state()
        |
        v
RecoveredEngineState
```

The Engine Loop handles the **new world**.

```text
RecoveredEngineState
        |
        | ownership move
        v
start_engine_loop()
        |
        v
new Command -> new Event -> new State
```

The server startup order is therefore:

```text
1. Recovery
      |
      v
2. Engine Runtime
      |
      v
3. TCP bind
      |
      v
4. Accept loop
      |
      v
5. Session threads
```

The server does not begin accepting traffic before recovery is complete.

### 3.3 Single-Owner

The mutable OrderBook is owned by one Engine thread.

```text
Session A ----+
Session B ----+--> EngineProxy --> mpsc --> Engine Thread --> OrderBook
Session C ----+
```

Gateway/session threads never mutate the OrderBook directly.

### 3.4 Journal is the Durable Source of Truth

For a state-changing event:

```text
EngineEvent
    |
    v
Sequencer
    |
    v
SequencedEvent
    |
    v
Journal.append()
    |
    v
sync_data()
    |
    v
OrderBook::apply()
    |
    v
last_applied_seq
    |
    v
Reply
```

The critical ordering rule is:

```text
Journal durable
      BEFORE
OrderBook apply
```

If Journal persistence fails, the new state must not be applied.

### 3.5 Snapshot is a Recovery Accelerator, not the Source of Truth

```text
Journal
=
authoritative durable history

Snapshot
=
recovery checkpoint / acceleration artifact
```

A broken or missing Snapshot can fall back to full Journal Replay.

A broken Journal cannot be silently ignored.

---

## 4. Order Flow

### 4.1 Read-only Request

`BOOK` does not create an Event and does not consume a sequence number.

```text
Client
  |
  v
OrderSession
  |
  v
OrderCodec
  |
  v
OrderRequest::Book
  |
  v
SimpleOrderHandler
  |
  v
EngineProxy
  |
  v
EngineCommand::GetBook
  |
  v
Engine Loop
  |
  v
OrderBook::snapshot()
  |
  v
BookSnapshot {
    as_of_seq,
    orders
}
```

`as_of_seq` is the version of the state represented by the returned snapshot.

### 4.2 State-changing Request

```text
Client
  |
  v
OrderSession
  |
  v
OrderCodec
  |
  v
OrderRequest::AddOrder
  |
  v
SimpleOrderHandler
  |
  v
EngineProxy
  |
  v
EngineCommand::AddOrder
  |
  v
================ Engine Thread ================
  |
  v
EngineEvent::OrderAdded
  |
  v
Sequencer
  |
  v
SequencedEvent
  |
  v
Journal
  |
  v
OrderBook::apply()
  |
  v
last_applied_seq
  |
  v
Reply
```

---

## 5. Persistence and Recovery

### 5.1 Full Replay

Trading-sys-008 introduced startup Replay.

```text
==================== STARTUP ====================

Journal File
    |
    v
SequencedEvent
    |
    v
Replay
    |
    v
OrderBook::apply()
    |
    v
restored OrderBook
    |
    +--> last_applied_seq
    |
    +--> Sequencer resumes at last_applied_seq + 1


===================== LIVE ======================

EngineCommand
    |
    v
EngineEvent
    |
    v
Sequencer
    |
    v
SequencedEvent
    |
    v
Journal
    |
    v
OrderBook::apply()
```

Replay validates sequence continuity. A sequence gap rejects recovery rather than silently producing an incomplete state.

### 5.2 Recovery / Runtime Boundary

Trading-sys-009 made the startup lifecycle explicit:

```text
recover_engine_state(...)
        |
        v
RecoveredEngineState
        |
        | ownership handoff
        v
start_engine_loop(...)
```

`RecoveredEngineState` represents the result of recovery, not the recovery method.

Recovery may come from:

```text
Full Journal Replay

or

Snapshot + Journal Tail Replay
```

The Engine Loop does not perform historical recovery. It begins from an already recovered state and processes new commands.

### 5.3 Snapshot Data

A persistent Snapshot records:

```text
schema_version
as_of_seq
journal_offset
orders
```

Example:

```text
SNAPSHOT|1|3|81|3
ORDER|1|BTCUSDT|10
ORDER|2|ETHUSDT|20
ORDER|3|SOLUSDT|30
```

Meaning:

```text
schema_version  = 1
as_of_seq       = 3
journal_offset  = 81 bytes
order_count     = 3
```

`as_of_seq` and `journal_offset` solve different problems:

```text
as_of_seq
=
logical recovery position

journal_offset
=
physical Journal read position
```

### 5.4 Snapshot + Journal Tail Recovery

```text
Journal:
seq1
seq2
seq3
seq4
seq5

Snapshot:
State@3
journal_offset = end of seq3

Recovery:
Snapshot@3
    |
    v
restore OrderBook
    |
    v
seek(journal_offset)
    |
    v
Replay seq4, seq5 only
    |
    v
Recovered State@5
    |
    v
Sequencer resumes at 6
```

This avoids parsing and applying the already-checkpointed prefix of a large Journal.

### 5.5 Checkpoint Validation

A Snapshot is only usable when its logical and physical positions agree.

```text
Snapshot:
as_of_seq = N
journal_offset = X
```

Recovery validates that:

```text
X is a Journal record boundary

and

the Journal record immediately before X has seq_id = N
```

This prevents a bad offset from silently skipping durable events.

### 5.6 Snapshot Fallback

```text
Snapshot missing
    |
    v
Full Replay

Snapshot broken / unsupported
    |
    v
Full Replay

Snapshot checkpoint invalid
    |
    v
Full Replay

Full Replay invalid
    |
    v
Startup failure
```

---

## 6. Snapshot Creation and Checkpoint Management

Trading-sys-010 completes the other half of the Snapshot lifecycle.

```text
Recovery side:
Snapshot + Journal Tail -> State

Creation side:
State + Journal Position -> Snapshot
```

### 6.1 Atomic Snapshot Replacement

Snapshot persistence uses a temporary file:

```text
order.snapshot.tmp
        |
        v
write complete
        |
        v
sync_data()
        |
        v
rename
        |
        v
order.snapshot
```

The existing valid Snapshot is not truncated before the replacement is complete.

This is a prototype-level atomic replacement mechanism; advanced crash-consistency work such as directory fsync, multiple generations, checksums, and retention is deferred.

### 6.2 Journal Current Offset

For the current append-only, single-writer Journal:

```text
current_offset = Journal file length
```

The checkpoint stores the offset immediately after the durable event represented by `as_of_seq`.

### 6.3 Bootstrap Checkpoint

After startup recovery:

```text
Recovery Complete
      |
      v
Recovered State@N
      |
      v
open Journal writer
      |
      v
current Journal offset
      |
      v
create Snapshot@N
      |
      v
start Engine Loop
```

An old deployment containing only a Journal needs one full Replay after upgrade, then future restarts can use Snapshot Recovery.

For an empty state (`as_of_seq = 0`), no Snapshot is required.

### 6.4 Periodic Checkpoint

During live processing:

```text
Command
   |
   v
Event
   |
   v
Journal durable
   |
   v
OrderBook apply
   |
   v
last_applied_seq
   |
   v
Reply
   |
   v
checkpoint due?
   |
   +-- no --> next command
   |
   +-- yes
          |
          v
   Journal current_offset
          |
          v
   create Snapshot@N
```

The prototype uses sequence-based checkpoint boundaries:

```text
last_applied_seq % checkpoint_interval == 0
```

Example with interval `2`:

```text
seq1 -> no checkpoint
seq2 -> Snapshot@2
seq3 -> Snapshot remains @2
seq4 -> Snapshot@4
```

### 6.5 Snapshot Failure Policy

Checkpoint creation is intentionally non-fatal.

```text
Journal append failure
        |
        v
state transition must not proceed


Snapshot creation failure
        |
        v
log error
        |
        v
Engine continues
```

A failed Snapshot only makes the next restart slower because the Journal remains authoritative.

---

## 7. Complete Persistence Lifecycle

Trading-sys-006 through Trading-sys-010 now form one closed architecture:

```text
006 Event + Sequencer
        |
        v
007 Journal
        |
        v
008 Replay
        |
        v
009 Snapshot Recovery
        |
        v
010 Snapshot Creation + Checkpoint Management
```

Full lifecycle:

```text
                       LIVE
                        |
                        v
Command -> Event -> Journal -> OrderBook
                        |          |
                        |          v
                        |      Snapshot
                        |          |
                        |        restart
                        |          |
                        +----------+
                                   |
                                   v
                         Snapshot Recovery
                                   |
                                   v
                              OrderBook
                                   |
                                   +----> LIVE
```

---

## 8. Workspace Responsibilities

```text
crates/model
    Shared domain models.

crates/order-gateway
    Order-side session and request dispatch.

crates/protocol
    Wire request/response codec.

crates/orderbook-engine
    Single-owner runtime, OrderBook state machine,
    recovery/runtime handoff, checkpoint orchestration.

crates/event
    EngineEvent, SequencedEvent, Sequencer.

crates/journal
    Append-only durable event storage and offset-aware reads.

crates/replay
    Full Replay and Journal Tail Replay.

crates/snapshot
    SnapshotData, Snapshot codec, Snapshot file persistence.

crates/instrument
    Reference-data boundary reserved for later phases.

crates/market-data-engine
    Future market-data transformation path.

crates/market-gateway
    Future market-data distribution path.

crates/transport
    Inter-component transport boundary reserved for later evolution.

crates/common
    Shared infrastructure utilities.

apps/order-server
    Composition root:
    Recovery -> Engine Runtime -> Network -> Sessions.

apps/market-server
    Reserved market-data application boundary.

apps/test-client
    Test client.
```

---

## 9. Version Summary

### Trading-sys-001 — Workspace + System Architecture Map (★★)

Established the workspace and the system-level component map.

The project was organized as a multi-crate system from the beginning so later stages could evolve inside stable architectural boundaries.

### Trading-sys-002 — TCP Foundation (★★)

Established the basic TCP layering:

```text
TcpStream
  |
Session
  |
Protocol
  |
Handler
```

### Trading-sys-003 — Multi-Connection Gateway (★★★)

Added multiple concurrent client connections using thread-per-connection.

This exposed the state-ownership problem: if each connection owns its own OrderBook, clients do not observe the same business state.

### Trading-sys-004 — Shared Mutable OrderBook (★★★★)

Solved cross-connection state visibility with:

```text
Connection A --+
Connection B --+--> Arc<Mutex<OrderBook>>
Connection C --+
```

This stage established the difference between shared ownership and synchronized mutable access.

### Trading-sys-005 — Single-Owner Engine Loop (★★★★)

Replaced shared mutable OrderBook access with message passing:

```text
Gateway Threads
      |
      v
EngineProxy
      |
      v
mpsc
      |
================ Thread Boundary ================
      |
      v
Engine Loop
      |
      v
OrderBook
```

The Engine thread became the unique mutable owner of the OrderBook.

### Trading-sys-006 — Event + Sequencer (★★)

Separated command intent from confirmed state-changing facts.

```text
EngineCommand
     |
     v
EngineEvent
     |
     v
Sequencer
     |
     v
SequencedEvent
     |
     v
OrderBook::apply()
```

`BOOK` remains read-only and does not consume `seq_id`.

### Trading-sys-007 — Journal (★★★)

Inserted durable event persistence before state mutation.

```text
SequencedEvent
      |
      v
Journal append
      |
      v
sync_data
      |
      v
OrderBook::apply
```

This established write-before-apply.

### Trading-sys-008 — Replay (★★★)

Added startup reconstruction from persisted `SequencedEvent`s.

Recovery restores:

```text
OrderBook
last_applied_seq
Sequencer continuation
```

Sequence gaps reject startup.

### Trading-sys-009 — Snapshot Recovery (★★★★)

Separated Recovery from Runtime:

```text
recover_engine_state()
        |
        v
RecoveredEngineState
        |
        v
start_engine_loop()
```

Added:

```text
SnapshotData
journal byte offset
checkpoint validation
Journal Tail Replay
Snapshot fallback to Full Replay
```

Recovery became:

```text
Snapshot@N + Journal(N+1...) -> State@Latest
```

### Trading-sys-010 — Snapshot Creation + Checkpoint Management (★★★～★★★★)

Completed the Snapshot lifecycle by adding:

```text
Journal current_offset
atomic Snapshot creation
bootstrap checkpoint
periodic checkpoint
non-fatal Snapshot failure policy
```

The system can now create a recovery checkpoint during runtime and consume it after restart.

---

## 10. Architecture Evolution

### 10.1 Shared State Problem

```text
Connection A -> OrderBook A
Connection B -> OrderBook B

Problem:
business state lifetime is bound to each connection.
```

The required model is:

```text
many connections
      |
      v
one business state
```

### 10.2 Shared Mutable State

```text
Connection A --+
Connection B --+--> Arc<Mutex<OrderBook>>
Connection C --+
```

Correct for shared visibility, but introduces shared mutable access and contention.

### 10.3 Single-Owner Runtime

```text
Client
  |
OrderSession
  |
OrderCodec
  |
OrderRequest
  |
SimpleOrderHandler
  |
EngineProxy
  |
EngineCommand
  |
mpsc Sender
  |
================ Thread Boundary ================
  |
mpsc Receiver
  |
Engine Loop
  |
OrderBook
  |
Engine Reply
  |
reply channel
  |
EngineProxy
  |
SimpleOrderHandler
  |
OrderResponse
  |
OrderCodec
  |
Client
```

### 10.4 Event + Sequence

```text
Gateway Threads
      |
      v
EngineProxy
      |
      v
mpsc
      |
================ Engine Thread ================
      |
      v
EngineCommand
      |
      v
EngineEvent
      |
      v
Sequencer
      |
      v
SequencedEvent
      |
      v
OrderBook::apply
      |
      v
last_applied_seq
      |
      v
Reply
```

### 10.5 Journal

```text
SequencedEvent
      |
      v
Journal.append
      |
      v
sync_data
      |
      v
durable OK
      |
      v
OrderBook::apply
      |
      v
last_applied_seq
```

### 10.6 Replay

```text
Journal
   |
   v
Historical SequencedEvent
   |
   v
OrderBook::apply
   |
   v
Recovered State
```

### 10.7 Snapshot Recovery

```text
Snapshot@N
    |
    v
OrderBook::from_orders
    |
    v
State@N
    |
    +---- Journal Tail N+1...
              |
              v
        OrderBook::apply
              |
              v
        State@Latest
```

### 10.8 Checkpoint Creation

```text
State@N
   +
JournalOffset@N
   |
   v
SnapshotData
   |
   v
SnapshotFile::save_atomic
   |
   v
order.snapshot
```

---

## 11. Prototype Acceptance Scenarios

```text
1. Empty Journal + no Snapshot
   -> State@0
   -> first live event starts at seq1

2. Journal only
   -> Full Replay
   -> recover latest state
   -> bootstrap Snapshot

3. Snapshot@N + empty Journal tail
   -> restore State@N

4. Snapshot@N + Journal Tail
   -> restore Snapshot
   -> replay N+1...
   -> recover latest state

5. Stale Snapshot
   -> tail replay catches up

6. Broken Snapshot
   -> fallback Full Replay

7. Invalid Snapshot journal_offset
   -> reject Snapshot
   -> fallback Full Replay

8. Journal sequence gap
   -> recovery failure

9. Periodic checkpoint boundary
   -> Snapshot created at configured sequence interval

10. Restart after stale Snapshot
    -> Snapshot Recovery + Tail Replay
    -> bootstrap checkpoint refreshes Snapshot

11. New live command after recovery
    -> sequence continues from last_applied_seq + 1

12. BOOK
    -> returns current as_of_seq
    -> consumes no sequence number
```

---

## 12. Current Prototype Boundaries

The following are intentionally **not solved yet** in Trading-sys-001 ~ Trading-sys-010:

```text
Async Tokio networking
Heartbeat / Timeout
Graceful Shutdown
Production Error System

Full OrderBook price-level model
OrderId index
Cancel / Reduce
Matching Engine
Execution / Fill model
Order lifecycle
Pre-trade risk

Market-data subscription / push / backpressure
FIX protocol

Journal compaction / truncation
Snapshot retention / multiple generations
Checksum / CRC
Snapshot compression
Incremental Snapshot
Production-grade crash consistency

Observability / metrics / tracing
Production hardening
Latency benchmark and optimization
```

These belong to later Trading System stages.

---

## 13. Performance / Lock-Free Review

The prototype uses single-owner state mutation, but that does **not** mean the whole system is lock-free.

Current runtime still contains:

```text
std::mpsc
reply channels
String allocation
Journal file I/O
sync_data()
Snapshot cloning
Snapshot serialization
Snapshot sync_data()
```

Periodic Snapshot creation currently runs on the Engine thread:

```text
Engine Thread
    |
    v
book.snapshot() clone
    |
    v
serialize
    |
    v
write Snapshot
    |
    v
sync_data()
```

Therefore a checkpoint can pause processing of the **next** command and affect queue buildup and tail latency.

This is intentionally accepted in the architecture prototype.

A future optimization candidate is:

```text
Engine
  |
  v
immutable snapshot image
  |
  v
background snapshot writer
```

That optimization is deferred because it introduces new ownership, queue, lag, and backpressure problems that should be studied separately.

---

## 14. Development Commands

From the workspace root:

```bash
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

Run the Order Server from the workspace root so relative persistence paths resolve as expected:

```bash
cargo run -p order-server
```

For example:

```text
data/order.journal
data/order.snapshot
```

are resolved relative to the process current working directory.

Important:

```text
Cargo locating Cargo.toml
!=
runtime relative-path base
```

Running `cargo run -p order-server` from another directory can therefore resolve `data/...` to a different location.

---

# Architecture Prototype Complete

Trading-sys-001 through Trading-sys-010 establish the first complete architecture foundation:

```text
TCP / Gateway
      |
      v
Multi-Connection
      |
      v
Shared Mutable State
      |
      v
Single-Owner Engine
      |
      v
Event + Sequencer
      |
      v
Journal
      |
      v
Replay
      |
      v
Snapshot Recovery
      |
      v
Snapshot Creation + Checkpoint Management
```

The most important result is a clear system model:

```text
OrderBook = State Machine

Recovery Phase
    !=
Runtime Phase

Journal = Durable Source of Truth

Snapshot = Recoverable Checkpoint

Single Owner
    +
Message Passing
    +
Event Sequencing
    +
Durable Journal
    +
Replay
    +
Snapshot Recovery
    +
Checkpoint Creation
```

This is the baseline for the next Trading System phases.
