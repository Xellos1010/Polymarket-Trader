# overview

The Coinbase Derivatives Exchange Simple Binary Encoding (SBE) API is a custom high performant order entry protocol used by lead market makers to send, modify, and cancel orders.

## Session Protocol

The SBE API implements a subset of the [FIX session protocol](https://developer.chrome.com/derivatives/sbe/session) and session messages behave like FIX.

## Sequence Numbers

Sequence numbers are assigned and validated like [FIX sequence numbers](https://developer.chrome.com/derivatives/fix/overview#sequence-numbers).

## Trade Busts and Corrects

The Coinbase Derivatives Exchange SBE API does not distribute unsolicited reports about trade busts and corrections. The firms are expected to use the FIX [Drop Copy](https://developer.chrome.com/derivatives/fix/drop-copy) connections if they need to receive these messages.

## Expiration Reports

When a day order expires at the close of a trading day, an [OrderCanceled](https://developer.chrome.com/derivatives/sbe/order-entry#ordercanceled) message is sent.

## CorrelationId

Every application message contains an 8-byte integer `correlationId`. Clients can assign any value to `correlationId`, which is not validated by the server. However, Coinbase recommends monotonically increasing the value. Correlation ID is used for indirectly correlated messages, such as order fill and system cancel notifications. Messages from server to client use the `correlationId` of either the **corresponding request message from the client** or of the **last related request**.

## Byte Alignment and Message Padding

Messages are laid out as follows:

-   8-byte fields start on 8-byte boundaries
-   4-byte fields start on 4-byte boundaries
-   2-byte fields start on 2-byte boundaries.

The frame length of all outbound messages to the client are rounded up to the nearest multiple of 8. Clients are encouraged to do the same with inbound messages, although this is not required.