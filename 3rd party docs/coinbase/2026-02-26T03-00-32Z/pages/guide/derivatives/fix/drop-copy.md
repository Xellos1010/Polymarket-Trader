# drop copy

The FIX Drop Copy session sends Execution Report (`35=8`) messages to the executing parties and client who might want a record of the trade.

## Drop Copy Message Types

Customers can choose to receive Execution Reports over Drop Copy at two levels: Trade Execution and Order Status.

## Parties Repeating Group (453)

Tag

Name

FIX Type

Req

Description

453

NoPartyIDs

NumInGroup

Y

Number of PartyIDs in repeating group: `453=5` for all order-status, trade, and drop copy messages.

→ 448

PartyID

String(20)

Y

Party identifier/code

→ 447

PartyIDSource

Char

Y

`447 = D (Proprietary)`

→ 452

PartyRole

Int

Y

1 - Subfirm Code  
4 - Clearing Firm Code  
11 - Order Originating Trader  
55 - Session ID  
3 - Client ID (ITM name used for the order)

## Trade Execution

### Trade (150=F)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

Represents order status, “Partial Fill” (`1`) or “Complete Fill” (`2`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordtype-40).

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

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59).

60

TransactTime

UTCTimeStamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

75

TradeDate

LocalMktDate(8)

Y

Indicates date of trade reference in this message in `YYYYMMDD` format.

150

ExecType

Char(1)

Y

