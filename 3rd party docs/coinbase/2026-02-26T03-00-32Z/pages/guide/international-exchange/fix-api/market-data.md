# market data

## SecurityListRequest (35=x)

Submits a request to get a list of the securities available to trade on the exchange.

Tag

Name

Type

Required

Description

320

SecurityReqID

string

Y

Unique identifier for the security request used to correlate the response to the request.

559

SecurityListRequestType

string

Y

Specifies the scope of the security request.

Supported values include:  
0 = Symbol  
4 = All Securities

55

Symbol

string

C

Required when SecurityListRequestType (tag 559) is set to Symbol (0). The SecurityList response will only include results matching the symbol specified in this field.

## SecurityList (35=y)

A response to a SecurityListRequest that includes information about the requested securities.

Tag

Name

Type

Required

Description

320

SecurityReqID

string

Y

The SecurityReqID of the SecurityListRequest that this message is in response to

322

SecurityResponseID

string

Y

A unique response ID

560

SecurityRequestResult

string

Y

0 = ValidReq  
1 = InvalidReq

393

TotNoRelatedSym

int

Y

Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation is required.

893

LastFragment

char

N

N = Not Last message  
Y = Last Message for SecurityListRequest

146

NoRelatedSym

int

Y

When >0 a repeating group of instrument definitions follows.

```
→ 55
```

Symbol

string

Y

The symbol used to represent the instrument.

```
→ 167
```

SecurityType

string

Y

Identifies the type of instrument. Values include:

FXSPOT = Spot pairs  
PERP = Perpetual futures

```
→ 762
```

SecuritySubType

string

Y

Identifies the mode of instrument. Values include:

STANDARD = Standard instrument mode  
PRE\_LAUNCH = Pre-launch mode  
PRE\_LAUNCH\_CONVERTING = Converting from pre-launch mode to standard mode

```
→ 231
```

ContractMultiplier

decimal

N

Specifies the multiply factor to indicate the number of units in one contract.

```
→ 969
```

MinPriceIncrement

decimal

Y

The tick size of the instrument.

```
→ 898
```

MarginRatio

decimal

Y

The base initial margin requirement.

```
→ 21000
```

DefaultMarginRatio

decimal

N

The default initial margin requirement.

```
→ 870
```

NoInstrAttrib

int

Y

Marks the beginning of the Instrument Attributes repeating group component.

```
→→ 871
```

InstrAttribType

int

Y

Identifies the type of instrument attribute the InstrAttribValue (872) represents.

Supported values include:  
40 = Average daily notional amount  
41 = Average daily trade quantity  
42 = Total day notional amount (30 days)  
43 = Total day trade quantity (30 days)  
44 = Total hour notional amount (24 hours)  
45 = Total hour trade quantity (24 hours)

Average daily notional amount (40), Total day notional amount (30 days) (42), Total hour notional amount (24 hours) (44) are not sent for spot instruments which route orders to Coinbase Exchange.

```
→→ 872
```

InstrAttribValue

string

Y

The value corresponding to the InstrAttribType (871).

```
→ 864
```

NoEvents

int

N

Marks the beginning of the Events repeating group component.

```
→→ 865
```

EventType

int

N

Identifies the event represented by EventTime (1145).

Supported values include:  
101 = Pre-launch conversion time

```
→→ 1145
```

EventTime

utc timestamp

N

The timestamp corresponding to the EventType (865).

```
→ 15
```

Currency

string

Y

Currency used for price.

```
→ 562
```

MinTradeVol

decimal

Y

The minimum quantity in terms of USDC notional value accepted for an order.

```
→ 1140
```

MaxTradeVol

decimal

N

The maximum quantity allowed for position size. (Will be deprecated in a future release)

```
→ 970
```

PositionLimit

decimal

N

The maximum notional value allowed for position.

```
→ 561
```

RoundLot

decimal

Y

The maximum precision allowed when specifying a quantity.

```
→ 1682
```

MDSecurityTradingStatus

int

Y

Identifies the status of the market. Values include:

