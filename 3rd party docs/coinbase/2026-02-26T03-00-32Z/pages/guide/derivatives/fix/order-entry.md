# order entry

The Coinbase Derivatives Exchange Order Entry FIX API for brokers and market makers allows connected firms to send, modify and cancel orders.

## Order Entry Message Types

The Coinbase Derivatives Exchange Order Entry FIX API supports the following message and execution types:

## New Order Single (35=D)

Used to send new single-legged orders for execution.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

38

OrderQty

Int(9)

Y

Order quantity. Must be a positive integer.

40

OrdType

Char(1)

Y

Order type. Market orders are not accepted during opening auction. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

44

Price

Price(20)

C

Price per single contract unit. Required for limit or stop-limit orders.

-   For sell OCO: Limit Price must be `>` StopPx
-   For buy OCO: Limit Price must be `<` StopPx

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

Y

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

77

PositionEffect

Char(1)

N

Order position effect. Default is `77=D` (PositionEffect=“Default”). See [PositionEffect (77) code set](https://developer.chrome.com/code-sets#positioneffect-77).

99

StopPx

Price(20)

C

Stop price of the order. Required for stop, stop-limit, and OCO orders.

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

110

MinQty

Int(9)

N

Minimum quantity of an order to be executed. Used only when `59=3` ([TimeInForce)](https://developer.chrome.com/code-sets#timeinforce-59)\=“Fill and Kill”).

528

OrderCapacity

Char(1)

Y

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

432

ExpireDate

LocalMktDate(8)

C

Order expiration date, or the last day the order can trade. `YYYYMMDD` format. Required only if `59=6` ([TimeInForce](https://developer.chrome.com/code-sets#timeinforce-59)\=“Good Till Date”).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

7928

SelfMatchPreventionID

Int(8)

C

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/code-sets#selfmatchpreventionstrategy-8000).

582

CustOrderCapacity

Int(1)

Y

Customer capacity represented by Customer Type Indicator (CTI) Code (1-4). See [CustOrderCapacity (582) code set](https://developer.chrome.com/code-sets#custordercapacity-582).

3040

StopLimitPx

Decimal

Y

Limit order price if stop loss triggers

453

Parties (Firm / ITM overrides)

NumInGroup

N

Present only when overriding the executing firm/ITM; see [Parties Repeating Group (453)](#parties-repeating-group-453).

### Parties Repeating Group (453)

Tag

Name

FIX Type

Req

Description

453

NoPartyIDs

NumInGroup

C

Number of PartyIDs in repeating group. Required only when multi-ITM overrides are used (`453=2`).

→ 448

PartyID

String(20)

Y

Party identifier/code

→ 447

PartyIDSource

Char

Y

`447 = 99 (PROPRIETARY_CUSTOM_CODE)`

→ 452

PartyRole

Int

Y

`1` - Executing Firm (sub-firm code override)  
`3` - Client ID (ITM name override). Both roles must be supplied together when using overrides.

LIMIT order example

```
### Example: LIMIT order
BeginString = FIX.4.4
BodyLength = …
MsgType = D
MsgSeqNum = 3
PossDupFlag =
SenderCompID = EBR123
SenderSubID = smithj
SendingTime = 20191202-10:15:41.383
TargetCompID = COIND
TargetSubID = TEST
Account = C123
ClOrdID = 314bb362:109f840f9c0
Symbol=EUM20
SecurityType=FUT
OrderQty = 100
OrdType = 2
Price = 1.10317
Side = 1
TimeInForce = 0
TransactTime = 20171102-10:15:40.383
PositionEffect = O
CustomerOrFirm = 0
OrderCapacity = A
ManualOrderIndicator = Y
CustOrderHandlingInst = Y
SelfMatchPreventionID = 12347565
SelfMatchPreventionStrategy = N
CTICode = 4
CheckSum = …

```

### Firm and ITM overrides

-   This feature is optional and only open to FCM.
-   Overrides apply **only** to `NewOrderSingle`; cancel, cancel/replace, and other messages must omit these Parties rows.
-   Each override request must include exactly two party rows in the `Parties (453)` group:
    1.  `452=1 (Executing Firm)` with `447=99` and `448=<sub-firm code>`
    2.  `452=3 (Client ID)` with `447=99` and `448=<ITM name>`
-   The supplied firm and ITM must reside under the same FCM as the trading user; cross-FCM overrides are rejected.
-   Both rows are mandatory; an incomplete pair, unknown identifiers, or an alternate `PartyIDSource` results in rejection.
-   SelfMatch Prevention fields (`7928` ID and `8000` mode) follow the normal SMP configuration for the session
-   If the firm/ITM override feature is disabled for the FIX session, any order containing the override Parties group is rejected, and the system does not fall back to the user’s default firm/ITM.

Example override block (placed after the standard 35=D fields):

```
453=2|448=SUBFIRM123|447=99|452=1|448=ITM-ABC|447=99|452=3

```

### OCO Orders

OCO (One Cancels the Other) orders combine the features of limit and stop orders (and CDE treats them as a single order, not two linked ones). OCO orders let users set predefined limit and stop levels simultaneously depending on their view of the market. Initially, an OCO order acts like a regular limit order, contributing to market data. When its stop price condition is met, it shifts and behaves like a stop limit order. **Price Rules**

-   Sell OCO: Limit Price must be `>` StopPx
-   Buy OCO: Limit Price must be `<` StopPx

**Caveats​**

-   DAY, GTC and GTD are supported for TimeInForce
-   The Post-Only tag is not supported. It cannot be populated or must be false.

**Parameters**

Tag

Name

FIX Type

Req

Description

40

OrdType

Char

Y

Order Type must be O (the letter Oh)

44

Price

Decimal

Y

Limit price (in this context)

99

StopPx

Decimal

Y

Stop loss trigger price

3040

StopLimitPx

Decimal

Y

Limit order price if stop loss triggers

## Order Cancel Request (35=F)

A firm may use this message to cancel the remaining quantity of any working single or multi-leg order.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

Y

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

CANCEL order example

```
# Example: Cancel order request
BeginString = FIX.4.4
BodyLength = …
MsgType = F
MsgSeqNum = 6
PossDupFlag =
SenderCompID = EBR123
SenderSubID = smithj
SendingTime = 20191202-10:15:41.383
TargetCompID = COIND
TargetSubID = TEST
Account = C123
ClOrdID = 65780bfc:901n328943d
OrderID = 5038
OrigClOrdID = 71fa0cda:109f847292d Side = 1
Symbol = EUM20
SecurityType = FUT
TransactTime = 20171102-10:22:24.725
ManualOrderIndicator = Y
CheckSum = …

```

## Order Cancel/Replace Request (35=G)

A firm may use this message to replace an order. The following restrictions exist:

-   Triggered Stop, Stop-Limit, and OCO orders cannot be replaced, only canceled.
-   Time in Force cannot be changed.
-   If a field cannot be changed, the replacing order gets rejected.
-   If the replaced-OrderQty is less than or equal to Total Fill Qty, then the Order is canceled.
-   `Parties` rows that attempt to override the executing firm (`452=1`) or ITM (`452=3`) are not allowed on replaces and will be rejected.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.  
**Caution:** If the Account value is changed from the original value submitted in the New Order message, the order loses priority in the order book.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

Y

Order quantity. Must be a positive integer.  
**Caution:** If OrderQty is reduced from the original value submitted in the New Order message, the order will maintain priority in the order book. If this value is increased, the order will lose priority on the order book.

40

OrdType

Char(1)

Y

Order type. Market orders are not accepted during opening auction. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit. Required for limit or stop-limit orders.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

77

PositionEffect

Char(1)

N

Order position effect. Default is `77=D` (PositionEffect=“Default”). See [PositionEffect (77) code set](https://developer.chrome.com/code-sets#positioneffect-77).

99

StopPx

Price(20)

C

Stop price of the order. Required for stop, stop-limit, and OCO orders.  
**Caution:** If the StopPx value is changed from the original value submitted in the New Order message, the order loses priority in the order book.

528

OrderCapacity

Char(1)

Y

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

432

ExpireDate

LocalMktDate(8)

C

Order expiration date, or the last day the order can trade. `YYYYMMDD` format. Required only if `59=6` ([TimeInForce](https://developer.chrome.com/code-sets#timeinforce-59)\=“Good Till Date”).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

7928

SelfMatchPreventionID

Int(8)

C

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/code-sets#selfmatchpreventionstrategy-8000).

582

CustOrderCapacity

Int(1)

Y

Customer capacity represented by Customer Type Indicator (CTI) Code (1-4). See [CustOrderCapacity (582) code set](https://developer.chrome.com/code-sets#custordercapacity-582).

3040

StopLimitPx

Price(20)

C

Limit order price if stop loss triggers

Cancel/Replace LIMIT order example

```
# Example: Cancel/Replace LIMIT order request
BeginString = FIX.4.4
BodyLength = …
MsgType = G
MsgSeqNum = 15
PossDupFlag =
SenderCompID = EBR123
SenderSubID = smithj
SendingTime = 20191202-10:15:41.383
TargetCompID = COIND
TargetSubID = TEST
Account = C123
ClOrdID = 71fa0cda:109f847292d:-7ffe
OrderID = 5095
OrderQty = 150
OrdType = 2
OrigClOrdID = 71fa0cda:109f847292d:-7ffd
Price = 1.10317
Side = 1
Symbol=EUM20
SecurityType=FUT
TimeInForce = 2
TransactTime = 20191102-10:22:21.725
PositionEffect = O
OrderCapacity = A
ManualOrderIndicator = Y
CustOrderHandlingInst = Y
SelfMatchPreventionID = 12345678
SelfMatchPreventionStrategy = N
CTICode = 4
CheckSum = …

```

## Execution Report (35=8)

### New/Canceled/Replaced Order (35=8, 150=0/4/5)

New (`150=0`), Canceled (`150=4`), and Replaced (`150=5`) reports share the same subset of Execution Report tags.

-   **New** (`150=0`): Confirms new order in response to `NewOrderSingle` message. Sent for every new order, even those matched immediately.
-   **Canceled** (`150=4`): Confirms order is canceled in response to `OrderCancelRequest`, or is an unsolicited cancel. A separate message is *not* sent for the canceled order itself.
-   **Replaced** (`150=5`): Confirms order is replaced in response to `OrderCancel/ReplaceRequest`. A separate message is *not* sent for the replaced order itself.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

Y

Order quantity.

39

OrdStatus

Char(1)

Y

Represents order status, “New”, “Canceled”, or “Modify” (replaced). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

-   `0` = New
-   `4` = Canceled
-   `5` = Modify

40

OrdType

Char(1)

Y

Order type. Market orders are not accepted during opening auction. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit. Required for limit or stop-limit orders.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

99

StopPx

Price(20)

C

Stop price of the order. Required for stop, stop-limit, and OCO orders.

110

MinQty

Int(9)

N

Minimum quantity of an order to be executed. Used only when `59=3` ([TimeInForce)](https://developer.chrome.com/code-sets#timeinforce-59)\=“Fill and Kill”).

150

ExecType

Char(1)

Y

Represents execution type, New (`150=0`), Canceled (`150=4`), or Replaced (`150=5`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

int(9)

Y

Number of contracts remaining for execution.

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

528

OrderCapacity

Char(1)

C

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). Pass through field from/when present in [New Order Single (35=D)](#new-order-single-35d). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

378

ExecRestatementReason

Int(3)

N

Reason why the order was canceled by the system (e.g., cancel on disconnect, self-match prevention, etc.). See [ExecRestatementReason (378) code set](https://developer.chrome.com/code-sets#execrestatementreason-378).

432

ExpireDate

LocalMktDate(8)

C

Order expiration date, or the last day the order could trade.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

7928

SelfMatchPreventionID

Int(8)

C

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/code-sets#selfmatchpreventionstrategy-8000).

### Rejected Order (35=8, 150=8)

Message notifies client system of a rejected order.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

C

Order quantity.

39

OrdStatus

Char(1)

Y

Represents order status, “Rejected” (`8`). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

58

Text

String(200)

N

Error code message. See [OrdRejReason (103) code set](https://developer.chrome.com/code-sets#ordrejreason-103).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimeStamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

75

TradeDate

LocalMktDate(8)

Y

Date of trading day, local time in `YYYYMMDD` format. When absent, represents the current day.

103

OrdRejReason

Int(6)

N

Error code. See [OrdRejReason (103) code set](https://developer.chrome.com/code-sets#ordrejreason-103).

150

ExecType

Char(1)

Y

Represents execution type, “Rejected” (`150=8`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

Int(9)

C

Number of contracts remaining for execution. Always `151=0`.

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

528

OrderCapacity

Char(1)

C

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). Pass through field from/when present in [New Order Single (35=D)](#new-order-single-35d). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

### Trade (35=8, 150=F)

Sent by the Coinbase Derivatives Exchange system to report a trade.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

880

TrdMatchID

String (20)

Y

Unique exchange ID representing a match event that results in multiple executions or trades.

31

LastPx

Price(20)

Y

Price at which order was filled.

32

LastQty

Int(9)

Y

Quantity filled.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

C

Order quantity.

39

OrdStatus

Char(1)

Y

Represents order status, “Partial Fill” (`1`) or “Complete Fill” (`2`). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit.

For Execution Report messages sent in response to Market or Stop orders (with protection), Price is the Protection Price Limit (best available price +/- protection points).  
If the order is not completely filled, the remaining open quantity rests on the order book at the Protection Price Limit.  
**Note:** For spread trade Execution Reports, Price (44) is sent in the Execution Report – Fill Notice (35=`8`, 39=`1` or `2`) for the spread only and not the legs of the spread.

6

AvgPx

Price(20)

C

Calculated average price of all fills on this order.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimeStamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

75

TradeDate

LocalMktDate(8)

Y

Date of trading day, local time in `YYYYMMDD` format. When absent, represents the current day.

150

ExecType

Char(1)

Y

Represents execution type, “Trade” (`150=F`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

Int(9)

C

Number of contracts remaining for execution after this fill.

393

TotalNumSecurities

Int(3)

N

Number of leg fill acknowledgment messages sent with spread summary. Sent for spread fill messages only.

442

MultiLegReportingType

Int(1)

N

Represents acknowledgment of Outright, Leg of Spread, and Spread. See [MultiLegReporting (442) code set](https://developer.chrome.com/code-sets#multilegreporting-442).

527

SecondaryExecID

String(40)

C

Unique exchange ID representing link between spread summary fill notice with leg fill notice and trade cancel messages.

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

528

OrderCapacity

Char(1)

Y

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

1057

AggressorIndicator

Char(1)

C

Represents the order as incoming or resting for the match event. See [AggressorIndicator (1057) code set](https://developer.chrome.com/code-sets#aggressorindicator-1057).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

### Stop Triggered (35=8, 150=L)

Sent when a stop order is triggered.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

Y

Order quantity.

39

OrdStatus

Char(1)

Y

Represents order status, “New” (accepted), “Canceled”, or “Modify” (replaced). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

-   `0` = New
-   `4` = Canceled
-   `5` = Modify

40

OrdType

Char(1)

Y

Order type. Market orders are not accepted during opening auction. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit. Required for limit or stop-limit orders.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

99

StopPx

Price(20)

C

Stop price of the order. Required for stop, stop-limit, and OCO orders.

150

ExecType

Char(1)

Y

Represents execution type, “Stop Triggered” (`150=L`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

int(9)

Y

Number of contracts remaining for execution.

378

ExecRestatementReason

Int(3)

N

Reason why the order was canceled by the system (e.g., cancel on disconnect, self-match prevention, etc.). See [ExecRestatementReason (378) code set](https://developer.chrome.com/code-sets#execrestatementreason-378).

432

ExpireDate

LocalMktDate(8)

C

Order expiration date, or the last day the order could trade.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

7928

SelfMatchPreventionID

Int(8)

C

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/code-sets#selfmatchpreventionstrategy-8000).

### Done for Day (35=8, 150=3)

Reports with execution type Done for Day (`150=3`) are sent by the exchange to the firms that need the status of their open orders after the trading session is closed.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

C

Order quantity.

39

OrdStatus

Char(1)

Y

Represents order status, “Done for the Day” (`39=3`). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

C

Price per single contract unit.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimeStamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

75

TradeDate

LocalMktDate(8)

Y

Date of trading day, local time in `YYYYMMDD` format. When absent, represents the current day.

150

ExecType

Char(1)

Y

Represents execution type, “Done for Day” (`150=3`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

Int(9)

C

Number of contracts remaining for execution.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

### Expired Order (35=8, 150=C)

Reports with execution type Expired (`150=C`) are sent by the exchange to the firms that need the track the expiration of their time-limited orders (such as DAY).

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

14

CumQty

Int(9)

Y

Cumulated traded quantity throughout lifespan of an order.

17

ExecID

String(40)

Y

Unique exchange ID representing the trade execution.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

38

OrderQty

Int(9)

Y

Order quantity. Must be a positive integer.

39

OrdStatus

Char(1)

Y

Represents order status, “Expired” (`C`). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/code-sets#ordtype-40).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

44

Price

Price(20)

N

Price per single contract unit.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/code-sets#timeinforce-59).

60

TransactTime

UTCTimestamp(21)

N

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

110

MinQty

Int(9)

C

Sent in the Order Elimination message if the originating order was a FAK/FOK/MinQty that contained tag MinQty (110).

150

ExecType

Char(1)

Y

Represents execution type, “Expired” (`150=C`). See [ExecType (150) code set](https://developer.chrome.com/code-sets#exectype-150).

151

LeavesQty

Int(9)

Y

Number of contracts remaining for execution. Always `151=0`.

378

ExecRestatementReason

Int(3)

N

Reason why the order was canceled by the system (e.g., cancel on disconnect, self-match prevention, etc.). See [ExecRestatementReason (378) code set](https://developer.chrome.com/code-sets#execrestatementreason-378).

528

OrderCapacity

Char(1)

C

Capacity of the firm placing the order (Agency=`A`/Principal=`P`). Pass through field from/when present in [New Order Single (35=D)](#new-order-single-35d). See [OrderCapacity (528) code set](https://developer.chrome.com/code-sets#ordercapacity-528).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Decimal

Y

Limit order price when stop loss is triggered

7928

SelfMatchPreventionID

Int(8)

N

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/code-sets#selfmatchpreventionstrategy-8000).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

## Order Cancel Reject (35=9)

Sent by the Coinbase Derivatives Exchange system to notify the client on rejection of an Order Cancel Request OR Order Cancel/Replace.

Tag

Name

FIX Type

Req

Description

1

Account

String(16)

Y

Unique ID representing the account.

11

ClOrdID

String(36)

Y

Unique client ID representing the order. Must not exceed 36 ascii characters. Client system must maintain uniqueness of this value for the life of the order.

37

OrderID

String(17)

Y

Unique exchange ID representing the order.

39

OrdStatus

Char(1)

Y

Represents order status, “Undefined” (`U`). See [OrdStatus (39) code set](https://developer.chrome.com/code-sets#ordstatus-39).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain. If a value is included in tag 41 on order entry, the same value is returned. If not, the tag contains `0`.

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

58

Text

String(200)

N

Text describing why the cancel request was rejected.

60

TransactTime

UTCTimestamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

102

CXlRejReason

Int(6)

N

Code representing the cancel reject reason. [See CXRejReason (102) code set](https://developer.chrome.com/code-sets#cxrejreason-102).

434

CxlRejResponseTo

Char(1)

Y

Represents the type of request that an Order Cancel Reject (35=9) message is in response to.

-   `1` = Order Cancel Request
-   `2` = Order Cancel/Replace Request

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

## Business Reject (35=j)

Sent by the Coinbase Derivatives Exchange system to notify the client of application-level rejection of client request which passed session-level validation but has business validation issues and cannot be reported with a well-formed Execution Report with Rejected status.

Tag

Name

FIX Type

Req

Description

45

RefSeqNum

SeqNum(10)

Y

MsgSeqNum `<34>` of rejected message.

58

Text

String(200)

N

Free format text string.

379

BusinessRejectRefID

String(32)

N

Value of business-level “ID” of the message being rejected.

380

BusinessRejectReason

Int(2)

Y

Code identifying reject reason:

`0`\=Other  
`1`\=Unknown ID  
`2`\=Unknown security  
`3`\=Unsupported message type  
`4`\=Application not available  
`5`\=Conditionally required field missing  
`6`\=Not authorized  
`7`\=Delivery to firm not available at this time

## Last ExecId Request (35=F1)

Send this message to request the ExecId of the last (most recent) event sent by the trading system to this user/session. Can be used to determine if the client missed any events while disconnected. Also serves as a means of validating that the trading system is available and accepting requests.

Tag

Name

FIX Type

Req

Description

35

MsgType

String(2)

Y

Header tag identifying message type.

## Last ExecId (35=F2)

Sent in response to LastExecIdRequest.

Tag

Name

FIX Type

Req

Description

45

RefSeqNum

SeqNum(10)

Y

MsgSeqNum `<34>` of request message

17

ExecID

String(40)

Y

ExecId of last event sent to this user.

## Event Resend Request (35=F3)

Send this message to request order events in the specified range be resent. Since this is an application-level request, resent messages will have new sequence numbers and PossDupFlag (43) will not be set; rather, PossResend (97) will be set. Rejects (and any other message that does not contain an ExecId) will not be resent.

Tag

Name

FIX Type

Req

Description

22003

BeginExecId

String(40)

Y

Lower bound (inclusive) of ExecIds.

22004

EndExecId

String(40)

N

Upper bound (inclusive) of ExecIds.  
Resend all events up the last known event if not set.

## Event Resend Complete (35=F4)

Sent in response to a successful Event Resend Request following all resent events.

Tag

Name

FIX Type

Req

Description

45

RefSeqNum

SeqNum(10)

Y

MsgSeqNum `<34>` of request message

22005

ResentEventCount

Int

Y

Total number of events resent.

## Event Resend Reject (35=F5)

Sent in response to an Event Resend Request if the request cannot be fulfilled.

Tag

Name

FIX Type

Req

Description

45

RefSeqNum

SeqNum(10)

Y

MsgSeqNum `<34>` of request message

22006

EventResendRejectReason

Int

Y

Represents reject reason:  
`1` = BEGIN\_EXEC\_ID\_TOO\_SMALL  
`2` = END\_EXEC\_ID\_TOO\_LARGE  
`3` = RESEND\_ALREADY\_IN\_PROGRESS  
`4` = TOO\_MANY\_RESEND\_REQUESTS  
`5` = SERVER\_ERROR

58

Text

String(200)

N

Free format text string.

**See Also:**

-   [Order Entry Gateways](https://developer.chrome.com/introduction/runbook#order-entry-gateways)