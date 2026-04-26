# market data

The Coinbase Derivatives Exchange (CDE) FIX Market Data API allows market participants to receive market information including orders, trades, market state changes and instrument definitions.

Market Data Message Types

Client and server use the following administrative messages:

-   [MarketDataRequest (35=V)](#marketdatarequest-35v):
-   [MarketDataRequestReject (35=Y)](#marketdatarequestreject-35y):
-   [MarketDataSnapshotFullRefresh (35=W)](#marketdatasnapshotfullrefresh-35w):
-   [MarketDataIncrementalRefresh (35=X)](#marketdataincrementalrefresh-35x):

## MarketDataRequest (35=V)

Tag

Name

FIX Type

Req

Description

262

MDReqID

String

Y

Unique client ID representing the MarketDataRequest.

263

SubscriptionRequestType

Int

Y

`1` = Subscribe for snapshots and updates. `2` = Unsubscribe (send with MDReqID)

146

NoRelatedSym

NumInGroup

N

Number of securities to subscribe to. If not specified, subscribes to all securities

→55

Symbol

String

N

Ticker symbol.

→167

SecurityType

String

N

See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

## MarketDataRequestReject (35=Y)

Tag

Name

FIX Type

Req

Description

262

MDReqID

String

Y

Unique client ID representing the MarketDataRequest.

281

MDReqRejReason

String

N

Reason for rejection of MarketDataRequest (as a numerical value).

58

Text

String

N

Reason for rejection of MarketDataRequest (as text).

## MarketDataSnapshotFullRefresh (35=W)

Tag

Name

FIX Type

Req

Description

262

MDReqID

String

Y

Unique client ID representing the MarketDataRequest.

55

Symbol

String

Y

Ticker symbol.

167

SecurityType

String

Y

See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

762

SecuritySubType

String

C

Represents spread strategy if applicable. See [SecuritySubType (762) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitysubtype-762).

268

NoMDEntries

NumInGroup

Y

Number of MDEntry groups included in a Market Data message.

→ 269

MDEntryType

Int

Y

See [MDEntryType (269) code set](https://developer.chrome.com/derivatives/fix/code-sets#mdentrytype-269).

→ 278

MDEntryID

String

N

Unique exchange ID representing the instrument, session, and entry type.

Active entries with duplicate values should be **replaced** (delete is not allowed):

-   MDUpdateAction=New/Change (`279= 0/1`): Replace active entry with the same MDEntryID (278)

→ 270

MDEntryPx

Price

N

All-in rate

→ 271

MDEntrySize

Qty

N

Entry size/quantity

→ 110

MinQty

Qty

N

Minimum fill size associated with the amount.

→ 60

TransactTime

UTCTimestamp(24)

Y

GMT time of event, in [microseconds](https://en.wikipedia.org/wiki/Microsecond#:~:text=A%20microsecond%20is%20equal%20to,tens%20or%20hundreds%20of%20microseconds)

→ 272

MDEntryDate

UTCDateOnly

C

Date of MDEntryType (269) = Funding Time (t) message

→ 273

MDEntryTime

UTCTimeOnly

C

Time of MDEntryType (269) = Funding Time (t) message

→ 286

OpenCloseSettllFlag

Int

C

Represents Open Price entry:

-   0=Daily Open Price
-   5=Indicative Opening Price (IOP)

→ 5796

TradingReferenceDate

LocalMktDate

C

Usually the current trading date, except for open interest and closing price (which is usually previous trading date). Value corresponds to [MDEntryType (269)](https://developer.chrome.com/derivatives/fix/code-sets#mdentrytype-269).

## MarketDataIncrementalRefresh (35=X)

Tag

Name

FIX Type

Req

Description

262

MDReqID

String

Y

Unique client ID representing the MarketDataRequest.

266

AggregatedBook

Char

C

Represents whether or not book entries are aggregated.

268

NoMDEntries

NumInGroup

Y

Repeating group. Number of entries in the Market Data message.

→279

MDUpdateAction

Int

Y

Represents type of Market Data update action. `279=0` (New), `279=1` (Change), `279=2` (Delete).

→ 269

MDEntryType

Int

Y

See [MDEntryType (269) code set](https://developer.chrome.com/derivatives/fix/code-sets#mdentrytype-269).

→ 278

MDEntryID

String

N

Order ID assigned by the exchange that is unique per instrument, session, and entry type.

Active entries with duplicate values should be **replaced or deleted**:

-   MDUpdateAction = New/Change (`279=0/1`): Replace active entry with the same MDEntryID (278)
-   MDUpdateAction = Delete (`279=2`): Delete active entry with same MDEntryID

→ 55

Symbol

String

Y

Ticker symbol.

→ 167

SecurityType

String

Y

See [SecurityType (167) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitytype-167--legsecuritytype-609).

→ 762

SecuritySubType

String

C

See [SecuritySubType (762) code set](https://developer.chrome.com/derivatives/fix/code-sets#securitysubtype-762) values. Represents spread strategy if applicable

→ 270

MDEntryPx

Price

N

All-in rate. Required when MDUpdateAction = New (`279=0`) or Change (`279=1`)

→ 271

MDEntrySize

Qty

N

Entry size/quantity. Required when MDUpdateAction = New (`279=0`) or Change (`279=1`)

→ 110

MinQty

Qty

N

Minimum fill size associated with the amount.

→ 60

TransactTime

UTCTimestamp(24)

Y

GMT time of event, in [microseconds](https://en.wikipedia.org/wiki/Microsecond#:~:text=A%20microsecond%20is%20equal%20to,tens%20or%20hundreds%20of%20microseconds)

→ 272

MDEntryDate

UTCDateOnly

C

Date of MDEntryType (269) = Funding Time (t) message

→ 273

MDEntryTime

UTCTimeOnly

C

Time of MDEntryType (269) = Funding Time (t) message

→ 286

OpenCloseSettllFlag

Int

C

Flag describing Open Price entry . `0`\=Daily Open Price. `5`\=Indicative Opening Price (IOP)

→ 5796

TradingReferenceDate

LocalMktDate

C

Current trading date. Value corresponds to [MDEntryType (269) code set](https://developer.chrome.com/derivatives/fix/code-sets#mdentrytype-269).

→ 5797

AggressorSide

Int

C

Represents the aggressor side in a transaction:

-   `0`\=No Aggressor
-   `1`\=Buy
-   `2`\=Sell

`AggressorSide` is sent for [MDEntryType](https://developer.chrome.com/derivatives/fix/code-sets#mdentrytype-269) = “Trade” (`269=2`)