2 = Halt  
17 = Ready to trade  
18 = Not available for trading  
19 = Not Traded On This Market  
99 = Pause

100 = Auction Mode  
101 = Cancel Only  
102 = Post Only  
103 = Limit Only  
104 = Trading Disabled  
105 = Cancel Only Enforced By Coinbase International Exchange  
106 = Coinbase Exchange Unreachable

-   During Halt (2), limit orders can be submitted or canceled, but will not match. When moving from Halt (2) to Ready to trade (17) all buy orders above and all sell orders below a communicated target price get canceled before matching resumes.
-   During Ready to trade (17), all supported order types are accepted and continuous matching occurs.
-   During Pause (99), orders can only be canceled. When moving from Pause (99) to Ready to trade (17), crossed orders fill at the last Opening price (sent as MDEntryType Opening price (269=4) in MarketDataSnapshotFullRefresh (35=W) and MarketDataIncrementalRefresh (35=X))
-   During Not Traded On This Market (19), orders will not be accepted but liquidation execution reports may reference this symbol
-   Auction Mode (100), Cancel Only (101), Post Only (102), Limit Only (103), Trading Disabled (104) are forwarded From Coinbase Exchange for spot instruments which route orders to Coinbase Exchange.
-   Cancel Only Enforced By Coinbase International Exchange (105) means Coinbase International Exchange only permits order cancellation separately from Coinbase Exchange
-   Coinbase Exchange Unreachable (106) means the Coinbase International Exchange component is unable to establish connectivity with Coinbase Exchange

711

NoUnderlyings

int

N

Number of underlyings

```
→ 310
```

UnderlyingSecurityType

string

N

Underlying security’s SecurityType.

Possible values are:  
INDEX = Underlying is an index  
FXSPOT = Underlying is a spot pair

## SecurityDefinition (35=d)

Sent when a new security becomes available for trading or something changes with an existing security.

Tag

Name

Type

Required

Description

980

SecurityUpdateAction

char

Y

Indicates whether the message is adding, modifying, or deleting a instrument.

Values include:  
A = Add  
M = Modify  
D = Delete

779

LastUpdateTime

utc timestamp

Y

Timestamp of the last update or creation if no updates have occurred yet.

55

Symbol

string

Y

The symbol used to represent the instrument.

167

SecurityType

string

Y

Identifies the type of instrument. Values include:  
FXSPOT = Spot pairs  
PERP = Perpetual futures

762

SecuritySubType

string

Y

Identifies the mode of instrument. Values include:

STANDARD = Standard instrument mode  
PRE\_LAUNCH = Pre-launch mode  
PRE\_LAUNCH\_CONVERTING = Converting from pre-launch mode to standard mode

231

ContractMultiplier

decimal

N

Specifies the multiply factor to indicate the number of units in one contract.

969

MinPriceIncrement

decimal

Y

The tick size of the instrument.

898

MarginRatio

decimal

Y

The base initial margin requirement.

21000

DefaultMarginRatio

decimal

N

The default initial margin requirement.

870

NoInstrAttrib

int

Y

Marks the beginning of the Instrument Attributes repeating group component.

```
→ 871
```

InstrAttribType

int

Y

Identifies the type of instrument attribute the InstrAttribValue (872) represents.

Supported values include:  
40 = Average daily notional amount  
41 = Average daily trade quantity  
42 = Total day notional amount (30 days)  
43 = Total day trade quantity (30 days)  
44 = Total hour notional amount (24 hours)  
45 = Total hour trade quantity (24 hours)

Average daily notional amount (40), Total day notional amount (30 days) (42), Total hour notional amount (24 hours) (44) are not sent for spot instruments which route orders to Coinbase Exchange.

```
→ 872
```

InstrAttribValue

string

Y

The value corresponding to the InstrAttribType (871).

864

NoEvents

int

N

Marks the beginning of the Events repeating group component.

```
→ 865
```

EventType

int

N

Identifies the event represented by EventTime (1145).

Supported values include:  
101 = Pre-launch conversion time

```
→ 1145
```

EventTime

utc timestamp

N

The timestamp corresponding to the EventType (865).

