# order entry messages

## NewOrderSingle (35=D)

Used to submit a new spot or perpetual future order.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. Must not exceed 18-31 characters depending on the character range or must be a variant 1 UUIDv4:

18 = All ASCII characters  
20 = Alphanumeric characters and dash (-)  
31 = Hexadecimal characters (0-9, a-f, lowercase and no preceding 0s)  
36 = A UUID in standard format

If using a UUID it must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. 1985ca2d-61ef-49f1-bfce-6c39d8462914). Failure to follow this formatting will result in a reject.

453

NoPartyIDs

int

Y

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID  
If no party is specified the message applies to the default portfolio UUID affiliated with the API key.

```
→ 448
```

PartyID

string

Y

The unique identifier representing the party entry or portfolio that this order originates from. When PartyRole (452) = 24, set PartyID (448) equal to the UUID of the desired portfolio for this order. You can obtain this portfolio UUID in the response of the [List all user portfolios](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/portfolios/list-all-user-portfolios) endpoint.

```
→ 452
```

PartyRole

int

Y

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

18

ExecInst

char

C

The execution instruction flags for the order. Multiple values should be space delimited. Currently supports:

6 = Post only  
E = Close only

Note: Close only orders must reduce the overall position size for a given instrument. Close only orders that increase position size will be rejected.

38

OrderQty

decimal

Y

The amount of the base asset to be transacted.

40

OrdType

char

Y

The type of order for the request which can be:

1 = Market  
2 = Limit  
3 = Stop  
4 = Stop Limit  
O = [Take Profit Stop Loss](https://developer.chrome.com/international-exchange/fix-api/tpsl-orders)

Note: Market (1) orders automatically get converted to aggressively priced limit orders.

44

Price

decimal

C

The limit price for limit orders of the quote asset or USDC for perpetual futures. The decimal precision must fall within the requirements for each market, see the REST API for precision and decimal limits.

54

Side

char

Y

Side of the order, valid options:

1 = Buy  
2 = Sell

55

Symbol

string

Y

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP)

59

TimeInForce

char

Y

Specifies how long the order remains in effect. The following values are supported:

1 = Good Till Cancel (GTC, up to 30 days)  
3 = Immediate or Cancel (IOC)  
4 = Fill or Kill (FOK)  
6 = Good Till Time (GTT, up to 30 days)

126

ExpireTime

utc timestamp

C

Required when TimeInForce (59) is set to GTT (6). Specifies the time when a GTT order expires.  
Time expressed in UTC, microseconds: YYYYMMDD-HH:MM:SS.ssssss

99

StopPx

decimal

C

Specifies the quote price at which the order activates for Stop, Stop Limit and TP/SL order types (40=3, 40=4, or 40=O).

3040

StopLimitPx

decimal

C

Specifies the limit price at which the TP/SL stop leg order is entered (40=O).

8000

SelfTradePreventionStrategy

char

N

The following values specify what to do when two orders submitted by the same Organization/Account attempt to match:

N = Cancel aggressing order  
Q = Cancel both orders  
O = Cancel resting order  
D = Decrement and cancel

Default if not specified is Cancel both orders (Q).

847

TargetStrategy

int

C

Specifies the trading strategy. The following value is supported:

1001 = TWAP

## OrderCancelReplaceRequest (35=G)

Used to modify a parameter of an existing order that is still live on exchange. Only the price and quantity can be modified.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. Must not exceed 18-31 characters depending on the character range or must be a variant 1 UUIDv4:

18 = All ASCII characters  
20 = Alphanumeric characters and dash (-)  
31 = Hexadecimal characters (0-9, a-f, lowercase and no preceding 0s)  
36 = A UUID in standard format

If using a UUID it must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. `1985ca2d-61ef-49f1-bfce-6c39d8462914`). Failure to follow this formatting will result in a reject.

453

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.  
If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents. Currently supports:  
3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

41

OrigClOrdID

string

Y

An identifier matching the ClOrdID from the OrderSingle or OrderCancelReplaceRequest that this request applies to.

38

OrderQty

decimal

N

The amount of the base asset to be transacted, this value can be changed from the order referenced in OrigClOrdId(41)

44

Price

decimal

N

