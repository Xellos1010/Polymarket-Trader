# best practices

-   You can subscribe to both `ws-feed` (Coinbase Market Data) and `ws-direct` (Coinbase Direct Market Data), but if `ws-direct` is your primary connection, we recommend using `ws-feed` as a failover.
-   Remember [WebSocket rate limits](https://developer.chrome.com/exchange/websocket-feed/rate-limits).
-   Spread subscriptions (especially full channel subscriptions) over more than one websocket client connection. For example, do not subscribe to BTC-USD and ETH-USD on the same channel if possible. Instead, open up two separate websocket connections to help load balance those inbound messages across separate connections.
-   Websocket clients should authenticate to help troubleshoot issues if necessary. Authenticating is optional and does not impact web socket performance.
-   Connected clients should increase their web socket receive buffer to the largest configurable amount possible (given any client library or infrastructure limitations), due to the potential volume of data for any given product.
-   Include the following header in the opening handshake to allow for compression, which will lower bandwidth consumption with minimal impact to CPU / memory: `Sec-WebSocket-Extensions: permessage-deflate`. See [Websocket Compression Extension](https://developer.chrome.com/exchange/websocket-feed/overview#websocket-compression-extension)
-   Use less verbose subscriptions where possible (e.g., Level 2 over Full/Level 3).
-   Use alternative batch channels like “level2\_batch” instead of “level2” and “ticket\_batch” instead of “ticket” which deliver a batched version of the respective data on a set interval reducing overall traffic.
-   Mitigate error messages which are returned when the client is actively disconnected for any of these reasons:
    
    -   The client has too many backed up messages (`ErrSlowConsume`)
    
    Limit the use of I/O operations and in-memory lock-free constructs when processing any websocket client callbacks. Queuing messages and processing them off-thread is another strategy that can prevent slow consumer errors.
    
    -   The client is sending too many messages (`ErrSlowRead`)
    
    Space out websocket requests to adhere to the above rate limits.
    
    -   The message size is too large (“Message too big”).
    
    Break up your subscription messages into smaller requests abiding by the rate limits.