# drop copy

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

Indicates whether this message is original or an amendment.

The potential values include:  
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

The type of party entry that the PartyID (448) value represents.

Currently supports:  
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
6 = Pending Cancel  
8 = Rejected  
A = Pending New  
C = Expired  
E = Pending Replace  
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
6 = Pending Cancel  
8 = Rejected  
A = Pending New  
E = Pending Replace  
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

The total amount of the OrderQty that has traded so far including the LastQty amount if applicable (i.e., if 150=1 or 150=2).

6

AvgPx

decimal

Y

The average price in Currency (tag 15) of the CumQty (tag 14) traded at so far including the current trade if applicable (i.e., if 150=1 or 150=2).

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
14 = Security lending

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

Specifies the type of fee. Values may include:

4 = Exchange fees  
7 = Other  
14 = Asset lending

## Quote Status Report (35=AI)

Sent to Liquidity Providers with Quote statuses and expired Quote Requests.

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