15

Currency

string

Y

Currency used for price.

562

MinTradeVol

decimal

Y

The minimum quantity in terms of USDC notional value accepted for an order.

1140

MaxTradeVol

decimal

N

The maximum quantity allowed for position size. (Will be deprecated in a future release)

970

PositionLimit

decimal

N

The maximum notional value allowed for position.

561

RoundLot

decimal

Y

The maximum precision allowed when specifying a quantity.

1682

MDSecurityTradingStatus

int

Y

Identifies the status of the market. Values include:

2 = Halt  
17 = Ready to trade  
18 = Not available for trading  
19 = Not Traded On This Market  
99 = Pause

100 = Auction Mode  
101 = Cancel Only  
102 = Post Only  
103 = Limit Only  
104 = Trading Disabled  
105 = Cancel Only Enforced By Coinbase International Exchange  
106 = Coinbase Exchange Unreachable

-   During Halt (2), limit orders can be submitted or canceled, but will not match. When moving from Halt (2) to Ready to trade (17) all buy orders above and all sell orders below a communicated target price get canceled before matching resumes.
-   During Ready to trade (17), all supported order types are accepted and continuous matching occurs.
-   During Pause (99), orders can only be canceled. When moving from Pause (99) to Ready to trade (17), crossed orders fill at the last Opening price (sent as MDEntryType Opening price (269=4) in MarketDataSnapshotFullRefresh (35=W) and MarketDataIncrementalRefresh (35=X))
-   During Not Traded On This Market (19), orders will not be accepted but liquidation execution reports may reference this symbol
-   Auction Mode (100), Cancel Only (101), Post Only (102), Limit Only (103), Trading Disabled (104) are forwarded From Coinbase Exchange for spot instruments which route orders to Coinbase Exchange.
-   Cancel Only Enforced By Coinbase International Exchange (105) means Coinbase International Exchange only permits order cancellation separately from Coinbase Exchange
-   Coinbase Exchange Unreachable (106) means the Coinbase International Exchange component is unable to establish connectivity with Coinbase Exchange

40

avgDailyQuantity

Type

Required

The 30 days average daily traded volume, updated daily

711

NoUnderlyings

int

N

Number of underlyings

```
→ 310
```

UnderlyingSecurityType

string

N

Underlying security’s SecurityType.

Possible values are:  
INDEX = Underlying is an index  
FXSPOT = Underlying is a spot pair

## MarketDataRequest (35=V)

Sent by the client to subscribe or unsubscribe to market data for a given security. The market data feed only supports aggregated books on levels 1, 10, or 20 (e.g., 266=Y and 264 = 10).

Tag

Name

Type

Required

Description

262

MDReqID

string

Y

A unique ID assigned by the client that is referenced on market data message relating to this request on in the following messages:

MarketDataRequestReject (35=Y)  
MarketDataSnapshotFullRefresh (35=W)  
MarketDataIncrementalRefresh (35=X)

And also used for a followup MarketDataRequest to unsubscribe (i.e. 263=2).

263

SubscriptionRequestType

char

Y

Type of subscription request:

1 = Subscribe (snapshots + updates)  
2 = Unsubscribe

264

MarketDepth

int

C

How many price levels from BBO inclusive to include for the market data snapshot and updates (defaults to 1). Limited to 1,10, or 20 levels and does not support full book depth.

146

NoRelatedSym

int

N

The number of symbols in the repeating group defined below. If unspecified or set to 0 the request applies to all symbols.

```
→ 55
```

Symbol

string

C

The symbol used to represent the instrument. Required if NoRelatedSym > 0.

```
→ 167
```

SecurityType

string

C

Identifies the type of instrument. Values include:

FXSPOT = Spot pairs  
PERP = Perpetual futures

Required if NoRelatedSym > 0.

## MarketDataRequestReject (35=Y)

Sent by the exchange a MarketDataRequest fails.

Tag

Name

Type

Required

Description

262

MDReqID

string

Y

The same value provided for the MDReqID on the MarketDataRequest message.

281

MDReqRejReason

