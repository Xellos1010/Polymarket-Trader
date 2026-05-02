# order entry messages

The baseline specification for this API is [FIX 4.2](https://www.fixtrading.org/standards/fix-4-2/). Below, we’ve noted the places in which the FIX API for Coinbase Prime extends (or clarifies) the FIX spec. For example, there are custom tags with a four-digit number range, as allowed by the standard, which are unique to Prime. For standard header configuration requirements, see [Administrative Messages](https://developer.chrome.com/prime/fix-api/admin-messages).

## New Order Single (D)

Sent by the client to enter an order. Not every tag is required for every order — it depends on the target strategy used. See the table below for more information.

Tag

Name

Description

Notes

1

Account

The portfolio ID

11

ClOrdID

A string selected by client to identify the order

ClOrdID (11) idempotency will be enforced on open & closed orders for a rolling 24hr window.

18

ExecInst

Must be `A` for Post Only

TimeInForce must be `6` (GTD) or `1` (GTC)

38

OrderQty

Order size in base units (e.g., BTC). Either this or CashOrderQty must be supplied.

40

OrdType

Order type. Must match `TargetStrategy`

See [OrdType Values table](#ordtype-values)

44

Px

Indicates the price of the order

Required for Limit, TWAP, and PEG orders

54

Side

Must be `1` to buy or `2` to sell

55

Symbol

The product to be traded (e.g., `BTC-USD`)

59

TimeInForce

A valid TimeInForce value; see the table below for a list.

Must match TargetStrategy

99

StopPx

Stop price for Stop Limit order

Specifies the stop price at which the order activates. The order is activated if the last trade price on Coinbase Exchange crosses the stop price specified on the order.

126

ExpireTime

Represents the time and date of order expiration

Required for TWAP/VWAP orders and Limit GTD orders, unless ParticipationRate is specified (TWAP/VWAP only)

152

CashOrderQty

Order size in quote units (e.g., USD)

Either this or OrderQty must be supplied.

168

EffectiveTime

Represents the start time

Required for TWAP/VWAP orders

210

MaxShow

Maximum quantity within an order to be shown to other customers (Display Size)

Only available for LIMIT orders

211

PegOffsetValue

The offset value in price, basis points, or cumulative depth units that offset the peg price

Required for PEG orders

836

PegOffsetType

Type of peg offset: `0` = Price, `1` = Basis Points, `4` = Cumulative depth in base units

Required for PEG orders

847

TargetStrategy

The target strategy of the order to place; see the table below for a list.

Requires `ExpireTime` and `EffectiveTime` for TWAP/VWAP orders. Must be `SL` and requires StopPx for Stop Limit orders. Must be `P` for PEG orders.

849

ParticipationRate

Represents the estimated percent of volume for TWAP/VWAP order types.

Can be used instead of ExpireTime which it computes based on historical participation of volume rate.

8007

WigLevel

”Would if Good” level - the target price to actively fill the remaining balance of a pegged order

Optional for PEG orders

8999

IsRaiseExact

Y or N, is this a raise exact order. If Y, the asset amount sold is adjusted so the total received after fees equals the input size.

Optional, defaults to N. Supported for SELL, Size In Quote orders only.

### TargetStrategy Values

Value

Description

OrdType

[TimeInForce](#timeinforce-values)

`L`

Limit order

Must be `2` (Limit)

-   Must be `1` (GTC) or `3` (IOC) or `4` (FOK) or `6` (GTD)
-   Limit FOK orders are currently only routed to the Coinbase Exchange
-   `44` (price) must also be provided

`M`

Market order

Must be `1` (Market)

Must be `3` (IOC)

`P`

Adaptive Peg order

Must be `P` (Pegged)

Must be `1` (GTC) or `6` (GTD); `44` (price), `836` (PegOffsetType), and `211` (PegOffsetValue) must also be provided

`T`

TWAP order

Must be `2` (Limit)

Must be `6` (GTD); `44` (price) must also be provided

`V`

VWAP order

Must be `2` (Limit)

Must be `6` (GTD); `44` (price) must also be provided

`SL`

Stop Limit order

Must be `2` (Limit)

Must be `1` (GTC) or `6` (GTD); `44` (price) and `99` (StopPx) must also be provided

### OrdType Values

Value

Description

`1`

Market

`2`

Limit

`D`

Previously Quoted

`P`

Pegged

### TimeInForce Values

Value

Description

`1`

Good Till Cancel (GTC)

`3`

Immediate or Cancel (IOC)

`4`

Fill or Kill (FOK)

`6`

Good Till Date (GTD)

## RFQ

### Quote Request (R)

A Quote Request (R) is the start of the RFQ process. Coinbase Prime sends a Quote Request to Liquidity Providers (LPs) on behalf of a customer looking to participate in an RFQ trade. LPs respond to a Quote Request (R) with a Quote (S).

Tag

Name

Description

131

QuoteReqID

A string selected by client to identify the quote

1

Account

The portfolio id

54

Side

Must be `1` to buy and `2` to sell

55

Symbol

A product pair (e.g. `BTC-USD`)

38

OrderQty

Order size in base units (e.g. `BTC`)

40

OrdType

Order type. Must be `2` (i.e. Limit)

44

Price

Indicates the limit price of the quote

59

TimeInForce

Time in force for the order. Must be `4` (i.e. FOK)

### Quote (S)

Quote (S) messages are submitted by Liquidity Providers (LP) in response to a [Quote Request (R)](#quote-request-r) in order to participate in the competitive RFQ auction.

Tag

Name

Description

117

QuoteID

A Coinbase generated identifier for the quote

131

QuoteReqID

A string selected by client to identify the quote

1

Account

The portfolio id

55

Symbol

The product pair from Quote Request <R>

132

BidPx

Quoted price. Only present in case of Sells

133

OfferPx

Quoted price. Only present in case of Buys

134

BidSize

Quoted size. Only present in case of Sells

135

OfferSize

Quoted size. Only present in case of Buys

40

OrdType

Order type. Must be 2 (i.e. Limit)

62

ValidUntilTime

Indicates expiration time of the quote (always expressed in UTC)

### Accept quote - New Order Single (D)

Sent by the client in order to accept a Quote (S). Must be accepted before expiration time indicated on Tag 62 of [Quote (S)](#quote-s). Refer to [New Order Single (D)](#new-order-single-d) for standard fields of submitting an order.

Tag

Name

Description

\[…\]

\[…\]

Standard fields for NewOrderSingle <D>

117

QuoteID

The quote id on tag 117 from Quote <S>

40

OrdType

Should be `D` (i.e. Previously Quoted)

847

TargetStrategy

Should be `R` (i.e. RFQ, coinbase’s custom value)

59

TimeInForce

Should be `4` (i.e. FOK)

44

Price

The price on tag 132/133 from Quote <S>

### Quote Acknowledgment (b)

Sent by the server in case the [Quote Requests (R)](#quote-request-r) fails.

Tag

Name

Description

117

QuoteID

A Coinbase generated identifier for the quote (optional)

131

QuoteReqID

A string selected by client to identify the quote

1

Account

The portfolio id

55

Symbol

The product pair from Quote Request <R>

297

QuoteAckStatus

Will be always `5` (i.e. Rejected)

300

QuoteRejectReason

See [QuoteRejectReason table](#quoterejectreason)

58

Text

Human-readable description of the error

#### QuoteRejectReason

Possible values for QuoteRejectReason (see [Quote Acknowledgment (b)](#quote-acknowledgment-b)).

Value

Description

1

Unknown symbol

2

Exchange closed

3

Quote Request exceeds limit

6

Duplicate Quote

8

Invalid price

99

Other

## Order Cancel Request (F)

Sent by the client to cancel an order.

Tag

Name

Description

Notes

1

Account

The portfolio ID

11

ClOrdId

ClOrdId identifying this cancel request

37

OrderID

OrderID assigned by Coinbase (available in any of the Execution Report messages)

38

OrderQty

Accepted order quantity

You must supply this tag or CashOrderQty (depending on whichever you originally submitted)

41

OrigClOrdID

ClOrdID from the New Order Single

You must also supply an OrderID

54

Side

Must be `1` to buy or `2` to sell (depending on whichever you originally submitted)

55

Symbol

The product from the original order (e.g., `BTC-USD`)

152

CashOrderQty

Order size in quote units (e.g., `USD`)

You must supply this tag or OrderQty (depending on which you submitted)

## Order Cancel/Replace Request (G)

The Order Cancel/Replace Request message (`MsgType=G`) (a.k.a. Order Modification Request) is used to amend replaceable [fields](https://help.coinbase.com/en/prime/trading-and-funding/editing-orders-on-prime) on open **LIMIT**, **STOP-LIMIT**, **TWAP**, and **VWAP** orders. Orders are modified with “in-flight mitigation” meaning any partially filled quantity on the original order is carried over to the edited order and is reflected in the edited order’s remaining quantity `LeavesQty (151)`.

Tag

Req

Name

Description

37

Y

OrderID

The unique identifier of the order

41

Y

OrigClOrdID

An identifier matching the `ClOrdID` from the [NewOrderSingle](#new-order-single-d) or [OrderCancelReplaceRequest](#order-cancel-request-f) that this request applies to

11

Y

ClOrdID

Unique identifier of the replacement order

55

Y

Symbol

Must match original symbol (e.g., `BTC-USD`)

54

Y

Side

Must match original side

40

Y

OrdType

Must match original order type

60

Y

TransactTime

Time this order request was initiated/released by the trader or trading system

21

Y

HandlInst

Must be `1`

38

C

OrderQty

Total intended order quantity (including the amount already executed on this order)

152

C

CashOrderQty

Total intended order quantity in quote units (including the amount already executed on this order). Either this or `OrderQty` must be supplied.

44

Y

Price

Limit price

99

C

StopPX

Stop price for stop limit

126

C

ExpireTime

Represents the time and date of order expiration

210

C

MaxShow

Display size

#### Usage Notes

-   Identifiers: Used for sequencing edits.
    -   `OrigClOrdID (41)` – The identifier of the current active order you intend to modify. This must match the existing order’s `ClOrdID (11)`.
    -   `ClOrdID (11)` – The new identifier representing the edited order’s state. This value must be unique and **cannot match** `OrigClOrdID (41)`.
-   Execution Reports: Each modification triggers `Execution Report (MsgType=8)` messages with the following `ExecType (150)` values:
    -   `E` – **Pending Replace** (acknowledgment that the modification is being processed)
    -   `5` – **Replaced** (confirmation that the modification has been applied)
-   Editing TWAP orders: We strongly recommend against aggressively increasing the order amount or significantly shortening the duration of active TWAP orders. The TWAP algorithm will adopt the new instructions and may execute more aggressively to catch up to the revised parameters.
-   Edit limit: A single order may be edited up to **25 times**.

## Order Status Request (H)

Sent by the client to obtain information about pending and completed orders.

Tag

Name

Description

Notes

11

ClOrdID

ClOrdID of the order to be sent back

37

OrderID

OrderID of the order to be sent back

Required

54

Side

Must be 1 to buy or 2 to sell

55

Symbol

The product to be traded (e.g., BTC-USD)

### Python Example

```
def get_order_status(self, order_id, clord_id):
   '''Send Order Status Request Message (H)'''
   order_status_message = fix.Message()
   header = order_status_message.getHeader()
   header.setField(fix.MsgType(fix.MsgType_OrderStatusRequest))  # 35 = H
   order_status_message.setField(fix.OrderID(order_id))
   order_status_message.setField(fix.ClOrdID(clord_id))
   order_status_message.setField(fix.Side(fix.Side_BUY))
   order_status_message.setField(fix.Symbol("DOGE-USD"))
   fix.Session.sendToTarget(order_status_message, self.sessionID)

```

#### FIX Message Request: (35=H)

```
8=FIX.4.2|9=181|35=H|34=12|49=SENDERCOMPID|52=20220526-16:12:08.000|56=COIN|11=CLOrdID|37=OrderID|54=1|55=DOGE-USD|10=011|

```

#### FIX Message Response:

```
8=FIX.4.2|9=307|35=8|34=13|49=COIN|52=20220526-16:12:08.134|56=TARGETCOMPID|1=ACCOUNT|6=0|11=CLOrdID|14=0|17=ExecID|20=1|32=0.0|37=OrderID|38=100|39=4|54=1|55=DOGE-USD|150=4|151=100|10=255|

```

## Execution Report (8)

Sent by the server when an order is accepted, rejected, filled, or canceled. Also sent when the user sends an `OrderStatusRequest`.

Tag

Name

Description

Notes

1

Account

The portfolio ID of the associated order

Especially pertinent if using an Org or Entity level API key

6

AvgPx

The average price of the order

11

ClOrdID

ClOrdID of order to be sent back

14

CumQty

Total amount filled on this order

17

ExecID

Unique ID for fill

30

LastMkt

Market of execution for last fill

31

LastPx

Price of the fill if ExecType indicates a fill

32

LastShares

Amount filled (if ExecType=1). Also called LastQty as of FIX 4.3

37

OrderID

OrderID from the ExecutionReport

38

OrderQty

OrderQty as accepted

You must supply this tag or CashOrderQty (depending on whichever you originally submitted)

39

OrdStatus

Order status as of the current message

40

OrdType

Order Type

`1` = Market  
`2` = Limit  
`3` = Stop  
`4` = Stop Limit  
`P` = Pegged

44

Price

Limit Price

50

SenderSubID

ID of the user that initiated the request (e.g. submitted the NOS)

54

Side

Must be `1` to buy or `2` to sell

55

Symbol

Symbol of the original order

58

Text

Human-readable description of error

`UserContext` populated if not null

103

OrdRejReason

Order Reject Reason

See [OrdRejReason Values table](#ordrejreason-values)

13

CommType

The Commission type

Always Absolute (3)

12

Commission

The Coinbase Commission incurred for this fill in quote currency

136

NoMiscFees

Number of repeating groups of miscellaneous fees if any

Only included if there are fees other than the Coinbase fee

137

MiscFeeAmt

Miscellaneous fee value

138

MiscFeeCurr

Currency of miscellaneous fee in quote currency symbol

139

MiscFeeType

Indicates type of miscellaneous fee

See [MiscFeeType Values table](#miscfeetype-values)

150

ExecType

Describes the type of Execution Report (8)

See [ExecType Values table](#exectype-values)

151

LeavesQty

Amount of order remaining

152

CashOrderQty

Order size in quote units (e.g., USD)

You must supply this tag or OrderQty (depending on whichever you originally submitted)

168

EffectiveTime

StartTime for a `TWAP` or `VWAP`

8002

FilledAmt

Cumulative net impact on quote currency

8006

NetAvgPrice

Filled quote size with commission divided by the filled base size

### OrdRejReason Values

OrdRejReason

Description

`0`

Broker option

`1`

Unknown symbol

`2`

Exchange closed

`3`

Order exceeds limit

`4`

Too late to enter

`5`

Unknown Order

`6`

Duplicate Order

`99`

Other

### MiscFeeType Values

MiscFeeType

Description

`1`

Financing fee

`2`

Client commission

`3`

CES commission

`4`

Venue fee (Cost plus pricing only; Only appears for non-zero fees)

### ExecType Values

ExecType

Description

`0`

New Order

`1`

Partial Fill

`2`

Filled

`3`

Done

`4`

Canceled

`6`

Pending Cancel

`7`

Stopped

`8`

Rejected

`D`

Restated

`A`

Pending New

`C`

Expired

`I`

Order Status

## Order Cancel Reject (9)

Sent by the server when an Order Cancel Request cannot be satisfied, e.g., because the order is already canceled or completely filled.

Tag

Name

Description

11

ClOrdID

The same value provided by the original cancel request

37

OrderID

The same value provided by the original cancel request

39

OrdStatus

The order status; see the table below for a list.

41

OrigClOrdID

The same value provided by the original cancel request

102

CxlRejReason

The reason the order was rejected

434

CxlRejResponseTo

The rejection response; see the table below for a list

### OrdStatus Values

Valid Values

Description

`0`

New

`1`

Partially filled

`2`

Filled

`3`

Done for day

`4`

Canceled

`5`

Replaced

`6`

Pending Cancel (e.g., result of `Order Cancel Request <F>`)

`7`

Stopped

`8`

Rejected

`9`

Suspended

`A`

Pending New

`B`

Calculated

`C`

Expired

`D`

Accepted for bidding

`E`

Pending Replace (e.g., result of `Order Cancel/Replace Request <G>`)

### CxlRejResponseTo Values

Valid Values

Description

`1`

`Order Cancel Request <F>`

`2`

`Order Cancel/Replace Request <G>`