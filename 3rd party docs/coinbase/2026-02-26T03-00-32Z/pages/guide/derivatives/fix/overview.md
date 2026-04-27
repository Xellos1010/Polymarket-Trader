# overview

Coinbase Derivatives Exchange for brokers and market makers provides the following FIX categories:

-   [Market Data](https://developer.chrome.com/derivatives/fix/market-data) (Pre-trade): Receive market data.
-   [Order Entry](https://developer.chrome.com/derivatives/fix/order-entry) (Trade): Send, modify and cancel orders.
-   [Drop Copy](https://developer.chrome.com/derivatives/fix/drop-copy) (Post-trade): Receive real-time copies of Order Entry Execution Reports and Acks.

## Required Identifiers

### Firm Identifiers

All messages sent to/from the exchange must contain both the `SenderCompID (49)` and `TargetCompID (56)` fields in the [Standard Header](https://developer.chrome.com/derivatives/fix/header-trailer#standard-header). The firm and the exchange agree on these values at the time of the firm onboarding. A firm can have multiple connections, each with its own “CompID” (or company ID), e.g., `SenderCompID`. A CompID is the concatenation of `SubFirmID` (3 letters) and `SessionID` (3 numbers).

Tag

Field

Firm to Exchange

Exchange to Firm

49

SenderCompID

, e.g., `EBR123`

Always `COIND`

56

TargetCompID

Always `COIND`

, e.g., `EBR123`

All fix-order application messages sent to/from the exchange must also contain both the `SenderSubID (50)` and `TargetSubID (57)` fields.

Tag

Field

Firm to Exchange

Exchange to Firm

50

SenderSubID

ID of exchange environment: `PROD` or `TEST`

57

TargetSubID

ID of destination exchange system

UUID of end trader submitting orders.

### Client Order Identifiers

All orders submitted to Coinbase Derivatives Exchange must have a unique `ClOrdID (11)` in the message body. The exchange only enforces the uniqueness of the identifier among working orders (for example, GTC and non-triggered stops). Non-unique IDs can cause issues with reporting, clearing, and support.

## Self-Match Prevention

Self-match prevention functionality helps market participants prevent self-trading so that orders for the same account, firm, or group of accounts do not match with each other. Self-Match Prevention is optional and controlled with a pair of FIX tags on incoming orders:

-   SelfMatchPreventionID (7928): Orders from the same executing firm with the same SelfMatchPreventionID will not match. Required length of the ID is 8 Digits.
-   [SelfMatchPreventionStrategy (8000)](https://developer.chrome.com/derivatives/fix/code-sets#selfmatchpreventionstrategy-8000): Defines the strategy of dealing with matching orders if self-match prevention is triggered. The exchange can either cancel the aggressor order, the resting order, or both. The exchange uses the strategy from the aggressor order to deal with self-matched orders.

## Sequence Numbers

Within a [session](https://developer.chrome.com/derivatives/fix/session), all FIX messages are identified by unique integer **sequence numbers** and are processed in that order. When an incoming sequence number does not match the expected one, the session must be recovered:

-   If the incoming sequence number is less than expected, and `PossDupFlag` ≠ `Y`, it is considered a fatal error, and the connection is dropped by the server.
-   If the sequence number of incoming message is greater than the next expected number, a [Resend Request (35=2)](https://developer.chrome.com/derivatives/fix/session#resend-request-352) is issued for missed messages.

The exchange resets the sequences weekly. Reset schedule is configured during initial setup. Resets may also be initiated by a firm sending a Logon message with `ResetSeqNumFlag=Y`.

## Trade Busts and Corrects

The Coinbase Derivatives Exchange FIX [Order Management API](https://developer.chrome.com/derivatives/fix/order-entry) does not distribute unsolicited reports about trade busts and corrections. The firms are expected to use the FIX [Drop Copy](https://developer.chrome.com/derivatives/fix/drop-copy) connections if they need to receive these messages.

## Done for Day & Expiration Reports

During initial setup, a connection may be configured to distribute [Done for Day](https://developer.chrome.com/derivatives/fix/order-entry#done-for-day-358-1503) execution reports for all the [GTC/GTD](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59) orders that remain open after the trading day is closed. It is also possible to configure the API to distribute the [Expired](https://developer.chrome.com/derivatives/fix/order-entry#expired-order-358-150c) execution reports for the [Day](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59) orders that have expired.