The limit price for limit orders of the quote asset or USDC for perpetual futures. The decimal precision must fall within the requirements for each market, see the REST API for precision and decimal limits. This value can be different from the message that the OrigClOrdID references.

55

Symbol

string

Y

Must match the symbol on the message that the OrigClOrdID references.

99

StopPx

decimal

N

Specifies the quote price at which the order activates for Stop and Stop Limit order types (44=3 or 44=4). Must match the StopPx on the message that OrigClOrdId references.

## OrderCancelRequest (35=F)

Used to cancel an order that is still live on the exchange.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. Must not exceed 18-31 characters depending on the character range or must be a variant 1 UUIDv4:

18 = All ASCII characters  
20 = Alphanumeric characters and dash (-)  
31 = Hexadecimal characters (0-9, a-f, lowercase and no preceding 0s)  
36 = A UUID in standard format

If using a UUID it must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. 1985ca2d-61ef-49f1-bfce-6c39d8462914). Failure to follow this formatting will result in a reject.

453

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.  
If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

41

OrigClOrdID

string

Y

An identifier matching the ClOrdID from the OrderSingle or OrderCancelReplaceRequest that this request applies to.

55

Symbol

string

Y

Must match the message that the OrigClOrdID references.

## OrderMassCancelRequest (35=q)

Used to cancel multiple orders that is still live on the exchange.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. Must not exceed 18-31 characters depending on the character range or must be a variant 1 UUIDv4:

18 = All ASCII characters  
20 = Alphanumeric characters and dash (-)  
31 = Hexadecimal characters (0-9, a-f, lowercase and no preceding 0s)  
36 = A UUID in standard format

If using a UUID it must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. 1985ca2d-61ef-49f1-bfce-6c39d8462914). Failure to follow this formatting will result in a reject.

54

side

string

Y

Side of the order, valid options:

1 = Buy  
2 = Sell. If `Side` is not provided, the request cancels all orders in that portfolio regardless of side.

55

Symbol

string

Y

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP). If `Symbol` is not provided, the request cancels all orders in that portfolio regardless of symbol.

453

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.  
If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

```
→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

If populated it must match the value on the referenced order specified in the OrigClOrdID(41).

## ExecutionReport (35=8)

Sent by the exchange to provide an update on a submitted order.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages relating to this request.

41

OrigClOrdID

string

C

An identifier matching the ClOrdID from the OrderSingle or OrderCancelReplaceRequest if this execution report belongs to an OrderCancelRequest or an OrderCancelReplaceRequest.

880

TrdMatchID

string

C

A unique ID provided on trade messages that is affiliated with the match. The same ID is provided on both sides of the trade which can aid in self trade detection.

17

ExecID

string

Y

A globally unique 64bit monotonically increasing integer identifier specified by Coinbase International that identifies an event generated in the system. Please reference this identifier when speaking with Coinbase International Support.

20

ExecTransType

char

Y

Indicates whether this message is original or an amendment. The potential values include:

0 = New  
1 = Cancel  
2 = Correct

All original messages get marked as New. Trade busts get marked as Cancel and trade corrections get marked as Correct. The trade bust and correct functionality will get implemented at a future date.

37

OrderID

string

Y

A unique identifier assigned by the exchange for the order. Please use this ID when referencing the order for support.

453

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.

```
→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

