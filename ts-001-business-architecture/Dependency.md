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