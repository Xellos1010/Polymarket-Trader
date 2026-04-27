# upcoming changes

This page provides information about upcoming changes to Coinbase Prime Broker.

## Prime FIX Execution Report (8) & MiscFeeType (139) changes

*This change will be available in February 2026* We are introducing changes to our FIX API **Execution Report (8)**, that will change how fees are represented using the **MiscFeeType (139)** value.  
We will utilize the NoMiscFees (136) repeating group to represent the different fees paid on an order. The following changes will be made to **MiscFeeType (139)**:

Previous MiscFeeType (139)

New MiscFeeType (139)

Description

1

14

Financing Commission

2

7

Client Commission

3

12

Trading Desk Commission

4

4

Venue Commission (Only present if configured as cost plus fee type)

All fees are represented on the fill level. The sum of all fees paid on a given fill equals the corresponding **Commission (12)** tag.  
The following **Execution Report (8)** is for an entity configured with an “All in” fee model:

```
8=FIX.4.2|9=583|35=8|34=5|49=COIN|50=22d47de1-bc5d-5d17-8028-4ec90e72c1dd|52=20260127-15:55:05.552|56=22d47de1-bc5d-5d17-8028-4ec90e72c1dd|1=314dbd76-4459-41cd-ba9a-dccdd86b44e2|6=87777.73|11=b828ec9e-636e-4b14-bad9-3dec955dcd22|14=0.0001|17=e23cba67-fde7-4ec0-8f97-aaf7ce900128|20=0|30=otc|31=87777.73|32=0.0001|37=f9618938-765c-4834-9361-cdd62a238b2e|38=0.0001|39=2|40=1|54=1|55=BTC-USD|60=20260127-15:55:05.523|78=1|79=314dbd76-4459-41cd-ba9a-dccdd86b44e2|80=0.0001|12=0.02|13=3|136=3|137=0.02|138=USD|139=7|137=0|138=USD|139=12|137=0|138=USD|139=14|120=USD|150=2|151=0.0000|8002=8.77|8006=87900|10=158|

```

**NoMiscFee (136)** repeating groups from above order:

```
- 136=3 -> NoMiscFees = 3
- 137=0.02|138=USD|139=7 -> Client Commission
- 137=0|138=USD|139=12 -> Trading Desk Commission 
- 137=0|138=USD|139=14 -> Financing Commission

```

**12=0.02 -> Commission (Sum of all Fees at the fill-level)**

## Get Staking Status Endpoint

*This change will be available in January 2026* We are introducing a new endpoint to retrieve the status of ongoing staking operations for an Ethereum wallet. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   **Get Staking Status**: `GET /v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/status` - retrieve status of all active staking requests

This endpoint will allow you to view the current status of staking operations in progress, including estimated completion times and amounts for each validator. This feature will only support ETH. **Request:**

```
{
  "portfolio_id": "string",
  "wallet_id": "string"
}

```

**Response:**

```
{
  "portfolio_id": "string",
  "wallet_id": "string",
  "wallet_address": "string",
  "current_timestamp": "string",
  "validators": [
    {
      "validator_address": "string",
      "statuses": [
        {
          "amount": "string",
          "stake_type": "STAKE_TYPE_INITIAL_DEPOSIT | STAKE_TYPE_TOP_UP",
          "estimated_stake_date": "string",
          "estimated_hours_to_stake": "string",
          "requested_at": "string"
        }
      ]
    }
  ]
}

```

## Get Unstaking Status Endpoint

*This change will be available in late October to early November 2025* We are introducing a new endpoint to retrieve the status of ongoing unstaking operations for an Ethereum wallet. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   **Get Unstaking Status**: `GET /v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/unstake/status` - retrieve status of all active unstaking requests

This endpoint will allow you to view the current status of unstaking operations in progress, including estimated completion times and amounts for each validator. This feature will initially only support ETH. **Request:**

```
{
  "portfolio_id": "string",
  "wallet_id": "string"
}

```

**Response:**

```
{
  "portfolio_id": "string",
  "wallet_id": "string",
  "wallet_address": "string",
  "current_timestamp": "string",
  "validators": [
    {
      "validator_address": "string",
      "statuses": [
        {
          "amount": "string",
          "unstake_type": "UNSTAKE_TYPE_PARTIAL | UNSTAKE_TYPE_FULL",
          "finishing_at": "string",
          "remaining_hours": "string",
          "requested_at": "string",
          "estimate_type": "LIVE | INTERIM",
          "estimate_description": "string"
        }
      ]
    }
  ]
}

```

## Prime Multinetwork Support

We’re expanding network support to make capital movement faster, cheaper, and easier:

-   ETH on Base and USDC on Base, Solana, Arbitrum, Optimism, and Avalanche C-Chain can now be transferred efficiently and at no cost within your trading balance on the Prime UI.
-   API support will be available starting **November 3rd**.