```
→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

150

ExecType

char

Y

The type of execution report indicating what happened to the order.

Potential values include:  
0 = New  
1 = Partial Fill  
2 = Fill  
4 = Canceled  
5 = Replaced  
8 = Rejected  
C = Expired  
L = Stop Triggered

39

OrdStatus

char

Y

Identifies the current state of the order.

The potential values include:  
0 = New  
1 = Partially Filled  
2 = Filled  
4 = Canceled  
5 = Replaced  
8 = Rejected  
C = Expired

103

OrdRejReason

int

C

Used for reject messages, a set of code based references for common types of rejects.

The potential values include:  
0 = Unknown  
1 = Invalid order parameter  
2 = Price violation

55

Symbol

string

Y

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP)

54

Side

char

Y

Side of the order, valid options:

1 = Buy  
2 = Sell

38

OrderQty

decimal

C

Required for limit orders. The quantity specified on the order.

40

OrdType

char

Y

The order type specified on the order.

44

Price

decimal

C

Required for limit orders. Price specified on the order.

99

StopPx

decimal

C

Required for stop orders. The stop price specified on the order.

3040

StopLimitPx

decimal

C

Required for TP/SL orders. The stop leg limit price specified on the order.

15

Currency

string

Y

The currency used for prices and commission values reflected in this message.

59

TimeInForce

char

N

Time in force specified on the order. The following values are supported:

1 = Good Till Cancel (GTC, up to 30 days)  
3 = Immediate or Cancel (IOC)  
4 = Fill or Kill (FOK)  
6 = Good Till Time (GTT, up to 30 days)

126

ExpireTime

utc timestamp

C

Required when TimeInForce (59) is set to GTT (6). Specifies the time when a GTT order expires.

18

ExecInst

char

C

The execution instruction flags for the order. Multiple values should be space delimited. Currently supports:

6 = Post only  
E = Close only

32

LastQty

decimal

C

Required for trades (150=1 or 150=2). Represents the total amount of the order’s specified OrderQty that traded.

31

LastPx

decimal

C

Required for trades (150=1 or 150=2). Price in the currency (tag 15) at which the LastQty (tag 32) amount traded.

151

LeavesQty

decimal

Y

The amount of the OrderQty that remains open for further execution.

14

CumQty

decimal

Y

The total amount of the OrderQty that has traded so far including the LastQty amount if applicable (i.e. if 150=1 or 150=2).

6

AvgPx

decimal

Y

The average price in Currency (tag 15) of the CumQty (tag 14) traded at so far including the current trade if applicable (i.e. if 150=1 or 150=2).

60

TransactTime

utc timestamp

Y

The original time that this execution report occurred (useful for replays).

58

Text

string

N

A text message that gets populated when the context of the execution report requires explanation (e.g. rejects or unsolicited cancels).

851

LastLiquidityInd

int

C

Provided for trades (150=1 or 150=2). Provides context on how the order traded on the exchange for use in reference to trading fees.

Values include:  
1 = Added liquidity  
2 = Removed liquidity

136

NoMiscFees

int

C

Used for trades (150=1 or 150=2). Provides information on the fees affiliated with the transaction. If specified and the value is > 0 it marks the beginning of a repeating group of the following 3 tags.

```
→ 137
```

MiscFeeAmt

decimal

C

The total fee amount in units of MiscFeeCurr

```
→ 138
```

MiscFeeCurr

string

C

The currency the fee amount is charged in (e.g. USDC)

```
→ 139
```

MiscFeeType

string

C

Specifies the type of fee. Values may include:

4 = Exchange fees  
7 = Other  
14 = Asset lending

8000

SelfTradePreventionStrategy

char

Y

The self trade prevention strategy associated with the order.

The following values specify what to do when two orders submitted by the same Organization/Account attempt to match:  
N = Cancel aggressing order  
Q = Cancel both orders  
O = Cancel resting order  
D = Decrement and cancel smaller order

Default if not specified is Cancel both orders (Q).

847

TargetStrategy

int

C

Specifies the trading strategy. The following value is supported:

1001 = TWAP

## OrderMassCancelReport (35=r)

Sent by the exchange to provide an update on a submitted order mass cancel.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages relating to this request.

1369

MassActionReportID

string

Y

A globally unique 64bit monotonically increasing integer identifier specified by Coinbase International that identifies an event generated in the system. Please reference this identifier when speaking with Coinbase International Support.

55

Symbol

string

N

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP)

54

Side

char

N

Side of the order, valid options:

1 = Buy  
2 = Sell

531

MassCancelResponse

char

Y

Identifies the response of order mass cancel request. For accepted requests, the value will be `CANCEL_ALL_ORDERS (7)`. For rejected requests, the value will be `CANCEL_REQUEST_REJECTED (0)` and `MassCancelRejectReason` will be provided.

The potential values include:  
0 = Cancel Request Rejected  
1 = Cancel orders for a security  
2 = Cancel orders for an Underlying security  
3 = Cancel orders for a Product  
4 = Cancel orders for a CFICode  
5 = Cancel orders for a SecurityType  
6 = Cancel orders for a trading session  
7 = Cancel all orders

532

MassCancelRejectReason

int

N

Identifies the rejected reason of order mass cancel request. For rejected requests, the value will be `MASS_CANCEL_NOT_SUPPORTED (0)`.

The potential values include:  
0 = Mass Cancel Not Supported  
1 = Invalid or unknown Security  
2 = Invalid or unknown Underlying security  
3 = Invalid or unknown Product  
4 = Invalid or unknown CFICode  
5 = Invalid or unknown SecurityType  
6 = Invalid or unknown trading session  
99 = Other

533

TotalAffectedOrders

decimal

N

Total number of orders affected by order mass cancel request

60

TransactTime

utc timestamp

N

The original time that this order mass cancel report occurred

58

Text

string

N

A message providing more detail about the order mass cancel reject

## OrderCancelReject (35=9)

Sent by the exchange when a cancel or cancel-replace request fails.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request.

41

OrigClOrdID

string

Y

An identifier matching the ClOrdID from the OrderSingle or OrderCancelReplaceRequest that this request applies to.

37

OrderID

string

Y

An identifier specified by Coinbase International that uniquely identifies the order in the system. Please reference this identifier when speaking with Coinbase International Support. If the cancel or cancel-replace request was unable to find the OrigClOrdID specified in the message this value contains the text `NONE`.

58

Text

string

N

A message providing more detail about the cancel reject

102

CxlRejReason

int

N

A code to identify common reasons why for the cancel reject. Potential values include:

0 = Too late to cancel  
1 = Unknown order  
3 = Order already pending cancel or cancel replace

434

CxlRejResponseTo

char

Y

Indicates whether this message is in response to a cancel request or cancel replace request.

Values include:  
1 = Order Cancel Request  
2 = Order Cancel/Replace Request

## BusinessMessageReject (35=j)

An application level reject message sent when the FIX session can’t process a message.

Tag

Name

Type

Required

Description

45

RefSeqNum

int

N

The MsgSeqNum of the referenced message that was rejected.

372

RefMsgType

int

Y

The message type that this reject message applies to.

Supported values include:  
A = Logon  
0 = Heartbeat  
1 = TestRequest  
D = NewOrderSingle  
F = OrderCancelRequest  
G = OrderCancelReplaceRequest  
F1 = LastExecIDRequest  
F3 = EventResendRequest  
x = SecurityListRequest  
V = MarketDataRequest

379

BusinessRejectRefID

string

N

An identifier pointing to the message referenced for this reject. In the context of an order request this points to the ClOrdId.

380

BusinessRejectReason

int

N

A code to quickly identify common reasons for a reject.

Values could include:  
0 = Other  
1 = Unknown ID  
2 = Unknown asset or instrument  
3 = Unsupported message type  
4 = Application not available  
5 = Conditionally required field missing  
6 = Not authorized  
8 = Throttle limit exceeded  
9 = Throttle limit exceeded, session will be disconnected

58

Text

string

N

A message explaining why the message was rejected.

## TradeCaptureReport (35=AE)

Sent by the exchange when a trade occurs from an automated liquidation or liquidation triggered position transfer.

Tag

Name

Type

Required

Description

828

TrdType

int

Y

Indicates the type of trade for the liquidation:

0 = Regular trade  
3 = Transfer

A regular trade occurs when the exchange’s liquidation engine automatically places orders in the open market. A transfer happens when the account gets closed out and the position goes to another participant that specializes in closing out liquidated positions.

830

TransferReason

string

C

Used when TydType (828) = Transfer (3) to indicate the transfer context.

Values include:  
LIQUIDATED  
ASSIGNED

The LIQUIDATED value indicates a position getting transferred out of the portfolio. The ASSIGNED value indicates a position getting transferred into the portfolio (only applicable for LSP portfolios).

880

TrdMatchID

string

Y

A unique ID provided on trade messages that is affiliated with the match. The same ID is provided on both sides of the trade which can aid in self trade detection.

17

ExecID

string

Y

A globally unique 64bit monotonically increasing integer identifier specified by Coinbase International that identifies an event generated in the system. Please reference this identifier when speaking with Coinbase International Support.

820

TradeLinkID

string

Y

An identifier used to track all trades linked with the current phase of the liquidation process. Changes each time the portfolio gets flagged for handling by liquidation engine or if the portfolio gets transferred to liquidation specialists.

55

Symbol

string

Y

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP)

32

LastQty

decimal

C

Represents the total amount of the asset that traded.

31

LastPx

decimal

C

Price in the currency (tag 15) at which the LastQty (tag 32) amount traded.

60

TransactTime

utc timestamp

Y

The original time that this execution report occurred (useful for replays).

552

NoSides

int

Y

Marks the beginning of the repeating group for sides and will always be set to 1.

```
→ 54
```

Side

char

Y

Side of the trade, valid options:

1 = Buy  
2 = Sell

```
→ 453
```

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.

```
→→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

