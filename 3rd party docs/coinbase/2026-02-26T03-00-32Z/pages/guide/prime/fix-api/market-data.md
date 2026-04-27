# market data

Prime FIX Market Data provides level 2 market data on FIX 5.0 for Prime Trading Spot instruments. This includes OHLCV (Open, High, Low, Close, Volume), bids and offers, and trade history data. This requires a separate API key from the API key used for FIX Order Entry.

## Getting Started

To get started quickly with FIX Market Data, take a look at the sample [FIX Market Data Go reference application](https://github.com/coinbase-samples/prime-fix-md-go) on Coinbase Samples.

## Logon (A)

Logon messages must conform to FIXT 1.1. For market data sessions, configure your Logon as follows:

Tag

Req

Name

Description

8

Y

BeginString

Must be `FIXT.1.1`

9

Y

BodyLength

Length of body

35

Y

MsgType

Must be `A`

34

Y

MsgSeqNum

Must be `1`

49

Y

SenderCompID

The Service Account ID (on messages from the client)

52

Y

SendingTime

Must be within 5 seconds of server time in UTC

56

Y

TargetCompID

Must be `COIN` (on messages from the client)

95

Y

RawDataLength

Number of bytes in the RawData field

96

Y

RawData

Client message signature (see [Logon](https://developer.chrome.com/prime/fix-api/admin-messages#logon-a))

98

Y

EncryptMethod

Must be `0` (none)

108

Y

HeartBtInt

Heartbeat interval is capped at 300s, defaults to 30s

141

Y

ResetSeqNumFlag

Resets the sequence number. Can be `Y`/`N`

553

Y

Username

Client API Key (Replaces tag 9407)

554

Y

Password

Client API passphrase

1137

Y

DefaultApplVerID

Must be `9` (FIX 5.0 SP2)

9406

Y

DropCopyFlag

Must be `N`

10

Y

CheckSum

Checksum

## Supported Message Types

The Prime FIX Market Data API supports the following message types: **Client to Server:**

-   MarketDataRequest (V)

**Server to Client:**

-   MarketDataRequestReject (Y)
-   MarketDataSnapshotFullRefresh (W)
-   MarketDataIncrementalRefresh (X)
-   SecurityStatus (f)

## MarketDataRequest (V)

Sent by the client when placing a market data request.

Tag

Req

Name

Description

262

Y

MDReqID

Client unique identifier for market data request

263

Y

SubscriptionRequestType

`0` = Snapshot only  
`1` = Snapshot+Updates (Subscribe)  
`2` = Disable previous Snapshot+Update (Unsubscribe)

264

Y

MarketDepth

`0` = Full depth (L2)  
`1` = Top of book  
`N>1` = Report best N price tiers of data

265

N

MDUpdateType

Required if SubscriptionRequestType <263> = 1:  
`0` = Snapshot+Updates  
`1` = Updates only

267

Y

NoMDEntryTypes

Number of MDEntryType <269> fields requested

↳269

Y

MDEntryType

`0` = Bid  
`1` = Offer  
`2` = Trade  
`4` = Open  
`5` = Close  
`7` = High  
`8` = Low  
`B` = Volume

146

Y

NoRelatedSym

Number of Symbols <55> requested

↳55

Y

Symbol

Repeating group of symbols for which the client requests market data

## MarketDataRequestReject (Y)

Sent by the server in case the MarketDataRequest (V) fails.

Tag

Req

Name

Description

262

Y

MDReqID

Client unique identifier for market data request

281

Y

MDReqRejReason

See [MDReqRejReason table](#mdreqrejreason)

58

N

Text

User friendly error message

### MDReqRejReason

Possible values for MDReqRejReason (see [MarketDataRequestReject (Y)](#marketdatarequestreject-y)).

Value

Description

0

Unknown symbol

1

Duplicate MDReqID

2

Insufficient bandwidth

3

Insufficient permission

4

Invalid SubscriptionRequestType <263>

5

Invalid MarketDepth <264>

6

Unsupported MDUpdateType <267>

7

Other

8

Unsupported MDEntryType <269>

## MarketDataSnapshotFullRefresh (W)

Sent by the server to view a new stream of market data information.

Tag

Req

Name

Description

262

Y

MDReqID

Client unique identifier for market data request

55

Y

Symbol

The trading pair from MarketDataRequest

268

Y

NoMDEntries

Number of market data updates in snapshot

911

Y

TotNumReports

Total number of reports being sent in response to a single request

963

Y

ReportID

Unique identifier of the report itself

↳269

Y

MDEntryType

`0` = Bid  
`1` = Offer  
`2` = Trade  
`4` = Open  
`5` = Close  
`7` = High  
`8` = Low  
`B` = Volume

↳278

Y

MDEntryID

Unique identifier for this market data entry

↳83

Y

RptSeq

Public sequence number for each entry in the snapshot by symbol

↳270

N

MDEntryPx

Price of the market data entry (Not present if MDEntryType = B)

↳271

N

MDEntrySize

Volume represented by the market data entry (Not present if MDEntryType = 4, 5, 7, or 8)

↳272

Y

MDEntryDate

Date of the market data entry

↳2446

N

AggressorSide

If MDEntryType = 2 (Trade), the side of the order:  
`1` = Buy  
`2` = Sell

↳273

Y

MDEntryTime

Time of the market data entry

↳453

N

NoPartyIDs

Only present if MDEntryType = 2 (Trade). Will always be 1

↳↳448

N

PartyID

Market Identifier Code (MIC) for Venue

↳↳447

N

PartyIDSource

Will always be `G`, Market Identifier Code (MIC)

↳↳452

N

PartyRole

Will always be `73`, Execution Venue

## MarketDataIncrementalRefresh (X)

Sent by the server to view updates to an existing stream.

Tag

Req

Name

Description

262

Y

MDReqID

Client unique identifier for market data request

55

Y

Symbol

The trading pair from MarketDataRequest

268

Y

NoMDEntries

Number of market data updates in snapshot

↳279

Y

MDUpdateAction

Type of entry update:  
`0` = NEW  
`1` = CHANGE  
`2` = DELETE

↳269

Y

MDEntryType

`0` = Bid  
`1` = Offer  
`2` = Trade  
`4` = Open  
`5` = Close  
`7` = High  
`8` = Low  
`B` = Volume

↳278

Y

MDEntryID

Unique identifier for this market data entry

↳83

Y

RptSeq

Public sequence number for each entry in the snapshot by symbol

↳270

N

MDEntryPx

Price of the market data entry (Not present if MDEntryType = B)

↳271

N

MDEntrySize

Volume represented by the market data entry (Not present if MDEntryType = 4, 5, 7, or 8)

↳272

Y

MDEntryDate

Date of the market data entry

↳2446

N

AggressorSide

If MDEntryType = 2 (Trade), the side of the order:  
`1` = Buy  
`2` = Sell

↳273

Y

MDEntryTime

Time of the market data entry

↳453

N

NoPartyIDs

Only present if MDEntryType = 2 (Trade). Will always be 1

↳↳448

N

PartyID

Market Identifier Code (MIC) for Venue

↳↳447

N

PartyIDSource

Will always be `G`, Market Identifier Code (MIC)

↳↳452

N

PartyRole

Will always be `73`, Execution Venue

## SecurityStatus (f)

Sent by the server once when an existing stream fails, and once when it reconnects.

Tag

Req

Name

Description

55

Y

Symbol

Symbol

326

Y

SecurityTradingStatus

`3` = Resume  
`999` = Market data feed temporarily unavailable

58

Y

Text

Market data feed temporarily unavailable for MDReqID `{MDReqID}` for stream type:  
\- bid/offer  
\- trade  
\- OHLCV