Represents execution type, “Trade” (`150=F`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

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

Represents acknowledgment of Outright, Leg of Spread, and Spread. See [MultiLegReporting (442) code set](https://developer.chrome.com/derivatives/fix/code-sets#multilegreporting-442).

527

SecondaryExecID

String(40)

C

Unique exchange ID representing link between spread summary fill notice with leg fill notice and trade cancel messages.

528

OrderCapacity

Int(1)

Y

Represents the type of business conducted:

-   `528=0` = Customer/Agency
-   `528=1` = Principal

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

1057

AggressorIndicator

Char(1)

C

Represents the order as incoming or resting for the match event. See [AggressorIndicator (1057) code set](https://developer.chrome.com/derivatives/fix/code-sets#aggressorindicator-1057).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

828

TrdType

Int(1)

N

Type of trade, only populated if enabled for blocks. Block trades are set to 1 (`828=1`). For all other trade types, TrdType is null.

**Return to [Drop Copy Message Types](#drop-copy-message-types).**

### Trade Correction (150=G)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

19

ExecRefID

String(40)

Y

Unique ID representing the trade being corrected.

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

39

OrdStatus

Char(1)

Y

Represents order status, “Partial Fill” (`1`) or “Complete Fill” (`2`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

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

Indicates date of trade reference in this message in `YYYYMMDD` format.

150

ExecType

Char(1)

Y

Represents execution type, “Trade Correct” (`150=G`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

393

TotalNumSecurities

Int(3)

N

Number of leg fill acknowledgment messages sent with spread summary. Sent for spread fill messages only.

442

MultiLegReportingType

Int(1)

N

Represents acknowledgment of Outright, Leg of Spread, and Spread. See [MultiLegReporting (442) code set](https://developer.chrome.com/derivatives/fix/code-sets#multilegreporting-442).

527

SecondaryExecID

String(40)

C

Unique exchange ID representing link between spread summary fill notice with leg fill notice and trade cancel messages.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

880

TradeMatchId

String(20)

Y

Unique exchange identifier to identify both sides of a match

**Return to [Drop Copy Message Types](#drop-copy-message-types).**

### Trade Cancel (150=H)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

19

ExecRefID

String(40)

Y

Unique ID representing the trade being corrected.

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

39

OrdStatus

Char(1)

Y

Represents order status, “Partial Fill” (`1`) or “Complete Fill” (`2`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

41

OrigClOrdID

String(36)

N

Last accepted `ClOrdID` in the order chain.

54

Side

Char(1)

Y

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

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

Indicates date of trade reference in this message in `YYYYMMDD` format.

150

ExecType

Char(1)

Y

Represents execution type, “Trade Cancel” (`150=H`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

393

TotalNumSecurities

Int(3)

N

Number of leg fill acknowledgment messages sent with spread summary. Sent for spread fill messages only.

442

MultiLegReportingType

Int(1)

N

Represents acknowledgment of Outright, Leg of Spread, and Spread. See [MultiLegReporting (442) code set](https://developer.chrome.com/derivatives/fix/code-sets#multilegreporting-442).

527

SecondaryExecID

String(40)

C

Unique exchange ID representing link between spread summary fill notice with leg fill notice and trade cancel messages.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

**Return to [Drop Copy Message Types](#drop-copy-message-types).**

## Order Status

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

Represents order status, “New”, “Canceled”, or “Modify” (replaced). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

-   `0` = New
-   `4` = Canceled
-   `5` = Modify

40

OrdType

Char(1)

Y

Order type. Market orders are not accepted during opening auction. See [OrdType (40) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordtype-40).

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

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59).

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

Stop price of the order. Required for stop and stop-limit orders.

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

Minimum quantity of an order to be executed. Used only when `59=3` ([TimeInForce)](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59)\=“Fill and Kill”).

150

ExecType

Char(1)

Y

Represents execution type, New (`150=0`), Canceled (`150=4`), or Replaced (`150=5`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

151

LeavesQty

int(9)

Y

Number of contracts remaining for execution.

378

ExecRestatementReason

Int(3)

N

Reason why the order was canceled by the system (e.g., cancel on disconnect, self-match prevention, etc.). See [ExecRestatementReason (378) code set](https://developer.chrome.com/derivatives/fix/code-sets#execrestatementreason-378).

432

ExpireDate

LocalMktDate(8)

C

Order expiration date, or the last day the order could trade.

528

OrderCapacity

Int(1)

C

Pass through field from/when present in NewOrder. Identifies origin of order (i.e., capacity of firm placing the order). See [OrderCapacity (528) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordercapacity-528).

582

CustOrderCapacity

Char(1)

C

Pass through field from/when present in NewOrder. Customer Type Indicator as defined by NFA. See [CustOrderCapacity (582) code set](https://developer.chrome.com/derivatives/fix/code-sets#custordercapacity-582).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

3040

StopLimitPx

Price(20)

C

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

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/derivatives/fix/code-sets#selfmatchpreventionstrategy-8000).

**Return to [Drop Copy Message Types](#drop-copy-message-types).**

### Rejected Order (150=8)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

Represents order status, “Rejected” (`8`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordtype-40).

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

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

58

Text

String(200)

N

Error code message. See [OrdRejReason (103) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordrejreason-103).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59).

60

TransactTime

UTCTimeStamp(21)

Y

Time when the order message was submitted. UTC format `YYYYMMDD-HH:MM:SS.sss` in microseconds.  
Example: `20091216-19:21:41.109`

18

ExecInst

Char

N

The execution instruction flags for the order.

Supported values:  
`6` = Add Liquidity Only (Post Only)

103

OrdRejReason

Int(6)

N

Error code. See [OrdRejReason (103) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordrejreason-103).

150

ExecType

Char(1)

Y

Represents execution type, “Rejected” (`150=8`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

**Return to [Drop Copy Message Types](#drop-copy-message-types).**

### Done for Day (150=3)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

Represents order status, “Done for the Day” (`39=3`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordtype-40).

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

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`.

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59).

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

Represents execution type, “Done for Day” (`150=3`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

151

LeavesQty

Int(9)

C

Number of contracts remaining for execution.

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

### Expired Order (150=C)

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

453

NoPartyIDs

NumInGroup

Y

Represents the Parties repeating group. See [Parties Repeating Group (453)](#parties-repeating-group-453).

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

Represents order status, “Expired” (`C`). See [OrdStatus (39) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordstatus-39).

40

OrdType

Char(1)

Y

Order type such as market, limit, etc. See [OrdType (40) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordtype-40).

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

Side of order. See [Side (54) code set](https://developer.chrome.com/derivatives/fix/code-sets#side-54--legside-624).

55

Symbol

String(24)

Y

Represents details of an instrument. Future Example: `EUM20`

167

SecurityType

String(6)

N

Represents security type. See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

59

TimeInForce

Char(1)

N

Represents how long the order remains in effect. Default is `59=0` (TimeInForce=“Day”). For `59=3` (TimeInForce=“FAK”), `MinQty` can also be specified. See [TimeInForce (59) code set](https://developer.chrome.com/derivatives/fix/code-sets#timeinforce-59).

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

Represents execution type, “Expired” (`150=C`). See [ExecType (150) code set](https://developer.chrome.com/derivatives/fix/code-sets#exectype-150).

151

LeavesQty

Int(9)

Y

Number of contracts remaining for execution. Always `151=0`.

378

ExecRestatementReason

Int(3)

N

Reason why the order was canceled by the system (e.g., cancel on disconnect, self-match prevention, etc.). See [ExecRestatementReason (378) code set](https://developer.chrome.com/derivatives/fix/code-sets#execrestatementreason-378).

528

OrderCapacity

Int(1)

C

Pass through field from/when present in NewOrder. Identifies origin of order (i.e., capacity of firm placing the order). See [OrderCapacity (528) code set](https://developer.chrome.com/derivatives/fix/code-sets#ordercapacity-528).

1028

ManualOrderIndicator

Boolean(1)

Y

Represents whether or not the order was generated manually (`Y`) or automatically (`N`) with trading software. See [ManualOrderIndicator (1028) code set](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028).

1031

CustOrderHandlingInst

String(1)

Y

Source of the original order. See [CustOrderHandlingInst (1031) code set](https://developer.chrome.com/derivatives/fix/code-sets#customerorderhandlinginst-1031).

7928

SelfMatchPreventionID

Int(8)

N

Unique ID (per executing firm) representing two orders that should not match. Required when market participants enable SelfMatch Prevention. Max length is 8 digits.

8000

SelfMatchPreventionStrategy

Char(1)

N

Represents type of cancel instruction when SelfMatch Prevention is triggered. Client systems must also include `SelfMatchPreventionID` (7928) on the originating message. See [SelfMatchPreventionStrategy (8000) code set](https://developer.chrome.com/derivatives/fix/code-sets#selfmatchpreventionstrategy-8000).

5979

RequestTime

Int(20)

N

Time when the request was received by the exchange. UTC format in microseconds: `YYYYMMDD-HH:MM:SS.sss`.

**Return to [Drop Copy Message Types](#drop-copy-message-types).**