```
→→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents.

Currently supports:  
3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

```
→ 136
```

NoMiscFees

int

C

Provides information on the fees affiliated with the transaction. If specified and the value is > 0 it marks the beginning of a repeating group of the following 3 tags.

```
→→ 137
```

MiscFeeAmt

decimal

C

The total fee amount in units of MiscFeeCurr

```
→→ 138
```

MiscFeeCurr

string

C

The currency the fee amount is charged in (e.g. USDC)

```
→→ 139
```

MiscFeeType

string

C

Specifies the type of fee.

Values may include:  
4 = Exchange fees  
7 = Other  
14 = Asset lending

## PreFillRequest (35=F6)

Used to subscribe to the PreFills channel to receive PreFillReports for portfolios scoped to your API key.

Tag

Name

Type

Required

Description

22007

PreFillRequestID

string

Y

Client generated ID for this request

A successful subscription is indicated by a PreFillRequestSuccess(35=F7) message with the **PreFillRequestID** field set to the same **PreFillRequestID** supplied in the PreFillRequest. An unsuccessful subscription is indicated by a BusinessMessageReject with the **BusinessRejectRefID** field set to the **PreFillRequestID** supplied in the PreFillRequest.

## PreFillReport (35=F8)

Sent by the exchange on every fill on an order to PreFills subscribers. These are not aggregated across multiple fills. Each fill has a separate PreFillReport.

Tag

Name

Type

Required

Description

11

ClOrdID

string

Y

An identifier specified by the sender to uniquely identify other messages relating to this request.

880

TrdMatchID

string

Y

A unique ID provided on trade messages that is affiliated with the match. The same ID is provided on both sides of the trade which can aid in self trade detection.

37

OrderID

string

Y

A unique identifier assigned by the exchange for the order. Please use this ID when referencing the order for support.

453

NoPartyIDs

int

Y

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.

```
→ 448
```

PartyID

string

Y

The unique identifier used to represent the party entry.

```
→ 452
```

PartyRole

int

Y

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).

55

Symbol

string

Y

Symbol of the instrument being traded (e.g. BTC-USDC or BTC-PERP)

54

Side

char

Y

Side of the order, valid options:

1 = Buy  
2 = Sell

38

OrderQty

decimal

Y

The quantity specified on the order.

44

Price

decimal

N

Price specified on the order, if any.

32

LastQty

decimal

Y

Quantity bought or sold of this (last) fill.

31

LastPx

decimal

Y

Represents the price of this (last) fill.

60

TransactTime

utc timestamp

Y

The original time that this execution report occurred (useful for replays).

851

LastLiquidityInd

int

Y

Provided for trades (150=1 or 150=2). Provides context on how the order traded on the exchange for use in reference to trading fees.

Values include:  
1 = Added liquidity  
2 = Removed liquidity

## RFQ Request (35=AH)

Clients must send an RFQ Request message (35=AH) after each successful Logon message (35=A) for any session in which they are interested in receiving Quote Requests.

-   If the session submitting this request is not approved by Coinbase for participating in the RFQ program, this request is rejected with a Business Message Reject (j).If this request is acknowledged, and no symbol is specified, this session receives all Quote requests (all assets).

Tag

Name

Type

Required

Notes

644

RFQReqID

UUID

Y

Unique identifier for RFQ Request

## Quote Request (35=R)

Sent by the exchange to signal the start of the RFQ process. Coinbase sends a Quote Request to Liquidity Providers (LPs) on behalf of a customer looking to participate in an RFQ trade. LPs respond to a Quote Request with a Quote (S).

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ

146

NoRelatedSym

NumInGroup

Y

Always 1

55

Symbol

String32

Y

Example: `BTC-PERP`

38

OrderQty

Float64

Y

The quantity the customer is looking to trade via RFQ

62

ValidUntilTime

UTCTimestamp

Y

The time by which quotes must be submitted for the RFQ

126

ExpireTime

UTCTimestamp

Y

The time by which the RFQ expires if there is no match

136

NoMiscFees

NumInGroup

Y

Always 1

137

MiscFeeAmt

Float64

Y

Fee as a percentage that Liquidity Providers are charged on a winning Quote.

139

MiscFeeType

Int32

Y

Always 4 = Exchange Fees

891

MiscFeeBasis

Int32

Y

Always 2 = Percentage

528

OrderCapacity

Char

Y

A = Agency (default)  
C = Corporate

## Quote (35=S)

Quote (S) messages are submitted by Liquidity Providers (LP) in response to a Quote Request (R) in order to participate in the competitive RFQ auction. Quotes can be submitted as either a one-way or two-way quote, and must be received by the `ValidUntilTime (62)` specified in the Quote Request. Only one side is traded if the Liquidity Provider wins the RFQ.

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ echoed from Quote Request

117

QuoteID

UUID

Y

Unique identifier for Quote specified by Liquidity Provider

55

Symbol

String32

Y

Example: `BTC-PERP`

132

BidPx

Float64

C

Required if submitting bid

133

OfferPx

Float64

C

Required if submitting offer

134

BidSize

String32

C

Required if submitting bid. Must match `OrderQty (38)` from Quote Request

135

OfferSize

String32

C

Required if submitting offer. Must match `OrderQty (38)` from Quote Request

453

NoPartyIDs

Int32

C

Marks the beginning of the Parties repeating group component. A max of 1 party is supported to specify the portfolio UUID. Supported values: 1 = Specify portfolio UUID If no party is specified, the message applies to the default portfolio UUID affiliated with the API key

448

PartyID

String32

C

The unique identifier used to represent the party entry or portfolio that this order generates from. When PartyRole(452)=24, set PartyID(448) equal to the UUID of the desired portfolio for this order.

452

PartyRole

Int32

C

The type of party entry that the PartyID (448) value represents. Currently supports: 24 = Customer account The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message.

## Quote Status Report (35=AI)

Sent by the exchange to Liquidity Providers with Quote statuses and expired Quote Requests.

-   If the Quote (S) is rejected b/c validation checks failed or it was sent too late, the response to the quoter is `297=5` (QuoteStatus = Rejected).
-   If the Quote (S) is accepted and eligible to participate in an RFQ auction, the response to the quoter is `297=16` (QuoteStatus = Active).
-   If the Quote (S) is accepted but not selected for execution, the response to the quoter is `297=17` (QuoteStatus = Canceled).
-   If the Quote (S) is accepted and selected for execution, the response to the quoter is `297=19` (QuoteStatus = Pending End Trade), followed by Execution Report - Filled.
-   If the Quote Request (r) is unmatched by `ExpireTime (126)` on the Quote Request, `297=7` (QuoteStatus = Expired) is broadcast to all LPs.

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ echoed from Quote Request

117

QuoteID

UUID

Y

Unique identifier for Quote specified

55

Symbol

String32

Y

Example: `BTC-AVAX`

54

Side

Char

C

Buy: `54=1`, Sell: `54=2`  
Specified if QuoteStatus=Pending End Trade (`297=19`)

38

OrderQty

Float64

Y

Echoed from Quote Request

132

BidPx

Float64

C

Echoed from Quote

133

OfferPx

Float64

C

Echoed from Quote

134

BidSize

Float64

C

Echoed from Quote

135

OfferSize

Float64

C

Echoed from Quote

62

ValidUntilTime

UTCTimestamp

Y

Echoed from Quote Request

126

ExpireTime

UTCTimestamp

Y

Echoed from Quote Request

297

QuoteStatus

Int32

Y

`5` = **Rejected**: Quote failed validation checks or was sent too late  
`7` = **Expired**: Quote Request expired w/no match  
`16` = **Active**: Quote was acknowledged  
`17` = **Canceled**: Quote not selected b/c LP did not win auction or had insufficient funds  
`19` = **Pending End Trade**: Quote selected for execution

58

Text

String

C

Reason the Quote was rejected if QuoteStatus=5

17

ExecID

String

Y

A globally unique 64bit monotonically increasing integer identifier specified by Coinbase International that identifies an event generated in the system. Equivalent to ExecID in ExecutionReport (9)