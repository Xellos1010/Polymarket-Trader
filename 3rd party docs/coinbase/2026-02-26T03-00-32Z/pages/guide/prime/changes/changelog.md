# changelog

These release notes list changes to Coinbase Prime Broker.

### 2026-JAN-27

Introducing PEG (Pegged) orders, which dynamically adjust their price based on market conditions while maintaining execution discretion. PEG orders “peg” their price to a market reference (best bid/offer) with a configurable offset.

#### REST API Changes

[Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) now supports the following new parameters for PEG orders:
    -   `type`: must be `"PEG"`
    -   `time_in_force`: must be `"GOOD_UNTIL_CANCELLED"` or `"GOOD_UNTIL_DATE_TIME"`
    -   `limit_price`: acts as ceiling (BUY) or floor (SELL)
    -   `peg_offset_type`: `"PEG_OFFSET_TYPE_PRICE"`, `"PEG_OFFSET_TYPE_BPS"`, or `"PEG_OFFSET_TYPE_DEPTH"`
    -   `offset`: offset value (0 means peg to best bid/offer)
    -   `wig_level`: optional - “Would if Good” level for aggressive fills
    -   `expiry_time`: optional - required if `time_in_force` is `"GOOD_UNTIL_DATE_TIME"`

#### FIX API Changes

