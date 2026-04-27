# best practices

## Optimize Traffic

To optimize your FIX set up, spread traffic over as many portfolios as possible to minimize order-entry latencies. You should adhere to the following:

-   `1` API key per session/connection to guarantee a connection
-   `75` maximum connections per profile
-   `175` maximum connections per user across all profiles

## Batch Messages

We strongly recommend batch messages for both order entry and cancellation. Batch Requests:

-   Can have up to `15` orders / cancels per request.
-   Only count for a single message for the purposes of rate limiting.
-   Can be more efficient to process compared to the equivalent individual requests.

Available batch messages are:

-   [New Order Batch (U6)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#neworderbatch-35%3Du6)
-   [Order Cancel Batch Request (U4)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelbatch-35%3Du4)
-   [New Order Batch Reject (U7)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#neworderbatchreject-35%3Du7)
-   [Order Cancel Batch Reject (U5)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelbatchreject-35%3Du5)

## Modify Order Requests

We strongly recommend Modify Order Requests where applicable. Modify Order Requests:

-   Keep your place in the order book queue when size is amended down.
-   Result in 50% fewer messages when compared to canceling an existing order and placing a new one.
-   Reduce your overall rate limit usage when compared to sending a cancellation followed by a new order.
-   Can be more efficient to process compared to the equivalent individual cancel and new order requests.

For rate limits, see [FIX API Rate Limits](https://developer.chrome.com/exchange/fix-api/rate-limits).

## Drop Copy Session

Enabling `DropCopyFlag="Yes"` (`9406=Y`) configures your session to receive execution reports across all active sessions for the same profile. We recommend that you enable DropCopyFlag on a **separate, read-only session**:

-   Execution latencies are higher for Drop Copy sessions because the same connection handles more read traffic.
-   Multiple Drop Copy sessions produce multiple copies of redundant data.