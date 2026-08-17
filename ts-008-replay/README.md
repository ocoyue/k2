# Trading System
## Trading System Architecture(业务全景结构)

```text
                         Trading System


 Client
   |
   |
   v

+----------------+
| order-gateway  |
+----------------+
        |
        |
        v

+----------------+
|   protocol     |
+----------------+
        |
        |
        v

+----------------------+
|   orderbook-engine   |
|                      |
|  Order State Machine |
+----------------------+

        |
        |
        v

+----------------+
|    event       |
+----------------+

        |
        |
        +----------------+
        |                |
        v                v

+--------------+   +----------------------+
|   journal    |   | market-data-engine   |
|              |   |                      |
| persistence  |   | market transformation|
+--------------+   +----------------------+

                          |
                          |
                          v

                 +----------------+
                 | market-gateway |
                 +----------------+

                          |
                          |
                          v

                       Client
```

## System Data Flow


### Order Path(订单流程路径)
```text
Client  
|  
Order Gateway  
|  
Protocol  
|  
OrderBook Engine  
|  
Event  
|  
Journal
```


### Market Data Path(行情流程路径)
```text
OrderBook Engine  
|  
Event  
|  
Market Data Engine  
|  
Market Gateway  
|  
Client
```


### Design Principle

Order processing and market data distribution
are separated because they have different
latency, consistency and throughput requirements.

(订单处理与市场数据分发是分开的，因为它们对延迟、一致性和吞吐量有着不同的要求。)



## Dependency Relationship
```text

                    common
                       |
                       |
                    model
                       |
          +------------+-------------+
          |                          |
    instrument                    event
                                     |
                                     |
                        +------------+-----------+
                        |                        |
                  orderbook-engine    market-data-engine
                                     |
                                     |
                                     |
                                 protocol
                                     |
                                     |
                            +-----------------+
                            |                 |
                     order-gateway   market-gateway
                                     |
                                     |
                                     |
                                 transport
                                     |
                                     |
                                   apps

```

# Version Summary

###### Trading System 000 achitecture

ld the Architecture for business。建立业务全景

###### Trading System 001 tcp raw

TCP RAW base

###### Trading System 002 session gateway

Gateway , split layer.

###### Trading System 003 multiple connections

Support multiple connection for clients

###### Trading System 004 sharing memory

Use Arc<Mutex<T>>  to resolve the trumble of "how many threads how many sets of data"

###### Trading System 005 single-owner

Arc<Mutex<T>> way is disadvantage. Switch single-owner through multiplt producer single consumer way .

###### Trading System 006

Introduce EngineEvent and Sequencer.
State-changing Commands are converted into SequencedEvents before OrderBook::apply().
BOOK does not generate an Event and returns as_of_seq as the current state version.

###### Trading System 007 journal

Add journal between OrderCommand and apply .

###### Trading System 008 replay

Restore OrderBook, last_applied_seq and Sequencer from persisted SequencedEvents before live processing begins.
Existing Journal can now be replayed and safely continued, while sequence gaps reject startup.


## Order Flow Evolution

### TCP-2 tcp raw

```text
Client
  |
  | TCP text
  v
OrderSession
  |
  | decode
  v
OrderRequest
  |
  v
OrderHandler
  |
  | 分发业务
  v
BookService
  |
  | 业务执行
  v
MiniOrderBook(domain)
  |
  | 业务执行结果
  v
BookService
  |
  v
OrderHandler
  |
  | 组织协议响应
  v
OrderResponse
  |
  | encode
  v
OrderSession
  |
  v
Client
```

### TCP-3 Shared State Problem
```text
Current runtime ownership:

Connection A
|
OrderSession A
|
SimpleOrderHandler A
|
BookService A
|
MiniOrderBook A


Connection B
|
OrderSession B
|
SimpleOrderHandler B
|
BookService B
|
MiniOrderBook B


Single-client behavior:

ADD 1 BTCUSDT 10
BOOK

works correctly.


Multi-client behavior:

Client A:

ADD 1 BTCUSDT 10


Client B:

BOOK


Client B cannot see Order 1.


Root Cause:

MiniOrderBook is currently created inside the
thread-per-connection closure.

Therefore its lifetime and ownership are bound to
one connection.

But the business requirement is:

many connections
|
v
one global MiniOrderBook


The current implementation is:

one connection
|
v
one MiniOrderBook


The TCP layer is correct.

The Session layer is correct.

The Protocol layer is correct.

The Handler/Business call chain is correct.

The error is the ownership scope of business state.


TCP-7B will solve:

many connection threads
|
v
shared mutable MiniOrderBook

using:

Arc<Mutex<MiniOrderBook>>
```


### TCP-4 Shared State Resolution
```text
TCP-7A:

Connection A -> MiniOrderBook A
Connection B -> MiniOrderBook B

Problem:

Business state was scoped to each connection.


TCP-7B:

Connection A --+
Connection B --+--> Arc<Mutex<MiniOrderBook>>
Connection C --+


Arc:

provides shared ownership of the same business state.


Mutex:

provides mutually exclusive mutable access.


Result:

Orders added by one client are visible to
other clients through BOOK.


At this stage Arc<Mutex<MiniOrderBook>> is treated
as the correct solution.

Its architectural limitations will be analyzed
in TCP-8.
```


### TCP-5 single owner engine

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



### Trading-sys-006-event + sequencer

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


State-changing command:

ADD
 |
 v
EngineCommand::AddOrder
 |
 v
EngineEvent::OrderAdded
 |
 v
Sequencer
 |
 v
SequencedEvent { seq_id, event }
 |
 v
OrderBook::apply()
 |
 v
last_applied_seq = seq_id


Read-only command:

BOOK
 |
 v
EngineCommand::GetBook
 |
 v
OrderBook::snapshot()
 |
 v
BookSnapshot {
    as_of_seq: last_applied_seq
 }

BOOK does not generate an Event
and does not consume a seq_id.
```

### Trading-sys-007-journal

```test
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
                     append
                          |
                          v
                      Journal
                          |
                     sync_data
                          |
                    durable OK
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

### Trading-sys-008-replay
```text
==================== STARTUP ====================

                    Journal File
                         |
                         v
                      Reader
                         |
                         v
                  SequencedEvent
                         |
                         v
                       Replay
                         |
                         v
                 OrderBook::apply
                         |
                         v
              restored OrderBook
                         |
                         +--> last_applied_seq
                         |
                         +--> Sequencer resumes


==================== LIVE =======================

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
                 OrderBook::apply
```