[Prime FIX API:](https://developer.chrome.com/prime/fix-api/order-entry-messages)

-   [New Order Single (D)](https://developer.chrome.com/prime/fix-api/order-entry-messages#new-order-single-d) now supports the following new fields for PEG orders:
    -   `PegOffsetType` (836): `0` = Price, `1` = Basis Points, `4` = Cumulative depth in base units
    -   `PegOffsetValue` (211): The offset value in price, basis points, or cumulative depth units
    -   `WigLevel` (8007): Optional - the target price to actively fill the remaining balance
-   [OrdType Values](https://developer.chrome.com/prime/fix-api/order-entry-messages#ordtype-values) now includes `P` for Adaptive Peg order
-   [TargetStrategy Values](https://developer.chrome.com/prime/fix-api/order-entry-messages#targetstrategy-values) now includes `P` for Adaptive Peg order (OrdType must be `P`, TimeInForce must be `1` (GTC) or `6` (GTD))

### 2025-NOV-3

Users are now able to view and manage assets across multiple networks. We’ve updated the use of the `symbol` parameter to represent assets on different blockchains. This will be available to users as `“network_scoped_symbol”`. You can find the details in the **ListEntityAssets** endpoint for each individual network supported for an asset: Network Scoped Symbols:

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

Example:

-   If a user wants to represent USDC on the Base network, they can use the `"network_scoped_symbol"` BASEUSDC.
-   When a user calls the ListEntityAssets endpoint for USDC, the response includes a unique `“network_scoped_symbol`” for USDC on each supported network, such as BASEUSDC for Base, OPTUSDC for Optimism, and so on.

A full list of endpoint-specific changes can be found below:

-   **List Entity Assets**
    -   Returns a new field, `“network_scoped_symbol”` for each asset supported by the entity
-   **List Wallet Transactions**
    -   Returns all transactions for the given wallet across all supported networks including `“network_scoped_symbol”` as the symbol field
-   **List Portfolio Balances**
    -   Users can enter a `“network_scoped_symbol”` in the `symbols` query parameter to see balances for an asset on a specific network,
    -   Users can enter an optional query parameter with balance\_type `“UNIFIED_TOTAL_BALANCES”` to get balances across all networks
-   **List Entity Balances**
    -   Users can enter a `“network_scoped_symbol”` in the ‘symbols’ query parameter to view balances for an asset on a specific network
    -   Users can enter an optional boolean query parameter with balance\_type `“UNIFIED_TOTAL_BALANCES”` to get balances across all networks
-   **List Portfolio Wallet**
    -   Users can enter a `“network_scoped_symbol”` in the `symbols` query parameter to get wallets for an asset on a specific network
    -   Users can enter `“get_network_unified_wallets”` as a boolean query parameter to get wallets across all networks
-   **List Wallet Addresses**
    -   `Network_id` is now an optional field and will return all wallet deposit addresses for the given network. If no network\_id provided, it will return all deposit addresses for that wallet across networks

### 2025-OCT-30

Added reward metadata to reward transaction responses, providing additional information about the reward received. This is only supported currently for Ethereum and Solana rewards, other assets reward transaction subtype will be unknown until supported. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Get Transaction by Transaction ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id), [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions), and [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) now include `reward_metadata` in reward transaction responses

The `reward_metadata` field contains:

-   `subtype` - The subtype of reward received:
    -   `REWARD_SUBTYPE_UNKNOWN` - For reward subtypes not yet supported in the API response
    -   `MEV_REWARD` - Maximum Extractable Value rewards
    -   `INFLATION_REWARD` - Inflation rewards
    -   `BLOCK_REWARD` - Block rewards, i.e. solana block rewards
    -   `VALIDATOR_REWARD` - Validator rewards, i.e. ethereum validator (consensus layer) rewards
    -   `TRANSACTION_REWARD` - Transaction fee rewards, i.e. ethereum transaction (execution layer) rewards
    -   `STAKING_FEE_REBATE_REWARD` - Staking fee rebates that coinbase pays to eligible delegators

This metadata is available for transactions with type `REWARD`.

### 2025-OCT-23

Added a new endpoint to [Prime Rest API](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction):

-   [Preview Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/preview-unstake): `POST /v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/unstake/preview` - preview an unstaking request and see the estimated amount that would be unstaked

This endpoint allows you to preview an unstaking operation before executing it, helping you understand the actual unstaking amount that will be processed. This feature currently only supports ETH wallets, and can be used to predict validator exit scenarios. **Request:**

**Response:**

```
{
  "estimated_amount": "string"
}

```

### 2025-OCT-23

Added Client Order ID (ClOrdID) idempotency across all prime trading surfaces (FIX, REST, & UI). Prime trading now enforces that ClOrdIDs are unique for 24hrs, for both open and closed orders. This change is scoped at the portfolio level. Consider the following scenario:

1.  Client places a `NewOrderSingle (D)` with `ClOrdID (11) = "test-clordID-1"`
2.  The order, `"test-clordID-1"` fills
3.  Client places another `NewOrderSingle (D)` with the same `clordID (11) "test-clordID-1"`
4.  This second order is rejected with the error message: `"duplicate client order ID"`

### 2025-OCT-15

Added new security and permission features for API keys:

-   API Key Scopes – Set fine-grained permissions:
    -   Endpoint-specific access (e.g., allow Get Wallet Balance but not Get Portfolio Balances)
    -   Category-level READ/WRITE access (e.g., all READ order endpoints only)
    -   Broad ALL READ / ALL WRITE access across categories

These scopes automatically apply to new endpoints as they are added, requiring no additional setup.

### 2025-OCT-10

Added new Cross Margin Overview API to get detailed overview for the Cross Margin product. REST Path: `GET /v1/entities/<prime_entity_id>/cross_margin` Example Response Structure:

```
{
    "overview": {
        "control_status": "TRADES_AND_WITHDRAWALS",
        "call_status": "ENTITY_NO_CALL",
        "margin_level": "HEALTHY_THRESHOLD",
        "margin_summary": {
            "margin_requirement": "0.00",
            "account_equity": "281.02",
            "margin_excess_shortfall": "281.02",
            "consumed_credit": "10.00",
            "xm_credit_limit": "4000.00",
            "xm_margin_limit": "0.00",
            "spot_equity": "-0.12",
            "futures_equity": "281.15",
            "risk_netting_info": {
                "nodal_margin_requirement": "0.00",
                "portfolio_margin_requirement": "0.00",
                "integrated_portfolio_margin_requirement": "0.00",
                "ineligible_futures_margin_requirement": "0.00",
                "position_margin_requirement": "0.00",
                "portfolio_margin_addon": "0.00",
                "integrated_position_margin_requirement": "0.00",
                "integrated_portfolio_margin_addon": "0.00",
                "netted_futures_notional": "0.00",
                "total_gmv_basis": "0.00",
                "ipm_cash_balance": "-0.12",
                "integrated_scenario_addon": {
                    "amount": "0.00",
                    "add_on_type": "SINGLE_COIN_STRESS"
                },
                "all_integrated_scenario_addons": [
                    {
                        "amount": "0.00",
                        "add_on_type": "SINGLE_COIN_STRESS"
                    },
                    {
                        "amount": "0.00",
                        "add_on_type": "CONCENTRATION_STRESS"
                    },
                    {
                        "amount": "0.00",
                        "add_on_type": "MACRO_STRESS"
                    },
                    {
                        "amount": "0.00",
                        "add_on_type": "SHORT_BIASED_STRESS"
                    }
                ],
                "xm_positions": [
                    {
                        "currency": "USDC",
                        "market_price": "1.00",
                        "margin_eligible": false,
                        "market_cap": "0.00",
                        "adv30_days": "0",
                        "hist5d_vol": "0",
                        "hist30d_vol": "0",
                        "hist90d_vol": "0",
                        "margin_requirement": "0",
                        "spot_balance": "-10.00",
                        "spot_balance_notional": "-10.00",
                        "spot_total_position_margin": "0.00",
                        "futures_balance": "0.00",
                        "futures_balance_notional": "0.00",
                        "futures_total_position_margin": "0.00",
                        "gmv_basis": "0.00",
                        "base_requirement": "0.00",
                        "liq_shorts_add_on": "0.00",
                        "liq_longs_add_on": "0.00",
                        "vol_shorts_add_on": "0",
                        "vol_longs_add_on": "0",
                        "vol5days_add_on": "0",
                        "vol30days_add_on": "0",
                        "vol90days_add_on": "0",
                        "total_position_margin": "0.00"
                    }
                ]
            }
        },
        "active_margin_calls": [
            {
                "margin_call_id": "ce5fbaef-d3da-4818-920c-9e855750b863",
                "currency": "USD",
                "initial_notional_amount": "32083.26",
                "outstanding_notional_amount": "32083.26",
                "margin_call_type": "CALL_TYPE_STANDARD",
                "margin_call_status": "CALL_STATUS_OPEN",
                "called_with_margin_level": "DEFICIT_THRESHOLD",
                "called_with_margin_summary": {
                    "margin_requirement": "0.00",
                    "account_equity": "281.02",
                    "margin_excess_shortfall": "281.02",
                    "consumed_credit": "10.00",
                    "xm_credit_limit": "4000.00",
                    "xm_margin_limit": "0.00",
                    "spot_equity": "-0.12",
                    "futures_equity": "281.15"
                },
                "due_at": "2025-09-25T12:05:02.880848Z",
                "created_at": "2025-09-25T00:05:02.880848Z",
                "updated_at": "2025-09-25T00:05:02.880848Z"
            }
        ],
        "active_loans": [
            {
                "loan_id": "0ceae6a1-7d4f-496a-92bb-6ff085a972c2",
                "loan_party": "CBE",
                "principal_currency": "USDC",
                "principal_currency_market_price": "1",
                "initial_principal_amount": "10",
                "outstanding_principal_amount": "10",
                "created_at": "2025-09-16T16:27:52.284368Z",
                "updated_at": "2025-09-25T00:05:02.880848Z"
            }
        ]
    }
}

```

### 2025-OCT-12

Added new GET endpoint to return candles of specified symbols. The request requires the following parameters: REST Path: `GET /v1/portfolios/{portfolio_id}/candles`

```
{
  "product_id": "string",
  "start": "string", // ISO 8601 timestamp e.g 2025-01-10T12:00:00
  "end": "string", // ISO 8601 timestamp e.g 2025-01-10T12:00:00
  "granularity": "string"
}

```

Response:

```
{
  "candles": [
    {
      "timestamp": "string", // ISO 8601 timestamp format
      "open": "string",
      "high": "string",
      "low": "string",
      "close": "string",
      "volume": "string"
    }
  ]
}

```

### 2025-JUL-02

Added new endpoint to get the margin call details for a given entity ID. This GET endpoint returns a list of margin calls for the specified entity. The request requires the following parameters:

```
{
  "entity_id": "string"
}

```

The response consists of the following:

```
{
  "margin_calls": [
    {
      "type": "enum", // "URGENT" or "REGULAR"
      "state": "enum",  // "CLOSED", "ROLLED_OVER", "DEFAULT", "OFFICIAL"
      "initial_amount": "string",
      "remaining_amount": "string",
      "business_date": "string", // UNIX timestamp e.g 1596640920
      "cure_deadline": "string"  // UNIX timestamp e.g 1596650920
    }
  ]
}

```

### 2025-JUN-26

Added new endpoint to get the risk limits for a given portfolio ID. This GET endpoint returns the risk limits for the specified portfolio. The request requires the following parameters:

```
{
  "entity_id": "string",
}

```

The response consists of the following:

```
{
  "cfm_risk_limit": "string",
  "cfm_risk_limit_utilization": "string",
  "cfm_total_margin": "string",
  "cfm_delta_ote": "string",
  "cfm_unsettled_realized_pnl": "string",
  "cfm_unsettled_accrued_funding_pnl": "string"
}

```

### 2025-Q3

Added following fields to [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Get Order By Order ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id) and [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders) now supports:
    -   `raise_exact` which returns a boolean indicating if an order is a raise exact order
    -   `display_base_size` and `display_quote_size` which returns the display size in the order currency. If the order does not have a display size, the response is empty.
-   [Get Order Preview](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-preview) now supports:
    -   `raise_exact` which returns a boolean indicating if an order is a raise exact order
    -   `display_base_size` and `display_quote_size` which returns the display size in the order currency. If the order does not have a display size, the response is empty.
    -   `stop_price` for a Stop Limit order displayed in quote currency
-   [List Order Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-order-fills) and [List Portfolio Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-fills) now supports:
    -   Add `venue_fee` which returns the venue fee in quote currency if the entity is enabled for cost-plus pricing.
    -   Add `CES_commission` which returns the Client Execution Services commission of the trade.

**Important:** When using the above endpoints, please note the new parameter validation rules to avoid invalid argument errors:

-   If user enters both `base_quantity` and `quote_value`, an error is thrown
-   If user enters `base_quantity` and `display_quote_size`, an error is thrown
-   If user enters `quote_value` and `display_base_size`, an error is thrown
-   If user enters both `display_quote_size` and `display_base_size`, an error is thrown

Added following fields to [Prime Websocket API:](https://developer.chrome.com/prime/websocket-feed/overview)

-   [Orders Channel](https://developer.chrome.com/prime/websocket-feed/channels#orders-channel) now supports:
    -   `user_id` which returns the unique user\_id in each response
    -   `venue_fee` which returns the venue fee in quote currency if entity is enabled for cost-plus pricing.
    -   `commission` which returns the trading fee of the order.
    -   `CES_commission` which returns the Client Execution Services commission of the trade.
-   [Products Channel](https://developer.chrome.com/prime/websocket-feed/channels#products-channel) now supports:
    -   `price_increment`
    -   `permissions` which always returns PRODUCT\_PERMISSION\_READ and if the client can trade the product pair also returns PRODUCT\_PERMISSION\_TRADE

Added following fields to [Prime FIX API:](https://developer.chrome.com/prime/fix-api/connectivity)

-   [Execution Report (8)](https://developer.chrome.com/prime/fix-api/messages#execution-report-8)
    -   `TimeInForce` which returns the Time in force for the order
    -   `StopPrice` for a Stop Limit order displayed in quote currency
    -   `MaxShow` displays Maximum quantity within an order to be shown to other customers (Display Size). Only present on LIMIT orders.
    -   `FilledValue` presents the sum of fills (inclusive of fees) in quote units of an order

### 2025-OCT-09

Added support for the Ethereum Pectra upgrade with significant enhancements to ETH staking functionality. **Flexible Staking Amounts:**

-   Stake any amount between 32 ETH and 180,000 ETH (not restricted to multiples of 32 ETH)
-   Minimum stake amount remains 32 ETH
-   Maximum stake amount per wallet: 180,000 ETH (100 validators × 1,800 ETH target size)

**Enhanced Unstaking Capabilities:**

-   Minimum unstake amount reduced from 32 ETH to 1 ETH
-   Partial unstaking now supports flexible amounts (not restricted to multiples of 32 ETH)
-   Validators with a balance less than 32 ETH will be automatically fully unstaked

**New Claim Rewards API:** Added new endpoint to claim staking rewards without unstaking. After the Pectra upgrade, validator rewards automatically compound and are claimed when you fully unstake. Use this endpoint to withdraw rewards independently. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Claim Rewards](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/claim-wallet-staking-rewards-alpha): `POST /v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/claim_rewards` - claim staking rewards

**Request:**

```
{
  "idempotency_key": "string",
  "inputs": {
    "amount": "string"  // Optional: Amount to claim (ETH only). If omitted, claims maximum available
  }
}

```

**Response:**

```
{
  "wallet_id": "string",
  "transaction_id": "string",
  "activity_id": "string"
}

```

### 2025-OCT-2

Added support for editing open LIMIT, ST0P-LIMIT, TWAP, or VWAP orders via REST and FIX

-   Prime REST API: [Edit Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/edit-order-beta)
-   Prime FIX API: [Order Cancel/Replace Request <G>](https://developer.chrome.com/prime/fix-api/messages#order-cancel-replace-request-g)

### 2025-SEP-16

RFQ now supports size in the base or quote asset. To support this offering we’ve be deprecated the `rfq_product_detail` fields: `min_notional_size` and `max_notional_size` and introduce the new fields:

-   `min_base_size`
-   `max_base_size`
-   `min_quote_size`
-   `max_quote_size`

### 2025-AUG-01

Added support for counterparty transfers via the Coinbase Transfer Network, enabling instant and fee-less transfers between Coinbase customers. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Get Portfolio Counterparty ID](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/get-portfolio-counterparty-id): `GET /v1/portfolios/{portfolio_id}/counterparty` - retrieve your portfolio’s unique counterparty ID for receiving transfers
-   [Create Withdrawal](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-withdrawal) now supports `DESTINATION_COUNTERPARTY` destination type for sending transfers to other Coinbase customers

**Usage Examples:**

1.  **Get your portfolio’s counterparty ID:**

Request:

```
{
  "portfolio_id": "11111111-1111-1111-1111-111111111111"
}

```

Response:

```
{
  "counterparty": {
    "counterparty_id": "CB12345678"
  }
}

```

2.  **Send transfer to another portfolio:**

Request:

```
{
  "portfolio_id": "11111111-1111-1111-1111-111111111111",
  "wallet_id": "22222222-2222-2222-2222-222222222222",
  "amount": "10",
  "destination_type": "DESTINATION_COUNTERPARTY",
  "idempotency_key": "33333333-3333-3333-3333-333333333333",
  "currency_symbol": "BTC",
  "counterparty": {
    "counterparty_id": "CB12345678"
  }
}

```

Response:

```
{
    "activity_id": "44444444-4444-4444-4444-444444444444",
    "approval_url": "https://prime.coinbase.com/portfolio/11111111-1111-1111-1111-111111111111/activity/44444444-4444-4444-4444-444444444444",
    "symbol": "BTC",
    "amount": "10",
    "fee": "",
    "destination_type": "Counterparty",
    "source_type": "BTC Trading Balance",
    "counterparty_destination": {
        "counterparty_id": "CB12345678"
    },
    "transaction_id": "55555555-5555-5555-5555-555555555555"
}

```

### 2025-JUL-29

Staking Solana (SOL) now enabled with validator address support. [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Initiate Staking](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-stake-or-delegate) now supports optional `validator_address` parameter
    -   For SOL staking only, optional `validator_address` must be the vote account address. Defaults to Coinbase validator when not specified.
-   [Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-unstake-a-wallet) input schema updated to exclude `validator_address` parameter (staking-only)
-   Updated `amount` parameter descriptions to specify “(ETH only)” behavior for both staking and unstaking

### 2025-JUL-22

Added support for Post-Only instructions on limit orders [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) now supports POST\_ONLY on type: LIMIT

[Prime FIX API:](https://developer.chrome.com/prime/fix-api/messages)

-   Additional value for NewOrder (D): Support for Post-Only Orders
-   Tag 18 (ExecInst): A (Post-Only)
-   Supported TimeInForce values:
    -   6 = GTD (Good Till Date)
    -   1 = GTC (Good Till Cancel)
-   Notes
    -   Post-Only orders (18=A) are now supported with TImeInForce values of GTD and GTC

### 2025-JUL-14

-   Added the following fields to [Products](https://developer.chrome.com/api-reference/prime-api/rest-api/products/list-portfolio-products) and Products Channel for Websocket feed inside `rfq_product_detail`:
    -   `min_base_size` The minimum the size can be when submitted in the base asset
    -   `max_base_size` The maximum the size can be when submitted in the base asset
    -   `min_quote_size` The minimum the size can be when submitted in the quote asset
    -   `max_quote_size` The maximum the size can be when submitted in the quote asset

### 2025-JUN-18

-   Added support for Prime Limit **Fill or Kill** (`FOK`) order with size in quote order instructions.
    -   FIX `CashOrderQty(152)` with `TimeInForce(59)` FILL\_OR\_KILL
    -   REST create order `quote_value` with `time_in_force` FILL\_OR\_KILL

### 2025-APR-28

Added new POST endpoints to create staking/unstaking transactions. These endpoints initially support ETH only. Staking is a request to stake or delegate funds to a validator, and unstaking is a request to unstake delegated or staked funds in a wallet. The request requires the following parameters:

```
{
  "portfolio_id": "string",
  "wallet_id": "string",
  "body": {
    "idempotency_key": "string", // The idempotency key associated with this transfer
    "inputs": "object" // String map of inputs for the given action.
  }
}

```

The response consists of the following:

```
{
  "wallet_id": "string",
  "transaction_id": "string",
  "activity_id": "string"
}

```

### 2025-APR-15

Added optional query parameter to the REST API [Get Portfolio Commission](https://developer.chrome.com/api-reference/prime-api/rest-api/commission/get-portfolio-commission) endpoint. This parameter allows you to request commission rates for a specific product ID. The request looks like the following: Path Parameters

```
{
  "portfolio_id": "string"
}

```

Query Parameters

```
{
  "product_id": "string"
}

```

The response schema remains unchanged.

### 2025-JAN-18

Added new endpoint to update a Prime Onchain Wallet address group. This PUT endpoint replaces the existing address group with the new address group. The request requires portfolio ID and address group as shown below:

```
{
  portfolio_id: string;
  address_group: {
    id;
    name;
    network_type;
    addresses: [{
      name;
      address;
      networks: [string];
    }]
  }
}

```

The response consists of the following:

```
{
  "activity_type": "ACTIVITY_TYPE_ADDRESS_BOOK",
  "num_approvals_remaining": integer,
  "activity_id": string
}

```

### 2024-DEC-17

Added new endpoint to list your onchain address groups. This GET endpoint lists all address groups for a given portfolio ID. The response consists of the following:

```
{
  address_groups: [{
    id;
    name;
    network_type;
    added_at;
    addresses: [{
      name;
      address;
      chain_ids: [string]; // This will be empty for solana, * or list of chain IDs for EVM
    }]
  }]
}

```

### 2025-APR-7

-   Added the following field to [Products](https://developer.chrome.com/api-reference/prime-api/rest-api/products/list-portfolio-products) and Products Channel for Websocket feed:
    -   `rfq_product_detail` contains product details for placing RFQ orders, which includes the following sub-fields:
        -   `tradable` determines if the product is tradable for RFQ
        -   `min_notional_size` is the minimum notional size for RFQ orders
        -   `max_notional_size` is the maximum notional size for RFQ orders

### 2025-MAR-27

Created API references for the following Prime endpoints

-   [List Interest Accruals](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-interest-accruals)
-   [Get Entity Locate Availabilities](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-entity-locate-availabilities)
-   [Get Margin Information](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-margin-information)
-   [List Margin Call Summaries](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-margin-call-summaries)
-   [Get Trade Finance Tiered Pricing Fees](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-trade-finance-tiered-pricing-fees)
-   [List Portfolio Interest Accruals](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-interest-accruals)
-   [Get Portfolio Buying Power](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-buying-power)
-   [Get Portfolio Credit Information](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-credit-information)
-   [List Existing Locates](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-existing-locates)
-   [Create New Locates](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/create-new-locates)
-   [List Margin Conversions](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/list-margin-conversions)
-   [Get Portfolio Withdrawal Power](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-withdrawal-power)
-   [List Aggregate Entity Positions](https://developer.chrome.com/api-reference/prime-api/rest-api/positions/list-aggregate-entity-positions)
-   [List Entity Positions](https://developer.chrome.com/api-reference/prime-api/rest-api/positions/list-entity-positions)
-   [List Entity Balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/list-entity-balances)

### 2025-MAR-24

-   Added the following field to [List Portfolio Balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/list-portfolio-balances), [Get Wallet Balance](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/get-wallet-balance):
    -   `unbondable_amount` is the amount available for unbonding/unstaking, in whole units

### 2025-FEB-26

Added support for Request For Quote [Prime Rest API:](https://developer.chrome.com/api-reference/prime-api/rest-api/introduction)

-   [Create Quote](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order): Request: Creates a RFQ
-   [Accept Quote](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/accept-quote): Accepts a RFQ

[Prime FIX API:](https://developer.chrome.com/prime/fix-api/messages)

-   Server now supports a new message type sent by the client: [Quote Request <R>](https://developer.chrome.com/prime/fix-api/messages#quote-request-r)
-   Server now sends a new message type: [Quote <S>](https://developer.chrome.com/prime/fix-api/messages#quote-s)
-   Server now sends a new message type: [Quote Acknowledgment <b>](https://developer.chrome.com/prime/fix-api/messages#quote-acknowledgment-b)
-   Server now supports new values on [New Order Single <D>](https://developer.chrome.com/prime/fix-api/messages#accept-quote---new-order-single-d)

### 2025-FEB-12

-   Updated the following endpoints to support the new `network` field for Prime Multinetwork Support:
    -   [List Assets](https://developer.chrome.com/api-reference/prime-api/rest-api/assets/list-assets), will return `network_details` for each asset supported by the entity
    -   [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions), will return the `network` field in the response if the transaction is onchain
    -   [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions), will return the `network` field in the response if the transaction is onchain
    -   [Get Transaction by Transaction ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id), will return the `network` field in the response if the transaction is onchain
    -   [Create Withdrawal](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-withdrawal), will take in optional `network` field as a request parameter and return the `network` field in the response if the withdrawal is onchain
    -   [List Portfolio Wallets](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-portfolio-wallets), will return the `network` field in the response if the wallet is a vault wallet
    -   [Get Wallet by Wallet ID](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-by-wallet-id), will return the `network` field in the response if the wallet is a vault wallet
    -   [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet), will take in optional `network` field as a request parameter and return the `network` field in the response if the wallet is a vault wallet
    -   [Get Wallet Deposit Instructions](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-deposit-instructions), will take in the optional `network` field as a request parameter and return the `network` field in the response if the deposit instruction is onchain

### 2025-JAN-31

-   Added [Create Onchain Transaction](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-onchain-transaction), which allows you to create on-chain transactions for a given portfolio wallet.

### 2024-JAN-17

-   Added FCM futures API series including
    -   [Get Entity Positions](https://developer.chrome.com/api-reference/prime-api/rest-api/positions/list-entity-positions), retrieving given entity’s active positions with optional input `product_id`
    -   [Get Entity FCM Balance](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/get-entity-fcm-balance), retrieving given entity’s current FCM balance
    -   [List Entity Futures Sweeps](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/list-entity-futures-sweeps), retrieving given entity’s sweep requests up to 1000
    -   [Schedule Entity Futures Sweep](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/schedule-entity-futures-sweep), scheduling a sweep for given entity. Only one pending sweep is allowed at a time. `currency` is required to be provided.
    -   [Cancel Entity Futures Sweep](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/create-entity-futures-sweep), canceling the pending sweep request for a given entity. If there is no pending available, a 404 will be returned.
    -   [Set Auto Sweep](https://developer.chrome.com/api-reference/prime-api/rest-api/futures/set-auto-sweep), setting the `auto_sweep` status of a given entity. Auto sweep will automatically sweep all available balance by each settlement.

### 2024-DEC-17

-   Added `user_context` to [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders), [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders), [Get Order By ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id).
-   Added `Text` (58), `OrdType` (40), `Price` (44) to [Execution Report (8)](https://developer.chrome.com/prime/fix-api/messages#execution-report-8).
-   Added `user_context`, `limit_px`, `side`, `order_type` to the [Orders Channel](https://developer.chrome.com/prime/websocket-feed/channels#orders-channel).

### 2024-DEC-12

-   Updated `WEB3` wallet type to `ONCHAIN` on `type` field in request and response for [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet), [List Wallets](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-portfolio-wallets), and [Get Wallet by Wallet ID](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-by-wallet-id)
-   Added support for creating `ONCHAIN` wallets to [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet)
-   Added `VISIBILITY` field to [List Wallets](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/list-portfolio-wallets) and [Get Wallet by Wallet ID](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-by-wallet-id)
-   Updated `WEB3_TRANSACTION` transaction type to `ONCHAIN_TRANSACTION` in request parameters and responses for [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions), [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions), and [Get Transaction by Transaction ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id). All transactions previously marked as `WEB3_TRANSACTION` will now be returned as `ONCHAIN_TRANSACTION`.

### 2024-DEC-05

Added `product_id` field to [Orders Channel](https://developer.chrome.com/prime/websocket-feed/channels#orders-channel)’s messages.

### 2024-OCT-18

Added support for filtered [Level2 Data Channel](https://developer.chrome.com/prime/websocket-feed/channels#level2-data-channel) by venue configuration for Websocket feed:

-   Updated L2\_data channel request to include `venue_filtering` boolean.
-   Updated L2\_data channel response to include `venue_configuration` for each product.

### 2024-OCT-15

-   [List Portfolio Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-fills): Added new endpoint to retrieve fills across orders on a given portfolio.

### 2024-OCT-2

-   Added FIX tag [DropCopyFlag](https://developer.chrome.com/prime/fix-api/messages#logon-a) (9406) to [Logon (A)](https://developer.chrome.com/prime/fix-api/messages#logon-a).

### 2023-NOV-20

Added support for Stop Limit orders:

-   Updated [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) and [Get Order Preview](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-preview) with order type `STOP_LIMIT`, a conditional order combined of stop order and limit order. The `stop_price` field is required for this new order type.
-   Added FIX Target Strategy Type `SL` for the new Stop Limit order and `StopPx` (99) to [NewOrderSingle](https://developer.chrome.com/prime/fix-api/messages#new-order-single-d).

### 2023-OCT-31

-   Added FIX tag [ParticipationRate](https://developer.chrome.com/prime/fix-api/messages#new-order-single-d) (849) to [NewOrderSingle](https://developer.chrome.com/prime/fix-api/messages#new-order-single-d).

### 2023-OCT-26

-   Updated [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) and [Get Order Preview](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-preview) with new parameter called `historical_pov` that estimates participation rate for a TWAP/VWAP order. Can be specified instead of expiry time.

### 2023-SEP-11

-   Updated FIX tag [TargetStrategy](https://developer.chrome.com/prime/fix-api/messages#targetstrategy-values) (847) by adding **Immediate or Cancel** (`IOC`) support to NewOrderSingle `LIMIT` orders.

### 2023-SEP-08

Added endpoints to support Net Allocations:

-   [Create Net Allocation](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/create-portfolio-net-allocations): Added new endpoint to create a net allocation from a given portfolio.
-   [Get Allocations By Netting ID](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/get-net-allocations-by-netting-id): Added new endpoint to retrieve allocations by netting ID.

### 2023-AUG-28

-   Added `net_average_filled_price` to [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders), [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders), [Get Order By ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id).
-   Added `NetAvgPrice` (8006), `EffectiveTime` (168), `NoMiscFees` (136), `MiscFeeAmt` (137), `MiscFeeCurr` (138), `MiscFeeType` (139) to [Execution Report (8)](https://developer.chrome.com/prime/fix-api/messages#execution-report-8).
-   Added `net_avg_px` to the [Orders Channel](https://developer.chrome.com/prime/websocket-feed/channels#orders-channel).

### 2023-AUG-10

Updated [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders):

-   Fixed query parameters `order_type`, `order_side`, `sort_direction`, `start_date`, and `end_date` which can again be used to filter orders.
-   Deprecated query parameters `limit`, and `cursor`.
-   Deprecated field `pagination` from response payload.

### 2023-AUG-08

-   Updated the WebSocket feed [rate limiting specs](https://developer.chrome.com/prime/websocket-feed/overview#specs) with the latest service configuration.

### 2023-JUL-25

-   Updated FIX [TimeInForce](https://developer.chrome.com/prime/fix-api/messages#timeinforce-values) (59) with support for **Fill or Kill** (`FOK`).

### 2023-JUN-29

Added volume-weighted average price trading (`VWAP`):

-   [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order), [Get Order Preview](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-preview): Added a new `type` value called `VWAP`
-   [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders), [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders): Users can filter by new `order_type` called `VWAP`

### 2023-MAR-17

Added 2 new response properties to [Get Portfolio credit Information](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-credit-information):

-   `adjusted_credit_utilized`
-   `adjusted_portfolio_equity`

### 2023-MAR-09

-   Enabled [Replay](https://developer.chrome.com/prime/fix-api/connectivity#replay) for all FIX clients.

### 2023-FEB-10

-   Added FIX tag `IsRaiseExact` (8999) to [NewOrderSingle](https://developer.chrome.com/prime/fix-api/messages#new-order-single-d) messages for all order types (LIMIT, MARKET, TWAP).

### 2023-JAN-24

-   Added FIX tag `SenderSubID` (50) to [ExecReports](https://developer.chrome.com/prime/fix-api/messages#execution-report-8).

### 2022-NOV-09

-   Fixed WebSocket bug whereby clients subscribed to Orders Channels could receive updates from product IDs different from the ones provided in subscription message.

### 2022-OCT-19

Updated WebSocket:

-   Improved performance of server’s resource usage.
-   Added new time formatting definition fields, `timestamp` and `event_time`.

### 2022-SEP-02

Added 3 new response properties to [Get Portfolio Allocations](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/get-portfolio-allocations):

-   `allocation_completed_at`
-   `order_ids`
-   `fees_allocated_leg` of destination level

### 2022-AUG-12

Added new endpoint [List Portfolio Allocations](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/get-portfolio-allocations) returns a list of allocations.

### 2022-AUG-10

-   Updated [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders), [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders):
    -   Users can now filter by a new `order_type` called: `BLOCK`
    -   Added a new possible value `BLOCK` to the field `type`, which refers to a block trade
-   Updated [Get Order by Order ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id):
    -   Added a new possible value `BLOCK` to the field `type`, which refers to a block trade

### 2022-JUL-27

-   Updated [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders), [Get Order by Order ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id), and [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders):
    -   Users can now see a field `exchange_fee`, populated when the order was placed with a `COST_PLUS` commission configuration.
-   Added the following fields to [List Portfolio Balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/list-portfolio-balances), [Get Wallet Balance](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/get-wallet-balance):
    -   `bonded_amount` is the amount currently locked due to bonding/staking, potentially subject to an unbonding period, in whole units
    -   `reserved_amount` is the amount that must remain in the wallet due to the protocol, in whole units
    -   `unbonding_amount` is the amount that is in the process of unbonding, in whole units
    -   `unvested_amount` is the unrealized amount subject to a vesting schedule, in whole units
    -   `pending_rewards_amount` is the pending bonding/staking rewards that have not yet been realized, in whole units
    -   `past_rewards_amount` is the previously realized bonding/staking rewards, in whole units
    -   `bondable_amount` is the amount available for bonding/staking, in whole units

### 2022-JUL-01

Added support for conversions:

-   [Create Conversion](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-conversion) is now available to perform conversions between USD and USDC.
-   [List Activities](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/list-activities) can now be filtered by a new type of activity called: `ACTIVITY_TYPE_CONVERSION`.
-   Added the conversion `destination_symbol` field to [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions), [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions), and [Get Transaction by Transaction ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id)
-   Added `CONVERSION` filter to [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) and [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions),

### 2022-JUN-08

-   Added the following endpoints
    -   [Get Address Book](https://developer.chrome.com/api-reference/prime-api/rest-api/address-book/get-address-book): `GET /v1/portfolios/{portfolio_id}/address_book`
    -   [Create Address Book Entry](https://developer.chrome.com/api-reference/prime-api/rest-api/address-book/create-address-book-entry): `POST /v1/portfolios/{portfolio_id}/address_book`
-   Updated [List Activities](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/list-activities) with new type of activity, `ACTIVITY_CATEGORY_ALLOCATION`

### 2022-MAY-19

-   Added FIX tag `MaxShow` (201) to [NewOrderSingle](https://developer.chrome.com/prime/fix-api/messages#new-order-single-d) for LIMIT orders.

### 2022-APR-22

-   Updated [List Portfolio Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders):
    -   The `product_ids` parameter is no longer required. If not specified you get all products.
    -   The 31 day limit has been removed so you can now request orders for any time period using `start_date` and `end_date`. The `end_date` parameter is still optional, but if not specified you get orders from `start_date` to the end of time. Previously, it returned `start_date` to `start_date` + 31 days.
-   Updated [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders):
    -   The `product_ids` parameter is no longer required. If not specified you get all products.
    -   All other query params are currently non-functional and result in an error if used. This includes: `order_type`, `cursor`, `limit`, `sort_direction`, `start_date`, `order_side`, and `end_date`.
    -   The maximum number of orders returned is 1000. If a client has more than 1000 open orders, an error is returned prompting the user to use WebSocket API, or FIX API to stream open orders.

### 2022-APR-07

-   Added the `has_next` boolean field to the `pagination` field in the response body. The `has_next` field indicates if there is more data available to paginate through.
-   Removed the `total_result_count` field has been removed from the `pagination` field in the response body.

### 2022-MAR-24

Added:

-   [Create Withdrawal](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-withdrawal) is now available to request new withdrawal transactions from a portfolio wallet.
-   [Create Transfer](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-transfer) is now available to request new transfers between portfolio wallets.

### 2022-MAR-19

Added:

-   [Get Entity Payment Method](https://developer.chrome.com/api-reference/prime-api/rest-api/payment-methods/list-entity-payment-methods) is now available to retrieve payment method information related to investment vehicles.
-   [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet) is now available to request new wallets for a portfolio.

### 2022-MAR-07

Added:

-   [List Activities](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/list-activities) is now available to retrieve a list of activities related to a given portfolio.
-   [Get Activity by Activity ID](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/get-portfolio-activity-by-activity-id) is now available to retrieve a specific activity.

### 2022-FEB-15

Fixed:

-   Blockchain Network Ids (`blockchain_ids`) now appear in the Transaction response.
-   Pagination was fixed for [List Open Orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-open-orders) and [List Order Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-order-fills)

### 2022-JAN-31

Added:

-   [Get Portfolio Credit Information](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-portfolio-credit-information) returns 200 response if post trade credit is not enabled.
-   [List Portfolio Balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/list-portfolio-balances) has a new `type` filter.
-   [Get Portfolio Commission](https://developer.chrome.com/api-reference/prime-api/rest-api/commission/get-portfolio-commission) is now available to retrieve commissions for your portfolio.

### 2022-JAN-19

Fixed:

-   The `account_identifier` field now appears within [Get Wallet Deposit Instructions](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/get-wallet-deposit-instructions).
-   Missing Transaction resources now appear in [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions).

Additions:

-   The fields `transfer_from` and `transfer_to` now appear in the Transaction resource for withdrawals and deposits.
-   External address deposit information is now included in the Transaction resource response.

### 2022-JAN-15

Additions:

-   For deposits and withdrawals to bank accounts, the payment method associated with that bank account now appears in the Transaction response as the `transfer_from` and `transfer_to` type.
-   Added non-balance impacting transaction types to [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions).

### 2021-DEC-15

-   Welcome to the Coinbase Prime REST API.