int

Y

A numerical reject code for common rejects to explain why the MarketDataRequest was rejected:

0 = Unknown symbol  
1 = Duplicate MDReqID  
5 = Unsupported market depth  
7 = Other

58

Text

string

N

## MarketDataSnapshotFullRefresh (35=W)

Sent in response to a MarketDataRequest. This message can contain a 2 sided list of quotes for a single security. It is used to initialize or reset the entire state of the book.

Tag

Name

Type

Required

Description

262

MDReqID

string

Y

The same value provided for the MDReqID on the MarketDataRequest message.

55

Symbol

string

Y

The symbol used to represent the instrument.

167

SecurityType

string

Y

Identifies the type of instrument. Values include:

FXSPOT = Spot pairs  
PERP = Perpetual futures

268

NoMDEntries

int

Y

Marks the beginning of a repeating group for market data entries while also indicating the number of market data entries in the group. Value must be > 0.

```
→ 269
```

MDEntryType

char

Y

Indicates the type of MD entry, values include:

0 = Bid  
1 = Offer  
2 = Trade  
3 = Index price  
4 = Opening price  
6 = Settlement price  
B = Trade quantity  
g = Fair value limit up  
h = Fair value limit down  
f = Final funding rate  
m = Mark price  
p = Predicted funding rate  
C = Open interest

-   Bid indicates the aggregated quantity interested in buying that is resting on the book at a specific price point.
-   Offer indicates the aggregated quantity interested in selling that is resting on the book at a specific price point.
-   Trade indicates the size and price of the most recent trade.
-   Index price is a calculated price based away market quotes used to determine funding for perpetual futures and calculate the fair value.
-   Opening price indicates the probable price at which the market will open and is only populated when the market is in the Halt or Pause statuses.
-   Settlement price indicates the price used to settle open positions which resets risk calculations for margin and unrealized P&L.
-   Trade quantity discloses how much of the instrument (in base asset/size terms) traded over the last 24 hours.
-   Fair value limit up represents the max price the market can trade at.
-   Fair value limit down represents the min price the market can trade at.
-   Final funding rate indicates the funding rate used to keep perpetual futures assets pegged to the index price.
-   Mark price is the median of the best bid, best offer, and last trade which gets used in various risk calculations.
-   Predicted funding rate gives the most recent rolling calculation update of the funding rate prior to completing a funding period for perpetual futures.
-   Open interest gives the total number of active positions held by traders.

```
→ 278
```

MDEntryID

long

C

A unique identifier used to reference the entry, only used for trades.

```
→ 270
```

MDEntryPx

decimal

Y

The price of the MD entry.

```
→ 271
```

MDEntrySize

decimal

C

The size of the MD entry.

Not required when the MDUpdateAction (279) is a delete (2) or the MDEntryType (269) is index price (3), Settlement price (6), Fair value limit up (g), Fair value limit down (h), Final funding rate (f), Mark price (m), or Predicted funding rate (p).

```
→ 60
```

TransactTime

utc timestamp

Y

The timestamp representing when the MD entry was generated.

```
→ 1023
```

MDPriceLevel

int

C

Indicates the level of price from the BBO with the BBO price as level 1.

Not required when the MDUpdateAction (279) is a delete (2) or the MDEntryType (269) is index price (3), Settlement price (6), Fair value limit up (g), Fair value limit down (h), Final funding rate (f), Mark price (m), or Predicted funding rate (p).

```
→ 2446
```

AggressorSide

int

C

Indicates what side was the aggressor causing the match to occur. Only used for trades.

Values include:  
0 = No aggressor  
1 = Buy  
2 = Sell

-   No aggressor (0) indicates trade is caused by the market moving from the Paused to Ready to trade status, which causes all crossed orders to fill at the Opening price

## MarketDataIncrementalRefresh (35=X)

Sent in response to a MarketDataRequest. This message provides incremental updates for quote changes in a single security.

Tag

Name

Type

Required

Description

262

MDReqID

string

Y

The same value provided for the MDReqID on the MarketDataRequest message.

893

LastFragment

