# Trading System
## Trading System Architecture

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


### Order Path
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


### Market Data Path
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

## Order Flow
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

## TCP-7A Shared State Problem
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


## TCP-7B Shared State Resolution
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
.