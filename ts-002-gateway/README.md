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







.