boolean

N

Permits clients to process multi-level book (`MDEntryType (269)` of `Bid (0)` or `Offer (1)`) batch fragments atomically. Either set to `YES/Y` (to denote the last fragment of a batch for an instrument) or not populated (to note this fragment is not the last).

Multi-level book updates are batched per instrument, meaning the exchange queues individual price level updates and sends all queued updates together for each instrument on a regular time interval. To limit the size of the `MarketDataIncrementRefresh` message for each batch, a batch may be fragmented into multiple `MarketDataIncrementalRefresh` messages. Clients have the option of either:

1.  Processing each batch fragment immediately, or
2.  Queuing batch fragments until the last fragment is received to process the entire batch at once.

Clients using option 1 can ignore this field. Clients using option 2 should queue all book updates from `MarketDataIncrementalRefresh` messages without `LastFragment` populated until receiving a `MarketDataIncrementalRefresh` with `LastFragment` set to `YES/Y`, at which point they can queue the included updates and process all queued updates for the instrument the included updates correspond to at once.

268

NoMDEntries

int

Y

Marks the beginning of a repeating group for market data entries while also indicating the number of market data entries in the group. Value must be > 0.

```
→ 279
```

MDUpdateAction

char

Y

Indicates the type of update, supported values include:

0 = New  
1 = Change  
2 = Delete

```
→ 269
```

MDEntryType

char

Y

Indicates the type of MD entry, values include:

0 = Bid  
1 = Offer  
2 = Trade  
3 = Index price  
4 = Opening price  
6 = Settlement price  
B = Trade quantity  
g = Fair value limit up  
h = Fair value limit down  
f = Final funding rate  
m = Mark price  
p = Predicted funding rate  
C = Open interest

-   Bid (0) and Offer (1) get disseminated when orders prices within a visible BBO level post to the book, get canceled, or trade.
-   Trade (2) only gets disseminated for trade events once per trade pair (i.e. the matching portions of a buy and sell generates a single trade message).
-   Index (3) price gets disseminated when calculated (typically 1 second).
-   Opening price (4) indicates the probable price at which the market will open and is only populated when the market is in the Halt or Pause statuses.
-   Settlement price (6) gets disseminated per settlement event (typically 5 minutes).
-   Trade quantity (B) gets disseminated daily around 12am UTC.
-   The Fair value limit up (g) and Fair value limit down (h) get disseminated when updated (no more frequently than once a second).
-   The Final funding rate (f) gets disseminated at the end of the funding interval (hourly).
-   The Mark price (m) gets disseminated no more than once per second.
-   The Predicted funding rate (p) gets disseminated no more than once a second.
-   The Open interest (C) gets disseminated no more than once a second.

```
→ 278
```

MDEntryID

long

C

A unique identifier used to reference the entry, set to the match ID when MdEntryType (269) has value trade (2)

```
→ 55
```

Symbol

string

Y

The symbol used to represent the instrument.

```
→ 270
```

MDEntryPx

decimal

C

The price of the MD entry.

```
→ 271
```

MDEntrySize

decimal

C

The size of the MD entry.

Only required when MDEntryType (269) is Bid (0) or Offer (1) while MDUpdateAction (279) is a New (0) or Change (1).

```
→ 273
```

MDEntryTime

utc timestamp

N

Only set when `MDEntryType (269)` is `Bid (0)` or `Offer (0)`. The timestamp when the order event which resulted in the price level update was processed. Clients with multiple connections may use this field to determine which connection received the latest update for a price level.

```
→ 60
```

TransactTime

utc timestamp

Y

The timestamp representing when the MD entry was generated.

```
→ 1023
```

MDPriceLevel

int

C

Indicates the level of price from the BBO with the BBO price as level 1.

Only required when MDEntryType (269) is Bid (0) or Offer (1) while MDUpdateAction (279) is a New (0) or Change (1).

```
→ 2446
```

AggressorSide

int

C

Indicates what side was the aggressor causing the match to occur. May not get set in certain future contexts like an opening auction.

Values include:  
1 = Buy  
2 = Sell