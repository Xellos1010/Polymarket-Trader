# runbook

## Service Overview

### Protocol Version

The Coinbase Derivatives Exchange (CDE) FIX API uses the FIX 4.4 protocol as a baseline. Refer to the official [FIX 4.4 specification](https://www.fixtrading.org/standards/fix-4-4/) for additional details. CDE also offers a custom Simple Binary Encoding (SBE) API.

### Hours of Operation

Orders entered outside trading hours are rejected. Firms are encouraged to stay connected 15 minutes after the official close to receive execution reports generated from trading session closing logic (e.g., Expired reports, Done for Day). [Contact the CDE team](mailto:derivatives@coinbase.com) for the current trading schedule—it resets its FIX sequences on a weekly basis. The reset schedule is configured during initial setup.

### Certification

To connect to Coinbase Derivatives Exchange, a firm must be certified. CDE provides separate environments for integration, acceptance testing, and certification. [Contact the CDE team](mailto:derivatives@coinbase.com) for details.

### Self-Match Prevention (SMP)

Coinbase Derivatives, LLC (CDE or Exchange) provides the ability for participants to utilize SMP to prevent the matching of orders with common ownership. Participants that fail to use SMP risk executing wash sales, which are prohibited under the Exchange’s Rulebook and constitute a violation of CFTC regulations. The following SMP Modes are supported:

-   Cancel Aggressing Order (default);
-   Cancel Resting Order;
-   Cancel Both Orders.

Default mode can be modified by having an Authorized Individual contact CDE at [derivatives@coinbase.com](mailto:derivatives@coinbase.com).

#### Trader - Order Entry (OE) Session Level SMP

Clients can also utilize this functionality across one or more of their OE sessions.

-   For FIX OE, the SMP identifier must be passed via Tag 7928 (`SelfMatchPreventionID`) on all [order messages](https://developer.chrome.com/fix/order-entry) in order for the functionality to work.
    -   The numeric tag value is provided by CDE during order entry onboarding.
    -   Default SMP Mode can be overridden by specifying [Tag 8000 (`SelfMatchPreventionStrategy`)](https://developer.chrome.com/fix/code-sets#selfmatchpreventionstrategy-8000) on an order.
-   For SBE Order Entry, there is no need to specify the SMP identifier.
    -   SMP Mode cannot be overridden via order message.

#### Entity-level SMP - Account Based SMP

CDE offers an Entity-Level SMP setting designed to simplify operations for brokers. This setting prevents self-matching within individual SubAccounts (Tag 1) under the same entity, while still allowing matching between different SubAccounts within the entity. Configured directly by the Exchange, this functionality removes the need for entities to send SMP identifiers, making it especially beneficial for retail brokers or scenarios where legal entities operate with a single SubAccount.

## Disaster Recovery

### Order Entry Gateways

Coinbase Derivatives Exchange operates two active gateways and clients can connect to either or both. All order events are sent to both gateways and are available for resend whether or not the client has an active session.

### Guaranteed Delivery

Sessions on each gateway are distinct and have independent sequence numbers. Events are uniquely identified by `ExecId` which guarantees that events received from dual sessions can be [deduplicated](https://www.google.com/search?sca_esv=567294360&cs=0&q=deduplicate+meaning&si=ALGXSlYpmWhtmlIZKYHTCPXiYmMEE7aOdl0cE_8JvblBTU7op-85NG57xcqXQmQHkQBE-9X2zAiI3EiCsbi-tcpLuFwtsxQ2e1GmGuubnqkd8LTPWoY37DC1aE9WzsfK5qZftml9pte0&expnd=1&sa=X&ved=2ahUKEwiy566GjbyBAxX1KFkFHY-zCwkQyNoBKAB6BAgREAA&biw=1404&bih=865&dpr=1.5&ictx=1).

### Recovering Missed Events

If the client is disconnected with one or more pending/unacknowledged requests, the client should:

1.  Connect to the other gateway (if not already connected).
2.  Send a [Last ExecId Request (35=F1)](https://developer.chrome.com/derivatives/fix/order-entry#last-execid-request-35f1) to determine the `ExecId` of the last order event sent by the trading system to the client.

If the client is missing events, where `ExecId` is less than `LastExecId` in [Last ExecId Request (35=F1)](https://developer.chrome.com/derivatives/fix/order-entry#last-execid-request-35f1), the client can either send an [Event Resend Request (35=F3)](https://developer.chrome.com/derivatives/fix/order-entry#event-resend-request-35f3), or a session-level [Resend Request (35=2)](https://developer.chrome.com/derivatives/fix/session#resend-request-352) message to retrieve missed events. Having recovered all missed events, the client can safely assume that any requests for which there was no corresponding event was either not processed by the trading system or was rejected.

### Sequence Number

CDE follows standard FIX protocol for sequence numbers. This dictates separate numbers for incoming and outgoing messages between the client system and Coinbase Derivatives Exchange, ensuring that all messages to and from CDE are in the correct order and recoverable. It is the client’s responsibility to reset the inbound and outbound sequence numbers to “1” prior to the Beginning of the Week Logon, increment the inbound/outbound sequence number by one for each message, maintain sequence numbers across all sessions and issue a resend request if gaps are detected. CDE resets sequence numbers to “1” across all gateways every Friday at 16:05 CT. See [FIX Session](https://developer.chrome.com/derivatives/fix/session) and [SBE Session](https://developer.chrome.com/derivatives/sbe/session) for client-initiated reset instructions.

### Cancel On Disconnect

A client may optionally be configured for **cancel-on-disconnect** orders. Cancel-on-disconnect applies to Day orders only (not GTC orders).

If a client has cancel-on-disconnect enabled and is connected to both active gateways, and disconnects from one of them, only the orders submitted (or last replaced/modified) by the disconnected session are canceled.

### Market Data Gateways

Coinbase Derivatives Exchange also operates two active Market Data gateways and clients can connect to either or both. When logging on, clients receive a full snapshot and ongoing updates of the market book after subscribing to the market data.