# drop copy

The FIX Drop Copy API provides execution reports for orders placed through other FIX sessions or the REST API. This service allows clients to receive real-time execution updates for all their trading activity in a single consolidated feed. About this API:

-   **Baseline**: [FIX 5.0 SP2 specification](https://www.onixs.biz/fix-dictionary/5.0.sp2/index.html).
-   **Environments**: Production, Sandbox

You can connect with the same authentication as our existing FIX order entry system. Connectivity is limited to a single connection per API key—Order Entry [rate limits](https://developer.chrome.com/exchange/fix-api/rate-limits) *do not apply* (connections, rps, burst rps).

A standard header must be present at the start of every message in both directions.

Tag

FieldName

Type

Required

Notes

8

BeginString

String

Y

Must be `FIXT.1.1`

49

SenderCompID

String

Y

Client API key (on messages from the client)

56

TargetCompID

String

Y

Must be `Coinbase` (on messages from client)

52

SendingTime

UTCTimestamp

Y

UTC time down to millisecond resolution in the format `YYYYMMDD-HH:MM:SS.sss`

## Logon (35=A)

Tag

FieldName

Type

Required

Notes

34

MsgSeqNum

Int

Y

Must be `1`

98

EncryptMethod

Int

Y

Must be `0` (None)

108

HeartBtInt

Int

Y

Heartbeat interval is capped at 300s, defaults to 10s

141

ResetSeqNumFlag

Boolean

Y

Resets the sequence number. Can be `Y`/`N`

553

Username

String

Y

Client API Key

554

Password

String

Y

Client API passphrase

95

RawDataLength

Int

Y

Number of bytes in RawData field

96

RawData

String

Y

[Client message signature](https://developer.chrome.com/exchange/rest-api/authentication#signing-a-message)

1137

DefaultApplVerID

String

Y

Must be `9` (FIX 5.0 SP2)

9406

DropCopyFlag

Char

Y

Must be `Y` for Drop Copy sessions

## Execution Report (35=8)

This message is sent by Coinbase Exchange to provide execution reports for all orders placed through any FIX session or REST API associated with your API key. Drop Copy sessions receive the same execution reports as order entry sessions, but without the ability to place new orders.

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

The client order ID of the order.

37

OrderID

UUID

Y

A unique identifier assigned by the exchange for the order.

41

OrigClOrdID

String

C

The client order ID of the parent order for [Order Cancel/Replace Requests](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelreplacerequest-35g).

6

AvgPx

Decimal

C

The volume-weighted average price of all fills on the order.

14

CumQty

Decimal

C

The cumulative base quantity (e.g. in BTC) filled on the order.

151

LeavesQty

Decimal

C

The remaining base quantity (e.g. in BTC) on the order.

Not sent for market orders that were sent using `CashOrderQty`.

17

ExecID

UUID

Y

ID identifying this execution report.

39

OrdStatus

Char

Y

**Supported values:**  
`0` = New  
`1` = Partially Filled  
`2` = Filled  
`4` = Canceled  
`5` = Replaced  
`8` = Rejected  
`C` = Expired (For IOC expirations)

150

ExecType

Char

Y

**Supported values:**  
`0` = New  
`4` = Canceled  
`5` = Replaced  
`8` = Rejected  
`C` = Expired (For IOC expirations)  
`D` = Restated (in cases where orders are partially canceled unsolicited due to self-trade prevention)  
`F` = Trade  
`I` = Order Status (in response to Order Status Requests)

55

Symbol

String

Y

The symbol of the order (e.g. BTC-USD).

54

Side

Char

C

**Supported values:**  
`1` = Buy  
`2` = Sell

40

OrdType

Char

C

**Supported values:**  
`1` = Market  
`2` = Limit  
`4` = Stop Limit

32

LastQty

Char

C

The base quantity (e.g. in BTC) of the most recent fill on the order when `ExecType` is `F` (Trade).

31

LastPx

Decimal

C

The price of the most recent fill on the order when `ExecType` is `F` (Trade).

44

Price

Decimal

C

The limit price of the order.

38

OrderQty

Decimal

C

The base quantity (e.g. in BTC) of the order.

152

CashOrderQty

Decimal

C

The quote quantity (e.g. in USD) of the order.

For market orders that were submitted using `CashOrderQty` instead of `OrderQty`, this is the remaining quote quantity of the order.

58

Text

String

N

Description of why the order was rejected, canceled, or expired.

60

TransactTime

UTCTimestamp

Y

Matching engine timestamp.

103

OrdRejReason

Int

N

**Supported values:**  
`0` = Broker  
`1` = Unknown Symbol  
`5` = Unknown Order

378

ExecRestatementReason

Int

N

**Supported values:**  
`5` = Partial Decline of `OrderQty` (in cases where orders are partially canceled unsolicited due to self-trade prevention).

1003

TradeID

String

C

Trade ID for a given fill used for reporting.

1057

AggressorIndicator

Boolean

C

**Supported values:**  
`Y` = Taker (if aggressor or auction trade)  
`N` = Maker

59

TimeInForce

Char

C

**Supported values:**  
`1` = GTC  
`3` = IOC  
`4` = FOK  
`6` = GTD

99

StopPx

Decimal

C

For stop-limit orders, the stop price of the order.

1109

TriggerPriceDirection

Char

N

For stop-limit orders.

**Supported values:**  
`U` = Trigger if market price goes UP to or through `StopPx`  
`D` = Trigger if market price goes DOWN to or through `StopPx`

18

ExecInst

Char

N

**Supported values:**  
`A` = Add Liquidity Only.

7928

SelfTradeType

Char

N

**Supported values:**  
`D` = Decrement and Cancel (default if not specified)  
`O` = Cancel Oldest (resting order)  
`N` = Cancel Newest (aggressing order)  
`B` = Cancel Both

126

ExpireTime

UTCTimestamp

C

Timestamp at which a GTD order would expire.

1430

VenueType

Char

C

Indicates an RFQ fill. **Supported values:**  
`E` = ELECTRONIC\_EXCHANGE  
`N` = QUOTE\_NEGOTIATION

136

NoMiscFees

Int

C

Repeating group for fees charged.

Always `1` on an order partial fill or fill.

\=>137

MiscFeeAmt

Decimal

C

See `MiscFeeBasis`.

\=>138

MiscFeeCurr

String

C

The currency that the fee is charged in.

\=>139

MiscFeeType

String

C

**Always:**  
`4` = Exchange Fees.

\=>891

MiscFeeBasis

Int

C

**Supported values:**  
`0` = Absolute (`MiscFeeAmt` is in `MiscFeeCurr` terms)  
`2` = Percentage (`MiscFeeAmt` should be multiplied by the fill quantity in `MiscFeeCurr` terms to calculate the fee in `MiscFeeCurr` terms)

## Administrative Messages

Drop Copy sessions use the same administrative messages as order entry sessions:

-   [Heartbeat (35=0)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#heartbeat-350)
-   [TestRequest (35=1)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#testrequest-351)
-   [Logout (35=5)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#logout-355)