Users will now be able to view and manage assets across multiple networks. We will be updating the use of `symbol` to represent assets on different blockchains. This will be available to users as `“network_scoped_symbol”` in the List Entity Assets endpoint for each individual network supported for an asset. Example:

-   If a user wants to represent USDC on the Base network, they will use the `"network_scoped_symbol"` BASEUSDC.
-   When a user calls the ListEntityAssets endpoint for USDC, the response will include a unique `“network_scoped_symbol`” for USDC on each supported network, such as BASEUSDC for Base, OPTUSDC for Optimism, and so on.

Users can use the `“network_scoped_symbol”` to get asset details specific to each network. Users can also add a new query parameter to see combined information for an asset across all supported networks denoted as:

-   `“get_network_unified_transactions”`
-   `”get_network_unified_wallets”`
-   `“get_network_unified_activites”`
-   `“unified_total_balances”`

All Endpoint Changes:

-   **List Portfolio Transactions**
    -   Users can enter a `“network_scoped_symbol”` in the ‘symbols’ query parameter to get transactions for an asset on a specific network
    -   Users can enter optional `“get_network_unified_transactions”` as a boolean query parameter to get transactions across all networks
-   **List Entity Activities**
    -   Users can enter `“network_scoped_symbol”` in the ‘symbols’ query parameter to view activities for an asset on a specific network
    -   Users can enter `“get_network_unified_activities”` as a boolean query parameter to get activities across all networks
-   **List Portfolio Activities**
    -   Users can enter `“network_scoped_symbol”` in the ‘symbols’ query parameter to view portfolio activities for an asset on a specific network
    -   Users can enter `“get_network_unified_activities”` as a boolean query parameter to get activities across all networks

Network Scoped Symbols:

Asset - Symbol

Network Name

Network\_Scoped\_Symbol

USDC

Ethereum

usdc

USDC

Avalanche C-Chain

avausdc

USDC

Optimism

optusdc

USDC

Base

baseusdc

USDC

Solana

splusdc

USDC

Arbitrum

arbusdc

ETH

Ethereum

eth

ETH

Base

baseeth

## Financing API Enhancements

*This change will be published in late October to early November 2025* We are implementing changes to an existing endpoint and adding two new endpoints to enhance the API experience for users of the Trade Finance and Bilateral Lending products.

### List TF Obligations Endpoint

This new endpoint returns all open trade finance obligations.

```
{
  "obligations": [
    {
      "portfolio_id": "D7927E4F-DE95-4CF2-BD4C-00477BC25D70",
      "symbol":"USD",
      "amount":"20339.64",
      "notional_amount": "23928.87",
      "due_date": "2025-08-13T22:00:00Z"
    }
  ]
}

```

### List Financing Eligible Assets Endpoint

This new endpoint returns all financing eligible assets and their associated trade finance credit utilization parameters.

```
{
  "assets": [
    {
      "asset": "XYO", 
      "asset_adjustment": "0.4", 
      "liability_adjustment": "3"
    },
    {
      "asset": "XRP", 
      "asset_adjustment": "0.7", 
      "liability_adjustment": "1.5"
    },
    ...
  ]
}

```

### List Interest Accruals Endpoint

This endpoint will now additionally include interest accruals for Bilateral Lending (in addition to Trade Finance and Portfolio Margin, as currently implemented)

## User generated IDs maximum length requirement

*Update: This will be live mid-October 2025* A maximum length requirement of 128 characters will be enforced for client generated IDs. Invalid argument errors will be raised if the IDs are longer than this limit. The affected fields currently are:

