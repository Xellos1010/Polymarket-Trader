# rate limits

Coinbase Exchange real-time WebSocket market data updates provide fast insight into order flow and trades. This means that you are responsible for reading the message stream and using the message relevant for your needs—this can include building real-time order books or tracking real-time trades. The WebSocket API has two forms of rate limits—subscription limits and inbound message limits highlighted below. See also [Market Data Connections](https://help.coinbase.com/en/exchange/managing-my-account/market-data-connections) in the Help docs.

-   Requests per second per IP: 8
-   Requests per second per IP in bursts: Up to 20
-   Messages sent by the client every second per IP: 100

### Subscription Limits

-   Exchange accounts are limited to **10** WebSocket subscriptions on a per product, per channel basis. Users can purchase higher subscription limits if desired. Navigate to [Coinbase Developer Platform](https://portal.cdp.coinbase.com/products/exchange) to change your subscription.
-   If a user attempts to exceed **10** subscriptions per product, per channel, and is not a member of a paid subscription tier, the new subscription will be rejected.

### What is a subscription?

-   A subscription is defined on a per product, per channel basis.
-   Below is an example of a total of 4 subscriptions for BTC-USD full
    -   User123: BTC-USD Full Channel (unique)
    -   User123: BTC-USD Full Channel (duplicate)
    -   User123: BTC-USD Full Channel (duplicate)
    -   User123: BTC-USD Full Channel (duplicate)
    -   User123: BTC-USD Level 2 (unique)
    -   User123: BTC-USD Level 3 (unique)
-   In this case, User123 has 6 remaining subscriptions to BTC-USD full channel, and 9 remaining subscriptions to BTC-USD Level 2 and Level 3 channels.

### Inbound Message Limits

-   All WebSocket inbound messages are subject to a rate limit of **10** RPS / **1000** burst RPS.

### What is an inbound message limit?

-   When a user sends any message (subscribing to a WebSocket channel, unsubscribing to a WebSocket channel, etc.) it is counted towards their inbound message limit.
-   Users can subscribe/unsubscribe to multiple channels and products within a single inbound message.
-   Limits are enforced using a **lazy-fill token bucket** implementation. More information, including examples, can be found at [How Rate Limits Work](https://developer.chrome.com/exchange/rest-api/rate-limits#how-rate-limits-work).