-   Create Portfolio Allocations: [allocation\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/create-portfolio-allocations#body-allocation-id), [allocation\_leg\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/create-portfolio-allocations#body-allocation-legs-allocation-leg-id)
-   Create Portfolio Net Allocations: [allocation\_leg\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/create-portfolio-net-allocations#body-allocation-legs-allocation-leg-id), [netting\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/create-portfolio-net-allocations#body-netting-id)
-   Accept Quote: [client\_order\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/accept-quote#body-client-order-id)
-   Create Order: [client\_order\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order#body-client-order-id)
-   Create Quote Request: [client\_quote\_id](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-quote-request#body-client-quote-id)

## FIX Market Data

*Update: This will be live in Q3* We will be adding level 2 market data on FIX 5.0 for Prime Trading Spot instruments. This includes OHLCV (Open, High, Low, Close, Volume), bids and offers, and trade history data. This will require a dedicated API key with READ permissions. Logon messages must conform to FIXT 1.1, for example:

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

Client message signature (see [Logon](https://developer.chrome.com/prime/fix-api/messages#logon-a))

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

[Prime FIX API:](https://developer.chrome.com/prime/fix-api/messages)

-   Server will support a new message type sent by the client: **MarketDataRequest <V>**
-   Server will send a new message type: **MarketDataRequestReject <Y>**
-   Server will send a new message type: **MarketDataSnapshotFullRefresh <W>**
-   Server will send a new message type: **MarketDataIncrementalRefresh <X>**
-   Server will send a new message type: **SecurityStatus <f>**

### MarketDataRequest (V)

Sent by the client when placing a market data request

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
`N`\>1 = Report best N price tiers of data

265

N

MDUpdateType

Required if SubscriptionRequestType <263> = `1`:  
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

### MarketDataRequestReject (Y)

Sent by the server in case the [MarketDataRequest (V)](#marketdatarequest-v) fails

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

#### MDReqRejReason

Possible values for MDReqRejReason (see [MarketDataRequestReject (Y)](#marketdatarequestreject-y))

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

### MarketDataSnapshotFullRefresh (W)

Sent by the server to view a new stream of market data information

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

Price of the market data entry (Not present if MDEntryType = `B`)

↳271

N

MDEntrySize

Volume represented by the market data entry (Not present if MDEntryType = `4`, `5`, `7`, or `8`)

↳272

Y

MDEntryDate

Date of the market data entry

↳2446

N

AggressorSide

If MDEntryType = `2` (Trade), the side of the order:  
`1` = Buy  
`2` = Sell

↳273

Y

MDEntryTime

Time of the market data entry

↳453

N

NoPartyIDs

Only present if MdEntryType = `2` (Trade). Will always be `1`

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

### MarketDataIncrementalRefresh (X)

Sent by the server to view updates to an existing stream

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

Price of the market data entry (Not present if MDEntryType = `B`)

↳271

N

MDEntrySize

Volume represented by the market data entry (Not present if MDEntryType = `4`, `5`, `7`, or `8`)

↳272

Y

MDEntryDate

Date of the market data entry

↳2446

N

AggressorSide

If MDEntryType = `2` (Trade), the side of the order:  
`1`\=Buy  
`2`\=Sell

↳273

Y

MDEntryTime

Time of the market data entry

↳453

N

NoPartyIDs

Only present if MdEntryType = `2` (Trade). Will always be `1`

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

### SecurityStatus (f)

Sent by the server once when an existing stream fails, and once when it reconnects

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

`3`\=Resume  
`999`\=Market data feed temporarily unavailable

58

Y

Text

Market data feed temporarily unavailable for MDReqID `MDReqID` for stream type:  
`bid/offer`  
`trade`  
`OHLCV`

## Settlement Currency

*Updating: This will be live in December 2025* We are introducing support for specifying a settlement currency for orders. With this update, clients can access USD trading pairs using USDC positions, enabling them to buy with USDC balances and settle trades directly into USDC positions.

### REST API Changes

-   Updated [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) with new optional parameter `settl_currency`.
-   Updated [Get Order By Order Id](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id#response-order) with the new field `client_product_id`.

### Websocket Changes

-   The orders channel response will include SettlCurrency to indicate the settlement currency for each order if different from the quote currency.

### FIX API Changes

-   The following fields will be added to [New Order Single (D)](https://docs.cdp.coinbase.com/prime/fix-api/order-entry-messages#new-order-single-d)

Tag

Req

Name

Notes

120

N

SettlCurrency

Optional, specifies the settle currency if different from quote currency, currently only USDC is supported for USD pairs

-   The following field will be added to [Execution Report (8)](https://docs.cdp.coinbase.com/prime/fix-api/order-entry-messages#execution-report-8)

Tag

Name

Description

120

SettlCurrency

Settle currency of the order if different from quote currency

## FCM Setting Endpoints

*This will be live in December 2025* We are introducing new endpoints to manage Futures Commission Merchant (FCM) settings for entity.

-   **Get FCM Settings**: `GET /v1/entities/{entity_id}/futures/settings` - retrieve current FCM settings for an entity
-   **Update FCM Settings**: `POST /v1/entities/{entity_id}/futures/settings` - update FCM settings for an entity

## Get FCM Equity Endpoint

*This change will be available in February 2026* We are introducing a new endpoint to retrieve equity information for Futures Commission Merchant (FCM) accounts. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   **Get FCM Equity**: `GET /v1/entities/{entity_id}/futures/equity` - retrieve current equity data for an entity’s FCM account

This endpoint provides visibility into your FCM account equity metrics, including end-of-day balances, unrealized P&L, excess/deficit calculations, and funds available for sweeping. **Response:**

```
{
  "eod_account_equity": "10000.00",
  "eod_unrealized_pnl": "100.00",
  "current_excess_deficit": "1000.00",
  "available_to_sweep": "500.00"
}

```