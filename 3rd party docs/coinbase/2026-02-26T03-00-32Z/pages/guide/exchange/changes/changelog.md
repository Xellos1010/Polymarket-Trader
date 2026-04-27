# changelog

These release notes list changes to Coinbase Exchange.

## 2026-Feb-02

**Address Book Support for All EVM Networks** We have expanded the Address Book to support EVM-wide address entries for all assets. This allows customers to store addresses once across all EVM networks. To support this, we have updated the currencies and address book endpoints. A new value of `_ALL_EVM_NETWORKS_` has been added to the `network` field in the address book endpoints. This value is only valid when the currency is `_ALL_ASSETS_`. Example request `POST https://api.exchange.coinbase.com/address-book`.

```
{
  "address": "0x1234567890123456789012345678901234567890",
  "currency": "_ALL_ASSETS_",
  "network": "_ALL_EVM_NETWORKS_"
}

```

For more details, refer to the [Address Book](https://developer.chrome.com/api-reference/exchange-api/rest-api/address-book/add-addresses) REST endpoint. In addition, we have added a new field under the `supported_networks` object in the currencies endpoints to indicate if the network is an EVM-compatible blockchain.

For more details, refer to the [Currencies](https://developer.chrome.com/api-reference/exchange-api/rest-api/currencies/get-all-known-currencies) REST endpoint.

## 2026-JAN-15

**Counterparty Address API Access** Clients can now programmatically retrieve stored counterparty addresses from their Address Book. This feature enables direct access to saved address data via the API. For implementation details, refer to the [Get Counterparty Address Book](https://developer.chrome.com/api-reference/exchange-api/rest-api/address-book/get-counterparty-address-book) REST endpoint.

## 2025-DEC-11

**MiCA-specific Coinbase Exchange URLs** Coinbase Exchange customers subject to the EU Markets in Crypto-Assets (MiCA) regulation must transition to MiCA-specific Coinbase Exchange URLs. Further details regarding Coinbase’s MiCA compliance can be found here: [https://help.coinbase.com/en/prime/compliance/MiCA-migration](https://help.coinbase.com/en/prime/compliance/MiCA-migration) Coinbase Exchange customers subject to MiCA regulation must use the following API endpoints: Production - MiCA API Endpoints:

-   REST
    -   [https://api-us.dma.prime.coinbase.com](https://api-us.dma.prime.coinbase.com/)
-   FIX - Order Entry
    -   tcp+ssl://fix-us.dma.prime.coinbase.com:7110
-   FIX - Drop Copy
    -   tcp+ssl://fix-dc.dma.prime.coinbase.com:7122
-   FIX - Market Data Snapshot Enabled
    -   tcp+ssl://fix-us.dma.prime.coinbase.com:7120
-   FIX - Market Data Snapshot Disabled
    -   tcp+ssl://fix-us.dma.prime.coinbase.com:7121
-   Websocket Feed
    -   wss://ws-us.dma.prime.coinbase.com
-   Websocket Direct Feed
    -   wss://ws-us-direct.dma.prime.coinbase.com

Production UI: [https://dma.prime.coinbase.com/](https://dma.prime.coinbase.com/) **Sandbox - MiCA API Endpoints**:

-   REST
    -   [https://api-us.dma.sandbox.prime.coinbase.com](https://api-us.dma.sandbox.prime.coinbase.com/)
-   FIX - Order Entry
    -   tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7110
-   FIX - Drop Copy
    -   tcp+ssl://fix-dc.dma.sandbox.prime.coinbase.com:7122
-   FIX - Market Data Snapshot Enabled
    -   tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7120
-   FIX - Market Data Snapshot Disabled
    -   tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7121
-   Websocket Feed
    -   wss://ws-us.dma.sandbox.prime.coinbase.com
-   Websocket Direct Feed
    -   wss://ws-us-direct.dma.sandbox.prime.coinbase.com

Sandbox UI: [https://dma-sandbox.prime.coinbase.com/](https://dma-sandbox.prime.coinbase.com/)

## 2025-DEC-4

**Counterparty Transfers API Access** Coinbase has released Counterparty Transfers [API](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-counterparty) access for all customers. This will allow customers to transfer crypto to and from other Coinbase customers without incurring on chain fees. For more details, navigate to the [Withdraw to counterparty Id](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-counterparty) REST endpoint.

## 2025-OCT-31

Coinbase implemented the following changes to our FIX DC channel on October 30, 2025: **FIX Drop Copy Session**:

-   Execution Reports (ER) will be delivered for `cancelled` and `Stop-Limit activation` orders.
-   Note: Execution Reports for `new` or `changed` orders was implemented on October 1, 2025

Coinbase implemented the following changes to our FIX MD channel on October 9, 2025: **FIX Market Data**

-   A `TradeID` tag has been added to completed order messages, i.e messages with MDEntryType=Trade (269=2)
-   `<field number='1003' name='TradeID' type='STRING'/>`

## 2025-OCT-20

Coinbase implemented the following changes to our TLS requirements October 20, 2025: Effective Monday, Oct. 20th 2025, Coinbase Exchange implemented a change to our exchange.coinbase.com domain to only allow system access requests that utilize TLS (Transport Layer Security) 1.2 or higher. This means that any client applications, trading bots, or other automated systems that currently rely on older versions of TLS (such as TLS 1.0 or TLS 1.1) will no longer be able to establish a secure connection with the Coinbase Exchange domain.

## 2025-OCT-1

Coinbase is implementing the following changes to our FIX channels on October 2, 2025 **FIX Drop Copy Session**

-   Execution Reports (ER) will be delivered for ‘new’ or ‘changed’orders. Note: ER’s for cancelled orders coming soon

**FIX OE Session**

-   Execution Reports (ER) for orders placed in previous sessions will now be delivered to currently connected sessions, if applicable.

## 2025-SEP-3

Enabling RFQs in terms of CashOrderQty (152)

-   Adding CashOrderQty (152) to Quote\_Request (R) messages
-   Adding BidCashOrderQty (8234) and OfferCashOrderQty (8235) tags to Quote (S) messages
-   Adding LeavesFunds (8152) and CashOrderQty (152) to Execution Reports (8)

We will allow RFQ takers to submit orders using a quantity specified in terms of the quote\_currency. The quote\_currency refers to the USD quantity or latter currency in the product pair (e.g. for BTC-USD, quote\_currency = USD. For BTC-SOL, quote\_currency = SOL).  
As a result, LPs must respond in terms of quote\_currency, as indicated by FIX tag CashOrderQty (152) on the Quote\_Request (R) message.

-   LPs must respond with the full BidCashOrderQty (8234) | BidPx (132) and OfferCashOrderQty (8235) | OfferPx (133) on their corresponding Quote (S) message.
-   Execution Reports (8) for corresponding orders will reflect the CashOrderQty (152) and the OrderQty (38), which is calculated as such:
    -   OrderQty (38) = (CashOrderQty / (BidPx or OfferPx))
-   Execution Reports (8) will also contain LeavesFunds (8152) field to indicate remaining CashOrderQty (152) that is unfilled.

Sample Execution Report:

```
8=FIXT.1.1|9=440|35=8|49=Coinbase|56=TARGET_COMP_ID|34=15|50=TEST|52=20250408-03:34:32.789012|369=8|6=70000|
11=CLIENT_ORDER_ID|14=0.01428|17=EXEC_ID|37=ORDER_ID|39=2|55=BTC-USD|54=2|40=2|32=0.01428|31=70000|44=70000|38=0.01428|
60=20250408-03:34:32.784254|152=999.60015994|150=F|8152=0|59=4|126=20250408-03:34:39.467|136=1|137=0.249900039985|
138=USD|139=4|891=0|10=204

```

## 2025-AUG-12

We’ve added a new `network` field to the POST, GET and PUT /address-book endpoints. You will be able to designate a network when creating or modifying whitelisted addresses in the address book. The response body will include the `network` field indicating the network of the address.

## 2025-JUN-3

The FIX 4.2 Order Entry Gateway has been deprecated on **June 3rd, 2025**. For FIX based order entry, **leverage the newer, more performant** [FIX 5 Order Entry Gateway](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5).

### 2025-MAY-27

We removed support for the Iceberg Order Type. This applies to both FIX4.2 and FIX5. We will continue to monitor client needs and hope to reintroduce an enhanced version of this order type with broader support in the future.

### 2025-MAY-13

Coinbase Exchange is introducing a dedicated FIX 5.0 Drop Copy (DC) fleet, accessible via a new URI and port. Drop copy session connected over this fleet will deliver complete execution report messages with full field parity to FIX 4.2.

-   Available via a dedicated connection:
    -   URI:
        -   **Prod** tcp+ssl://fix-dc.exchange.coinbase.com
        -   **Sandbox** tcp+ssl://fix-dc.sandbox.exchange.coinbase.com
    -   Port: 6122
-   New tags on Execution Reports on DC:
    -   ClOrdID `<11>`: (String), An identifier specified by the sender to uniquely identify other messages correlating to this request.
    -   OrdStats `<39>`: (Char), Identifies current status of order.
    -   OrdQty `<38>`: (Qty), Quantity ordered.
    -   OrdType `<40>`: (Char), Order type.
    -   LeavesQty `<151>`: (Qty), Quantity open for further execution.
    -   CashOrderQty `<152>`: (Qty), Specifies the approximate order quantity desired in total monetary units vs. as a number of shares.

**Note**: Existing Order Entry drop copy functionality (via tag 9406=Y) will not change.

### 2025-APR-22

Added functionality for users to redeem CBETH to ETH via Web & API:

-   [Create a new redeem](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/create-new-redeem) `POST https://api.exchange.coinbase.com/wrapped-assets/redeem`
-   [List all redeems](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-all-redeems) `GET https://api.exchange.coinbase.com/wrapped-assets/redeem`
-   [Get a single redeem](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-single-redeem) `GET https://api.exchange.coinbase.com/wrapped-assets/redeem/{redeem_id}`

### 2025-JAN-28

Rolled out a restriction requiring the ClOrdID of any new order or modification request to not match the ClOrdID of any open orders. This is applicable to both FIX4.2 and FIX5.

### 2025-JAN-01

We have deprecated the Coinbase Price Oracle API.

### 2024-DEC-2

A new [Get All Conversions](https://developer.chrome.com/api-reference/exchange-api/rest-api/conversions/get-all-conversions) endpoint has been added to the REST API. This endpoint returns all conversions associated with the profile tied to the API key used to make the request. `GET https://api.exchange.coinbase.com/conversions`

### 2024-OCT-16

FIX Market Data Snapshot Enabled Gateway deprecation has been postponed until further notice.

### 2024-SEP-19

`trade_id` has been added to the [RFQ Matches Channel](https://developer.chrome.com/exchange/websocket-feed/channels#rfq-matches-channel).

### 2024-SEP-11

The Pro API ([https://api.pro.coinbase.com](https://api.pro.coinbase.com/)) has been deprecated. For REST API access, leverage the [Exchange API](https://developer.chrome.com/api-reference/exchange-api/rest-api/introduction)

### 2024-AUG-21

We released Snapshot Disabled [FIX Market Data Gateway](https://developer.chrome.com/exchange/fix-api/market-data) to production. It is based on the [FIX 5.0 SP2 specification](https://www.onixs.biz/fix-dictionary/5.0.sp2/index.html) and will only provide incremental messages.

### 2024-AUG-01

-   Modified permission requirement for [Update settlement preference](https://developer.chrome.com/api-reference/exchange-api/rest-api/users/update-settlement-preference) to `MANAGE`
-   Updated WebSocket subscription and rate limits. Users can now purchase additional subscriptions to facilitate their needs. See [WebSocket Rate Limits](https://developer.chrome.com/exchange/websocket-feed/rate-limits).

### 2024-JUN-27

Added support for [Limit With Funds orders](https://developer.chrome.com/exchange/fix-api/order-entry-messages/limit-orders) in FIX 5.0, FIX 4.2, and REST.

### 2024-APR-22

Added the following rule for the [Get a single account’s ledger](https://developer.chrome.com/api-reference/exchange-api/rest-api/accounts/get-single-account-ledger) API—if neither `start_date` nor `end_date` is set, the endpoint returns ledger activity for the past 1 day only.

### 2024-APR-02

-   Added new endpoint, [Get user trading volumes](https://developer.chrome.com/api-reference/exchange-api/rest-api/users/get-user-trading-volume) to list a single user’s aggregated and individual trading volumes.

`GET https://api.exchange.coinbase.com/users/{user_id}/trading-volumes`

### 2024-MAR-21

Added support for [Take Profit Stop Loss orders](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders) in FIX 5.0 and FIX 4.2.

### 2024-MAR-19

Added new Loan Interest endpoints:

-   [Get all interest summaries](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-interest-summaries)
-   [Get a single loan’s interest rate history](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-interest-rate-history)
-   [Get a single loan’s interest charges](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-interest-charges)

### 2024-MAR-18

Removed all properties, except `exchange_withdraw`, from [Get user exchange limits](https://developer.chrome.com/api-reference/exchange-api/rest-api/users/get-user-exchange-limits):

```
buy
sell
ach
ach_no_balance
credit_debit_card
secure3d_buy
paypal_buy
paypal_withdrawal
ideal_deposit
sofort_deposit
instant_ach_withdrawal

```

### 2024-MAR-06

Added the following response properties to [Get product stats](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-product-stats):

-   `rfq_volume_24hour`
-   `conversions_volume_24hour`
-   `rfq_volume_30day`
-   `conversions_volume_30day`

```
// Example
"apiProductStats": {
  "type": "object",
  "example": {
    "open": "5414.18000000",
    "high": "6441.37000000",
    "low": "5261.69000000",
    "volume": "53687.76764233",
    "last": "6250.02000000",
    "volume_30day": "786763.72930864",
    "rfq_volume_24hour": "78.23",
    "conversions_volume_24hour": "0.000000",
    "rfq_volume_30day": "0.000000",
    "conversions_volume_30day": "0.000000"
  }
}

```

### 2024-FEB-27

-   Added FIX 5.0 support for Iceberg Orders (in addition to FIX 4.2).
-   Moved documentation for Iceberg Orders to a dedicated page.

### 2024-FEB-23

Added support for OrderMassCancelRequest (by **Trading Session** only):

-   [OrderMassCancelRequest (35=q)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordermasscancelrequest-35q): Sent by customer to Coinbase to request mass cancellation of all orders on a FIX session previously submitted by customer.
-   [OrderMassCancelReport (35=r)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordermasscancelreport-35r): Sent by Coinbase to the customer as an acknowledgement of an Order Mass Cancel Request for processing or a rejection of the request.

### 2024-FEB-07

-   Added `fee_amount` to existing endpoint, [/conversions](https://developer.chrome.com/api-reference/exchange-api/rest-api/conversions/convert-currency) response.
-   Added new endpoint, [/conversions/fees/](https://developer.chrome.com/api-reference/exchange-api/rest-api/conversions/get-conversion-fee-rates) to help monitor current L30d net conversion volume and calculate expected fees.

### 2024-FEB-05

Added the `display_name` to:

-   The top level of the [Account](https://developer.chrome.com/api-reference/exchange-api/rest-api/accounts/get-single-account-by-id) and [Currency](https://developer.chrome.com/api-reference/exchange-api/rest-api/conversions/get-a-conversion) response objects (but did not remove from the lower level `details` object as originally planned).
-   The WebSocket [Status Channel](https://developer.chrome.com/exchange/websocket-feed/channels#status-channel) `currencies` object.

```
{
    "id": "BTC",
    "name": "Bitcoin",
    "details":
    {
        "type": "crypto",
        "symbol": null,
        ...
        "display_name": null
    },
    "default_network": "bitcoin",
    "supported_networks":
        [{...}],
    "display_name": "BTC"  // UI only
}

```

### 2024-JAN-24

Added `account_ids` to the WebSocket [subscriptions](https://developer.chrome.com/exchange/websocket-feed/overview#subscriptions-message) message, in support of the [Balance Channel](https://developer.chrome.com/exchange/websocket-feed/channels#balance-channel). Users not subscribing to this channel can ignore this field.

```
{
   "type": "subscriptions",
   "channels": [
      {
         "name": "balance",
         "product_ids": null,
         "account_ids": [
            "826863ab-ce92-47a0-86f6-30b4776190c1"
         ]
      }
   ]
}

```

### 2024-JAN-16

Added a new WebSocket [Balance Channel](https://developer.chrome.com/exchange/websocket-feed/channels#balance-channel) to track account balance updates. It is recommended for accounts with frequent balance changes. Authentication is required.

```
// Balance Channel subscribe message
{
  "type": "subscribe",
  "channels": [
    {
      "name": "balance",
      "account_ids": [
        "d50ec984-77a8-460a-b958-66f114b0de9b",
        "d50ec984-77a8-460a-b958-66f114b0de9a"
      ]
    }
  ]
}

```

#### FIX 4.2 Execution Report

Updated the FIX 4.2 Execution Report as follows:

-   `LeavesQty` and `CumQty` are now additionally populated in the Execution Report when `OrdStatus` is New or Replaced. In sum, they populate it when:
    -   `39=0` ⇒ OrdStatus = New
    -   `39=1` ⇒ OrdStatus = Partially filled
    -   `39=3` ⇒ OrdStatus = Done for day
    -   `39=5` ⇒ OrdStatus = Replaced
-   `AvgPx` only populates Execution Report if CumQty > 0.
-   `OrderQty` now represents the original order quantity when `OrdStatus` is Canceled or Done for day.

### 2024-JAN-09

Added [Travel Rule](https://developer.chrome.com/exchange/travel-rule/withdrawals) support:

-   Added support for providing Travel Rule data when withdrawing to a crypto address.
-   Added support for Coinbase to act as intermediary VASP for crypto withdrawals.
-   [Withdraw to crypto address](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-crypto-address) accepts new optional parameters as part of its `POST` request:
    -   `travel_rule_data`: Data necessary to satisfy Travel Rule data requirements.
    -   `is_intermediary`: Flag to create transfer with Coinbase as intermediary VASP.
        -   If `true`, `intermediary_jurisdiction` must be provided.
        -   Parameter `travel_rule_data` may be necessary if the jurisdiction requires data.
    -   `intermediary_jurisdiction`: Jurisdiction (ISO 3166-1 alpha-2) for Travel Rule data validation.

### 2023-DEC-22

Added [RFQ](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#rfq) support to FIX 5.0 Order Entry.

### 2023-DEC-20

Released [ResendRequest (35=2)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#resendrequest-352) and [SequenceReset-GapFill](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#sequencereset-354) to production for [FIX 5.0 Order Entry](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5).

### 2023-NOV-27

Added Iceberg Orders to FIX 5.0 production:

-   [NewOrderSingle (35=D)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#newordersingle-35d)
-   [ExecutionReport (35=8)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#executionreport-358)

See Iceberg Orders in 4.2 for conceptual details.

### 2023-NOV-21

Added **[Loan APIs](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-loans)** for underwritten users to view and monitor loan/collateral metrics, initiate loan opens and initiate loan repayments.

-   [List loans](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-loans) `GET https://api.exchange.coinbase.com/loans`
-   [List loan assets](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-loan-assets) `GET https://api.exchange.coinbase.com/loans/assets`
-   [Get lending overview](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/get-lending-overview) `GET https://api.exchange.coinbase.com/loans/lending-overview`
-   [Get new loan preview](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/get-new-loan-preview) `GET https://api.exchange.coinbase.com/loans/loan-preview`
-   [Open new loan](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/open-new-loan) `POST https://api.exchange.coinbase.com/loans/open`
-   [List new loan options](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/list-new-loan-options) `GET https://api.exchange.coinbase.com/loans/options`
-   [Repay loan principal](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/repay-loan-principal) `POST https://api.exchange.coinbase.com/loans/repay-principal`
-   [Get principal repayment preview](https://developer.chrome.com/api-reference/exchange-api/rest-api/loan/get-principal-repayment-preview) `GET https://api.exchange.coinbase.com/loans/repayment-preview`

### 2023-NOV-02

Added Iceberg Orders to FIX 4.2 production and the REST API.

-   FIX 4.2 New Order Single (D) and Execution Report (8)
-   REST API [Create a new order](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/create-new-order).

### 2023-OCT-30

In FIX 4.2, disabled `DropCopyFlag` by default by setting `9406=N` (from Y). See Logon (A). To ensure a session continues to receive Drop Copy reports, explicitly enable `DropCopyFlag` by setting `9406=Y`. See [Drop Copy Session](https://developer.chrome.com/exchange/fix-api/best-practices#drop-copy-session) in *Best Practices* for more.

### 2023-OCT-24

Added `destination_tag_regex` field to the `supported_networks` of the `currencies` object in [Status message](https://developer.chrome.com/exchange/websocket-feed/channels#status-channel)

### 2023-OCT-02

[FIX Order Entry on the FIX 5.0 SP2 protocol](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5) was released to production.

### 2023-SEP-19

Added Iceberg orders to the FIX 4.2 Sandbox. See [Upcoming Changes](https://developer.chrome.com/exchange/changes/upcoming-changes) for details.

### 2023-SEP-07

Added the following RFQ feature to production:

-   RFQ Request (AH) enables users to submit subscription requests based on specific assets, ensuring that only these particular requests are forwarded. For example, if subscribing to “ETH-USD” with both products present, a quote request message and quote status report for both “ETH-USD” and “USD-ETH” are returned as responses.

### 2023-SEP-01

Added Travel Rule APIs for users to provide and manage their travel rule deposit information. Additionally, for users in certain jurisdictions, either `vasp_id` or `is_verified_self_hosted_wallet` must be provided when creating address book entries.

-   [Get all travel rule information](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/get-all-travel-rule) `GET https://api.exchange.coinbase.com/travel-rules`
-   [Create travel rule entry](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/create-travel-rule) `POST https://api.exchange.coinbase.com/travel-rules`
-   [Delete existing travel rule entry](https://developer.chrome.com/api-reference/exchange-api/rest-api/travel-rules/delete-existing-travel-rule) `DELETE https://api.exchange.coinbase.com/travel-rules/{travel_rule_id}`
-   [Submit travel information for a transfer](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/submit-travel-info) `POST https://api.exchange.coinbase.com/transfers/{transfer_id}/travel-rules`
-   [Add addresses](https://developer.chrome.com/api-reference/exchange-api/rest-api/address-book/add-addresses) `POST https://api.exchange.coinbase.com/address-book`

### 2023-AUG-28

Exchange began enforcing the following validation logic:

-   **REST & FIX**: Extra leading zeros on quantities and prices are not accepted. They must be in standard form.
-   **FIX only**: When sending market orders, only `CashOrderQty` or `OrderQty` can be included on the order, not both.
-   **REST only**: When sending market orders, only `size` or `funds` can be included on the order, not both.

### 2023-AUG-14

Added the following RFQ features to production:

#### FIX

-   Quote (S): The response was reduced to 250ms (from 1000ms).
-   MiscFeeAmt (137): Fees are returned on MiscFeeAmt (137) per product.
    -   Fees published on tag `137` match what is available on the REST endpoint.
    -   Fees may be published at a rate lower than 5bps on a per-pair basis.

#### REST

-   `/rfq/products` (unauthenticated) returns all currently supported and eligible RFQ product pairs and their associated fee rates (`maker_fee_bps`), plus additional metadata.

```
GET https://api-public.exchange.coinbase.com/rfq/products

```

```
## Sample Response
{
   "id": "AAVE-BCH",
   "base_currency": "AAVE",
   "quote_currency": "BCH",
   "enabled": true,
   "maker_fee_bps": "5",
   "rfq_tier": 2
}

```

See [Upcoming Changes](https://developer.chrome.com/exchange/changes/upcoming-changes#rfq-updates) for RFQ features available in the sandbox.

### 2023-AUG-03

Added the ability to [download FIX specifications](https://developer.chrome.com/exchange/fix-api/dictionary-downloads).

### 2023-AUG-01

The following WebSocket feeds now enforce authentication (in addition to the [User channel](https://developer.chrome.com/exchange/websocket-feed/channels#user-channel)):

-   [Full channel](https://developer.chrome.com/exchange/websocket-feed/channels#full-channel)
-   [Level2 channel](https://developer.chrome.com/exchange/websocket-feed/channels#level2-channel)
-   [Level3 channel](https://developer.chrome.com/exchange/websocket-feed/channels#level3-channel)

See [WebSocket Authentication](https://developer.chrome.com/exchange/websocket-feed/authentication) for instructions.

### 2023-JUL-21

Added the Exchange FIX 5.0 SP2 specification for Order Entry (sandbox only):

-   Exchange FIX 5.0 SP2 [Order Entry (Sandbox Only)](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5)

### 2023-JUN-14

#### API Manage Permission Type

Introduced new `MANAGE` API key permission type to manage user settings and preferences.

#### Address Book API

Added functionality for users to manage their Address Books via API:

-   [Add new crypto address to address book](https://developer.chrome.com/api-reference/exchange-api/rest-api/address-book/add-addresses) `POST https://api.exchange.coinbase.com/address-book`
-   [Delete an address from address book](https://developer.chrome.com/api-reference/exchange-api/rest-api/address-book/delete-address) `DELETE https://api.exchange.coinbase.com/address-book/{address_book_id}`

### 2023-JUN-13

Added documentation on [Systems & Operations](https://developer.chrome.com/exchange/introduction/systems-operations)

### 2023-JUN-06

Updated the Exchange FIX 4.2 specification

### 2023-MAY-01

FIX Resend Requests (or FIX “Replay”) is live in production. Resend requests are sent by the receiving application to initiate the retransmission of messages.

Tag

Name

Description

7

[BeginSeqNo](https://www.onixs.biz/fix-dictionary/4.2/tagnum_7.html)

Sequence number of first message in range to be resent

16

[EndSeqNo](https://www.onixs.biz/fix-dictionary/4.2/tagnum_16.html)

Sequence number of last message in range to be resent

### 2023-APR-24

We added USD ⟨⟩ USDC unification which lets Coinbase Exchange users update their settlement preference to be either the stable coin counterpart for a fiat currency, or the fiat currency itself.

-   [Update settlement preference](https://developer.chrome.com/api-reference/exchange-api/rest-api/users/update-settlement-preference) `POST https://api.exchange.coinbase.com/users/{user_id}/settlement-preferences`

### 2023-APR-19

Added functionality for users to stake-wrap ETH to CBETH via Web & API:

-   [Create a new stake-wrap](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/create-new-stake-wrap) `POST https://api.exchange.coinbase.com/wrapped-assets/stake-wrap`
-   [Get all stake-wraps](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-all-stake-wraps) `GET https://api.exchange.coinbase.com/wrapped-assets/stake-wrap`
-   [Get a single stake-wrap](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-all-wrapped-assets) `GET https://api.exchange.coinbase.com/wrapped-assets/stake-wrap/{stake_wrap_id}`

Learn more at [Coinbase Exchange CBETH Stake Wrap Instructions](https://coinbase.bynder.com/m/5d3cff1b03f8aeb0/original/exchange-CBETH-Stake-Wrap-Instructions.pdf).

### 2023-MAR-29

We added a new [Level3 WebSocket channel](https://developer.chrome.com/exchange/websocket-feed/channels#level3-channel) which is a compact, low bandwidth, and easier to parse version of the Full channel.

### 2023-MAR-15

Added new `apy` field to [Get Wrapped Asset Details](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-wrapped-asset-details). Implied APY is the current annualized percentage yield earned as the net rewards by the staked ETH underlying cbETH. This estimate is based on the past 7 days of staking performance and is updated daily. For more details, refer to the “rate calculation” section of the [cbETH whitepaper](https://www.coinbase.com/cbeth/whitepaper).

### 2023-MAR-02

We added a `time` property that denotes the last time our system processed an update. **REST API** [Get product book](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-product-book) now returns `time` as a property when passing in “level=L1” / “level=L2” / “level=L3” on the `/products/{product_id}/book/` endpoint. **Websocket API** [Level2 Channel](https://developer.chrome.com/exchange/websocket-feed/channels#level2-channel) now includes the `time` property on the level2 snapshot message.

### 2023-MAR-01

We updated the Market Data feed: `client-oid` was removed from the **unauthenticated** [Full Channel Received message](https://developer.chrome.com/exchange/websocket-feed/channels#received). The `client-oid` field will still be available in the authenticated Full Channel, and also the [User Channel](https://developer.chrome.com/exchange/websocket-feed/channels#user-channel) (which also requires authentication). You can only see your own `client-oid`.

### 2023-FEB-23

#### FIX Execution Report ExecID

We released a change to how we publish the FIX tag, `ExecID (Tag=17)`. `ExecID` will now be consistent across sessions for the same underlying Execution Report. This should allow users the **ability to join Execution Reports** between their trading session and their drop-copy session for the same profile ID.

#### FIX Market Data Gateway

We released [FIX Market Data gateway](https://developer.chrome.com/exchange/fix-api/market-data) (**beta**) to production. It is based on the [FIX 5.0 SP2 specification](https://www.onixs.biz/fix-dictionary/5.0.sp2/index.html).

This new offering provides an L3 feed only with direct, low-latency, deterministic access. Users must connect with the same authentication as our existing FIX order-entry system.

-   [Header](https://developer.chrome.com/exchange/fix-api/market-data#header)
-   [Logon (35=A)](https://developer.chrome.com/exchange/fix-api/market-data#logon-35a)
-   [Market Data Request (35=V)](https://developer.chrome.com/exchange/fix-api/market-data#market-data-request-35v)
-   [Market Data Request Reject (35=Y)](https://developer.chrome.com/exchange/fix-api/market-data#market-data-request-reject-35y)
-   [Security Status (35=f)](https://developer.chrome.com/exchange/fix-api/market-data#security-status-35f)
-   [Market Data Incremental Refresh (35=X)](https://developer.chrome.com/exchange/fix-api/market-data#market-data-incremental-refresh-35x)
-   [Market Data Snapshot Full Refresh (35=W)](https://developer.chrome.com/exchange/fix-api/market-data#market-data-snapshot-full-refresh-35w)
-   [Security List Request (35=x)](https://developer.chrome.com/exchange/fix-api/market-data#security-list-request-35x)
-   [Security List (35=y)](https://developer.chrome.com/exchange/fix-api/market-data#security-list-35y)

### 2023-JAN-30

#### Expire Time Tag

Added FIX message tag `ExpireTime` to Quote Request (R) and Quote Status Report (AI).

#### High Bid Limit Percentage

Added new `high_bid_limit_percentage` field to our products endpoint. As described in our [Trading Rules](https://www.coinbase.com/legal/trading_rules), the **High Bid Limit** (HBL) order control limits how high a Buy Limit Order can be filled based on the last sale price or the current best bid price. Currently, the High Bid Limit is calculated as follows:

-   **Last execution price + `high_bid_limit_percentage`** if the current best bid price is < 95% of the last execution.
-   **Current best bid price + `high_bid_limit_percentage`** otherwise.

HBL is only enforced for specific trading pairs. You can see which trading pairs have an HBL set by looking at the `high_bid_limit_percentage` field of our [/products](https://api.exchange.coinbase.com/products) endpoint. For example, the following value represents a 3% high bid limit.

```
{
  "high_bid_limit_percentage":"0.03000000"
}

```

### 2023-JAN-26

We made a breaking change to **[Get all wrapped assets](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-all-wrapped-assets)**. Specifically, `GET /wrapped-assets/` now returns a list of json objects instead of a list of strings.

### 2023-JAN-06

Added new [Get Wrapped Asset Details](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-wrapped-asset-details) endpoint: `/wrapped-assets/{wrapped_asset_id}/` Get Wrapped Asset Details returns the following properties:

-   `circulating_supply`: Customer held wrapped assets, not pre-minted or held in abeyance.
-   `total_supply`: Wrapped assets that have been minted and exist on-chain.
-   `conversion_rate`: Underlying staked units that can be exchanged for 1 wrapped asset.

For example:

```
{
  "wrapped_assets":[
    {
      "id":"CBETH",
      "circulating_supply":"314666.9334401934801646",
      "total_supply":"1067354.0841178434801646",
      "conversion_rate":"1.0241755005866786"
    }
  ]
}

```

You can test for cbETH in the sandbox at `https://api-public.sandbox.exchange.coinbase.com/wrapped-assets/CBETH/`

### 2022-DEC-09

Request For Quote on Coinbase Exchange is live in production. RFQ allows liquidity providers to respond and interact with real-time RFQ requests. **New FIX Messages:**

-   RFQ Request (AH)
-   Quote Request (R)
-   Quote (S)
-   Quote Status Report (AI)

**New WebSocket Channel:** [RFQ Matches Channel](https://developer.chrome.com/exchange/websocket-feed/channels#rfq-matches-channel). **Updated REST API:**

-   Order and fill information for RFQ can be queried with new `market_type` query parameter.
-   New report type option, `rfq-fills`, can be used to generate reports for RFQ fills.

### 2022-NOV-22

We released [Coinbase Direct Market Data](https://developer.chrome.com/exchange/websocket-feed/overview), a WebSocket endpoint with direct access to Coinbase Exchange servers. [Authentication](https://developer.chrome.com/exchange/websocket-feed/authentication) is required. You can subscribe to both endpoints, but if `ws-direct` becomes your primary connection, we recommend using the existing `ws-feed` as a failover connection. **Coinbase Direct Market Data endpoint:** `wss://ws-direct.exchange.coinbase.com`

### 2022-OCT-26

Updated **[Get all fills](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/get-all-fills)** with new response property, `market_type`, in preparation for the upcoming Request For Quote (RFQ) feature.

### 2022-OCT-17

Added two new fields to the [Ticker Channel](https://developer.chrome.com/exchange/websocket-feed/channels#ticker-channel): `best_bid_size` and `best_ask_size`.

### 2022-OCT-04

Modify Order Request (G) is live in production. See [2022-JUN-27](#2022-jun-27) for release details.

### 2022-SEP-22

FIX Logon (A) messages now require that MsgSeqNum (34) be equal to `1`. Any other value is rejected.

### 2022-SEP-19

Added **[Balance Statement Reports](https://developer.chrome.com/api-reference/exchange-api/rest-api/reports/get-all-reports)** for historical and current crypto/fiat balances, with the Exchange REST API and in the Exchange UI at [`exchange.coinbase.com/profile/statements`](https://exchange.coinbase.com/profile/statements).

-   [Create Balance Statement Report](https://developer.chrome.com/api-reference/exchange-api/rest-api/reports/create-report)
-   [Get Balance Statement Reports](https://developer.chrome.com/api-reference/exchange-api/rest-api/reports/get-report)

The Balance Report API:

-   Leverages the existing `/reports` endpoint.
-   Adds a new report type of `balance`.
-   Adds a `balance` object to the request with `datetime` (and `group_by_portfolio_id` for the UI only).
-   Keeps the same response schema (with the possibility that `"type"="balance"`).

*Example of Balance Report API Request*

```
// Create balance statement for the portfolio tied to the API key
{
  "balance": {
    "datetime": "2022-02-25T05:00:00.000Z"
  },
  "email": "user1@example.com",
  "format": "csv",
  "type": "balance"
}

```

### 2022-AUG-24

Added a new Wrapped Asset API, comprised of two endpoints:

-   [Get all wrapped assets](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-wrapped-asset-details): Returns a list of all Coinbase supported wrapped asset IDs.
-   [Get conversion rate of wrapped asset](https://developer.chrome.com/api-reference/exchange-api/rest-api/wrapped-assets/get-wrapped-asset-conversion-rate): Returns the conversion rate of a wrapped asset to its corresponding staked asset.
    
    ```
    /wrapped-assets/{wrapped_asset_id}/conversion-rate
    
    ```
    

### 2022-AUG-16

Added a `reason` field to the WebSocket Full Channel [Change message](https://developer.chrome.com/exchange/websocket-feed/channels#change):

-   `"reason":"STP"` for Self-Trade Prevention message
-   `"reason":"modify_order"` for Modify Order Request (See [2022-JUN-27](#2022-jun-27) entry for more.)

```
{
  "type": "change",
  "reason": "STP",
  "time": "2014-11-07T08:19:27.028459Z",
  "sequence": 80,
  "order_id": "ac928c66-ca53-498f-9c13-a110027a60e8",
  "side": "sell",
  "product_id": "BTC-USD",
  "old_size": "12.234412",
  "new_size": "5.23512",
  "price": "400.23"
}

```

### 2022-JUL-27

We added support for multiple blockchain networks. Users can now withdraw USDC on Solana, and ETH, USDC, and MATIC on Polygon. A new `network` field was added for select endpoints and in the UI. Affected API endpoints are:

-   `/withdrawals/crypto` accepts an optional network parameter as part of its `POST` request parameters. This parameter designates which network the currency will be sent on. For example, if the user withdraws USDC on Solana or Ethereum, the withdrawal address is validated against the network, the appropriate fee is calculated based on the network, and the currency is sent over the network. If no network is specified in the `POST` request, the default network (Ethereum) is used.
-   `/address-book` validates new addresses against all supported networks for the currency.
-   `/address-validation` accepts an optional `network` parameter in the `address_info` object or `source` parameter in the body as part of its `POST` request parameters. The address validator uses the `network` parameter to verify that the address is valid for the designated network. If no `network` is specified in the `POST` request, the default network is used. The `source` parameter currently only accepts the value `address_book`. When `source` has the value `address_book`, the address is validated against all supported networks for the currency.

```
"id": "ATOM",
"name": "Cosmos",
"min_size": "1",
"status": "online",
"message": "",
"max_precision": "0.000001",
"convertible_to": [
],
"details": {
  "type": "crypto",
  "symbol": null,
  "network_confirmations": null,
  "sort_order": 51,
  "crypto_address_link": "https://cosmos.bigdipper.live/account/{{address}}",
  "crypto_transaction_link": "https://cosmos.bigdipper.live/transactions/{{txId}}",
  "push_payment_methods": [
    "crypto"
  ],
  "group_types": [
  ],
  "display_name": null,
  "processing_time_seconds": 5,
  "min_withdrawal_amount": 0.1,
  "max_withdrawal_amount": 302000
},
"default_network": "cosmos",
"supported_networks": [
  "id": "ETH",
  "name": "ethereum",
  "status": "offline",
  "contract_address": "0x..",
  "crypto_address_link": "address_link",
  "crypto_transaction_link": "tx_link",
  "network_confirmations": 15,
  "min_withdrawal_amount": 0.001,
  "max_withdrawal_amount": 250
]

```

### 2022-JUL-25

We increased the order size for two FIX batched messages from 10 to 15:

-   New Order Batch (U6)
-   Order Cancel Batch Request (U4)

### 2022-JUN-30

The following API parameters were removed per the lifting of [maximum and minimum order size limits](#2022-jun-02):

API Parameter

Description

base\_max\_size

Base order size max (limit orders)

base\_min\_size

Base order size min (limit orders)

max\_market\_funds

Quote order size max (market orders)

Affected features are:

-   REST API: [Get all known trading pairs](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-all-known-trading-pairs)
-   REST API: [Create a new order](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/create-new-order)
-   Websocket API [Status Channel](https://developer.chrome.com/exchange/websocket-feed/channels#status-channel)

### 2022-JUN-27

Modify Order Request (G) is now **available for testing** in the [public sandbox](https://developer.chrome.com/exchange/introduction/sandbox). It will go to production in a few weeks. Modify Order Request is an implementation of the [Order Replace Request](https://www.onixs.biz/fix-dictionary/4.2/msgtype_g_71.html) outlined in the FIX 4.2 protocol. You can send a single request to modify the price or size of an existing order using the FIX API. [WebSocket Full channel “Change”](https://developer.chrome.com/exchange/websocket-feed/channels#change) messages (and by extension the [User channel](https://developer.chrome.com/exchange/websocket-feed/channels#user-channel)) are also affected. Three fields are being added (`new_price`, `old_price`, `reason`).

**FIX API** Example FIX Request:

```
BeginString=FIX.4.2 BodyLength=265 MsgType=ORDER_CANCEL_REPLACE_REQUEST MsgSeqNum=17 SenderCompID=00000000100000000000000000000003 SendingTime=20220609-04:01:48.757 TargetCompID=Coinbase ClOrdID=907b6ae6-bcbe-441a-b7bb-d932afdb9edb OrderID=71de0cdf-938f-495b-9fad-108837bde704 OrderQty=2 OrdType=LIMIT OrigClOrdID=907b6ae6-bcbe-441a-b7bb-d932afdb9eda Price=100.00 Side=BUY Symbol=ETH-USD TransactTime=20220609-04:01:48.757 CheckSum=107

```

Tag

Name

Description

37

OrderID

Unique identifier of most recent order as assigned by broker

41

OrigClOrdID

`ClOrdID <11>` of previous order (NOT initial order of the day) when canceling or replacing an order

11

ClOrdID

Unique identifier of replacement order as assigned by institution.

55

Symbol

Must match original order

54

Side

Must match original side

38

OrderQty

Total Intended Order Quantity (including the amount already executed for this chain of orders)

60

TransactTime

Time this order request was initiated/released by the trader or trading system

40

OrdType

Only limit orders are supported for now (2)

44

Price

Price per share

**Websocket Full Channel Change Message** Example of a change message from a Modify Order Request:

```
{
  "type": "change",
  "reason": "modify_order",
  "time": "2022-06-06T22:55:43.433114Z",
  "sequence": 24753,
  "order_id": "c3f16063-77b1-408f-a743-88b7bc20cdcd",
  "side": "buy",
  "product_id": "ETH-USD",
  "old_size": "80",
  "new_size": "80",
  "old_price": "7",
  "new_price": "6"
}

```

### 2022-JUN-02

Lifted maximum and minimum order size limits. With this change:

-   Market orders are no longer subject to max or min size checks unless market funds are specified.
-   Limit orders are now subject to **notional minimum size checks only**. We repurposed the existing API parameter, `min_market_funds`, to represent this value.

Price Protection Points serve as a dynamic and real-time protection against slippage for large-sized orders. The following API parameters have been deprecated (and will be removed June 30):

API Parameter

Description

base\_max\_size

Base order size max (limit orders)

base\_min\_size

Base order size min (limit orders)

max\_market\_funds

Quote order size max (market orders)

Affected APIs are [Get all known trading pairs](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-all-known-trading-pairs) and [Create a new order](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/create-new-order).

### 2022-MAY-25

Changes were made to Coinbase Pro:

-   We disabled the Coinbase Pro API, [Deposit from Coinbase account](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/deposit-from-coinbase-account), which lets you deposit funds from a Coinbase retail account into Coinbase Pro. The endpoint `/deposits/coinbase-account` now returns a `403` when called from a Coinbase Pro account. All other payment methods into Coinbase Pro remain the same and Coinbase Exchange is not affected.
-   Creating a new API Key on Coinbase Pro with the Transfer permission has new requirements (the use of old API keys will remain the same):
    
    -   You must enable non-SMS 2FA.
    -   You must allowlist all receive addresses for your transfers.
    
    Creating new API Keys on Coinbase Exchange is not affected by this change.

### 2022-MAY-05

-   Added cancel reason to the FIX and websocket feeds.

-   For FIX, we enhanced the Execution Report to include the cancel reason using the [Text field](https://www.onixs.biz/fix-dictionary/4.2/tagNum_58.html).
-   For the websocket feed, we added a new `cancel_reason` field for authenticated messages by the user, accessible in the [Full](https://developer.chrome.com/exchange/websocket-feed/channels#full-channel) and [User](https://developer.chrome.com/exchange/websocket-feed/channels#user-channel) channels.

Supported cancel reasons are:

```
101:Time In Force
102:Self Trade Prevention
103:Admin
104:Price Bound Order Protection
105:Insufficient Funds"
106:Insufficient Liquidity
107:Broker

```

### 2022-MAY-02

We enabled new TLS/SSL features on the FIX servers for Exchange/Pro to increase security and performance. This includes limiting the supported ciphers and a new SSL certificate. Please check that your FIX SSL client:

-   Can support the new ciphers listed below.
-   Is not validating a specific SSL server certificate. If it is, you must update to the new certificate.

The new infrastructure will support only **TLSv1.2** with the following Supported Server Ciphers:

Recommend

Length

Cipher Suite

Elliptic Curve

Preferred

128 bits

ECDHE-RSA-AES128-GCM-SHA256

Curve P-256 DHE 256

Accepted

128 bits

ECDHE-RSA-AES128-SHA256

Curve P-256 DHE 256

Accepted

256 bits

ECDHE-RSA-AES256-GCM-SHA384

Curve P-256 DHE 256

Accepted

256 bits

ECDHE-RSA-AES256-SHA384

Curve P-256 DHE 256

Affected Endpoints:

-   In sandbox
    
    ```
    fix-public.sandbox.pro.coinbase.com:4198
    fix-public.sandbox.exchange.coinbase.com:4198
    
    ```
    
-   In production
    
    ```
    fix.pro.coinbase.com:4198
    fix.exchange.coinbase.com:4198
    
    ```
    

New Production FIX Server SSL Certificate:

```
-----BEGIN CERTIFICATE-----
MIIEezCCA2OgAwIBAgIQB4BoKHs2w7hI3RLss1nw7DANBgkqhkiG9w0BAQsFADBG
MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRUwEwYDVQQLEwxTZXJ2ZXIg
Q0EgMUIxDzANBgNVBAMTBkFtYXpvbjAeFw0yMjAzMzEwMDAwMDBaFw0yMzA0Mjky
MzU5NTlaMCIxIDAeBgNVBAMMFyouZXhjaGFuZ2UuY29pbmJhc2UuY29tMIIBIjAN
BgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAnmFkdVt+0k/d+HkZaX/kgjtCU6Ts
5TePVbYCGOjT05ClT7GHu7fSzUjO/BNCkBItp5WwFRdOOkWV8Zeg2WSHpDBOZNJt
cXiQx6BSTRD/myjv0NBxVsKby9BbKmj8df3C1PehakUQPdsiP7DkviYUpXz+T4FQ
PAg8M6PXu7sT7Rfbc6gY49VyyRU6slcij/Xwn4WSVWK+GMYPlsu7M2Vp0rr+sCIZ
uLHh/23TNzlGiYzXgypoZ/F57AIi+ToeRvnLe++ZfIKP37uhNxYfYrr4c3wPBoGc
LIWgSNMK9/Oue6VUCG7AVioCy2yL0CEiTmvS4Eb2urbt3iDWI+6wySW9LwIDAQAB
o4IBhzCCAYMwHwYDVR0jBBgwFoAUWaRmBlKge5WSPKOUByeWdFv5PdAwHQYDVR0O
BBYEFOlcV+BjGTnleq713Nzl4UifIPZCMDkGA1UdEQQyMDCCFyouZXhjaGFuZ2Uu
Y29pbmJhc2UuY29tghVleGNoYW5nZS5jb2luYmFzZS5jb20wDgYDVR0PAQH/BAQD
AgWgMB0GA1UdJQQWMBQGCCsGAQUFBwMBBggrBgEFBQcDAjA9BgNVHR8ENjA0MDKg
MKAuhixodHRwOi8vY3JsLnNjYTFiLmFtYXpvbnRydXN0LmNvbS9zY2ExYi0xLmNy
bDATBgNVHSAEDDAKMAgGBmeBDAECATB1BggrBgEFBQcBAQRpMGcwLQYIKwYBBQUH
MAGGIWh0dHA6Ly9vY3NwLnNjYTFiLmFtYXpvbnRydXN0LmNvbTA2BggrBgEFBQcw
AoYqaHR0cDovL2NydC5zY2ExYi5hbWF6b250cnVzdC5jb20vc2NhMWIuY3J0MAwG
A1UdEwEB/wQCMAAwDQYJKoZIhvcNAQELBQADggEBADemhPMMlRRLMlnSvaVaaSCF
ncdehfDVg3Lmr2UjcMCq2MJxriz8elgu7M6TqVGwiRVrMb4j2kD+3EUc/+V+W1dE
uX8aEzxuV01MKTFEh4R/WihCKM2l0NCfg6O8jYmtKPE9gkHe+5hW4igsM90mK+hA
GlhH7hGSouHDjkwbvlN0yrNFJXaTZE8wHd1VTDtYmzTQXkn8hAR4muesAgEtc22W
B8vbLCt6ZOeoMH/SKh2vsAmWE/3DR7+TIh9Dm+54jEwuUID+nmETaacY2wDT1XU9
eWR4xJMa8QuK1sGuO3TgvYZPCyzAXoXCjR5mVYit8PteMVwfJnTm1nLc73rAiWA=
-----END CERTIFICATE-----

```

New Sandbox FIX Server SSL Certificate:

```
-----BEGIN CERTIFICATE-----
MIIEdDCCA1ygAwIBAgIQD03L1cHVypYSDFuvcnpAHzANBgkqhkiG9w0BAQsFADBG
MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRUwEwYDVQQLEwxTZXJ2ZXIg
Q0EgMUIxDzANBgNVBAMTBkFtYXpvbjAeFw0yMjAzMjcwMDAwMDBaFw0yMzA0MjUy
MzU5NTlaMCoxKDAmBgNVBAMMHyouc2FuZGJveC5leGNoYW5nZS5jb2luYmFzZS5j
b20wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQC8LYRdqMoVNa/0M4MF
+Wkr8SiybZ95JycTE+0ZVmf92DKo4I8m/8fBtOrH0jgrhvamVSJ0lI6VFiAzlTd1
doUbliQ9Xm1aE/YHQO9J64AIP97peysgHBd+g3/Vhz33aaaU2vyHH5kPHiekU8n/
ObXPPoFd/Awul8uxxlXsVFx8oBWL2MeMjLNLLWNiGWq+lQloGKsQYVR/fQZizvpP
vyZO6pCLRId6+Wq3Tcb7NHQZc6+tePVi+5fovE+lm/yQrhjGqDzI7P4rWjJqCPrA
sYJeYFcVJhdSuFY2Ngm8eKeDP14TVEs9pkIWvyMGmB17QBPbRJipdoKu1N6fsx54
N9JDAgMBAAGjggF4MIIBdDAfBgNVHSMEGDAWgBRZpGYGUqB7lZI8o5QHJ5Z0W/k9
0DAdBgNVHQ4EFgQUa5RZ0yvv71YteSuqO1VRvmGGKv0wKgYDVR0RBCMwIYIfKi5z
YW5kYm94LmV4Y2hhbmdlLmNvaW5iYXNlLmNvbTAOBgNVHQ8BAf8EBAMCBaAwHQYD
VR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMD0GA1UdHwQ2MDQwMqAwoC6GLGh0
dHA6Ly9jcmwuc2NhMWIuYW1hem9udHJ1c3QuY29tL3NjYTFiLTEuY3JsMBMGA1Ud
IAQMMAowCAYGZ4EMAQIBMHUGCCsGAQUFBwEBBGkwZzAtBggrBgEFBQcwAYYhaHR0
cDovL29jc3Auc2NhMWIuYW1hem9udHJ1c3QuY29tMDYGCCsGAQUFBzAChipodHRw
Oi8vY3J0LnNjYTFiLmFtYXpvbnRydXN0LmNvbS9zY2ExYi5jcnQwDAYDVR0TAQH/
BAIwADANBgkqhkiG9w0BAQsFAAOCAQEATpjyCMwAOSFKFTA67UaVkDCjz/ULBY6P
L4JwTJ+7kmT+HMvGimx15CsVjne64bT5twWlzqA/l4h25HGj0hD0TU2ktqmFhfAm
DpjGVp4KgIcZpvv7oRIU4e5I422Y++2UVuATwLWdELgpnm4AVq1aqI10XrQlJeHL
gRVfV5qkr9Vsc+fk7HY7YwbNQk2jXbRaj22f6GxiJ/6VmUcCD7zZ1GZtUipv0JEy
PtWD/BbSKNx1GJnLZ6L+QytPs+MW+FEetlU/oqPuyYRhmJUBUiwKkm6yKWRj9tQf
sq0a4uLI3SUgsBv/CQ/Qa9LnRdNjvlWSKLzeIX2LU9rE/3F3oQh7HQ==
-----END CERTIFICATE-----

```

### 2022-APR-14

-   Updated the maximum number of portfolios (or profiles) to 25.

### 2022-MAR-21

-   Updated the maximum number of portfolios (or profiles) to 15.

### 2022-MAR-17

-   Added FIX message tags: `cumQty`, `leaveQty`, `AvgPx`

### 2022-FEB-22

-   REST API will enforce case sensitivity for all URLs.
    -   Example: [https://api.exchange.coinbase.com/products/BTC-USD/Ticker](https://api.exchange.coinbase.com/products/BTC-USD/Ticker) should be [https://api.exchange.coinbase.com/products/BTC-USD/ticker](https://api.exchange.coinbase.com/products/BTC-USD/ticker). Note the lowercase `t` in `ticker`.
    -   This does not apply to URL parameters, just the URL itself: `https://api.exchange.coinbase.com/orders?product_id=BTC-USD&sortedBy=created_at&sorting=desc&limit=100` is valid as the URL is lowercase. Query parameters such as `product_id` can have values with capitals. However `https://api.exchange.coinbase.com/Orders?product_id=BTC-USD&sortedBy=created_at&sorting=desc&limit=100` would be invalid as the `O` in `/Orders` is not the same URL as specified in its [docs](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/get-all-orders).

### 2022-JAN-31

-   Web Socket API users are notified when the client is actively disconnected for having a full buffer, or for being too slow to consume or read messages.

### 2022-JAN-25

-   `GET` and `POST` responses for the `/orders` endpoint will return client order id as `client_oid` if exists.

### 2021-OCT-25

-   FIX API will now enforce CheckSum validations for incoming FIX messages.

### 2021-SEP-21

-   All reports can be generated in parallel. Clients are no longer restricted to only have 3 reports being created at a time. Now clients can have up to 3 accounts reports and 3 fills reports *per* product generating at a time.

### 2021-SEP-09

-   Return the full aggregated order book for Level 2 queries under the `GET /products/<product-id>/book` endpoint.

### 2021-AUG-23

-   Reduced the set of fields returned by orders in “pending” status for `GET /orders`, `GET /orders/<id>`, and `GET /orders/client:<client_oid>` APIs. See `List Orders` documentation for more details. Orders with non-pending statuses will be unaffected by this change.

### 2021-AUG-17

-   Return client order ID rather than order ID in successful cancel order response for REST API endpoint `DELETE /orders/client:<client_oid>`.

### 2021-AUG-12

-   Require the field Symbol(55) on the following FIX API messages: OrderCancelRequest(F) and OrderStatusRequest(H). Messages (F) and (H) without Symbol(55) will be rejected.

### 2021-AUG-06

-   Add pagination support for the `GET /fills` endpoint.

### 2021-AUG-01

-   Increased the maximum number of FIX connections allowed per profile from 5 to 7.

### 2021-JUL-01

-   Added sendingTime 5 minute validation.

### 2021-JUN-22

-   Added fx\_stablecoin to products.

### 2021-JUN-21

-   Order Cancel Batch Request(U4) will accept optional ClOrdID(11) field for each cancel request. The provided ClOrdID(11) will be included in Order Cancel Reject(9) for partial reject.

### 2021-JUN-14

-   Order Cancel Batch Request(U4) will now return Order Cancel Reject(9) for partial rejected cancel request.

### 2021-JUN-10

-   Added failed status to reports.

### 2021-JUN-03

-   Our API endpoints were moved to `exchange.coinbase.com` from `prime.coinbase.com`.

***Production URLs***

Product

Old URL

New URL

Website

[https://prime.coinbase.com](https://prime.coinbase.com/)

[https://exchange.coinbase.com](https://exchange.coinbase.com/)

REST API

[https://api.prime.coinbase.com](https://api.prime.coinbase.com/)

[https://api.exchange.coinbase.com](https://api.exchange.coinbase.com/)

FIX API

tcp+ssl://fix.prime.coinbase.com:4198

tcp+ssl://fix.exchange.coinbase.com:4198

Web Socket API

wss://ws-feed.prime.coinbase.com

wss://ws-feed.exchange.coinbase.com

***Sandbox URLs***

Product

Old URL

New URL

Website

[https://public.sandbox.prime.coinbase.com](https://public.sandbox.prime.coinbase.com/)

[https://public.sandbox.exchange.coinbase.com](https://public.sandbox.exchange.coinbase.com/)

REST API

[https://api-public.sandbox.prime.coinbase.com](https://api-public.sandbox.prime.coinbase.com/)

[https://api-public.sandbox.exchange.coinbase.com](https://api-public.sandbox.exchange.coinbase.com/)

FIX API

tcp+ssl://fix-public.sandbox.prime.coinbase.com:4198

tcp+ssl://fix-public.sandbox.exchange.coinbase.com:4198

Web Socket API

wss://ws-feed-public.sandbox.prime.coinbase.com

wss://ws-feed-public.sandbox.exchange.coinbase.com

### 2021-MAY-27

-   API FIX - Order Cancel Request (F) endpoint requires the Symbol field now.

### 2021-MAY-20

-   `/fills` custom rate limit.

### 2021-MAY-14

-   Increased public and private rate limits.

### 2021-APR-22

-   Increase pagination limit from 100 to 1000.

### 2021-APR-05

-   Updated max profiles to 10 and max API keys to 200.

### 2021-FEB-04

-   The Trailing Volume endpoint has been deprecated in favor of the Fees endpoint to get the latest volumes.

### 2021-JAN-15

-   Now recommending that clients opt to batch cancel orders by profile rather than session due to recent performance optimizations.

### 2020-DEC-23

-   `HandlInst` in API FIX is no longer required.

### 2020-NOV-16

-   Addition of `max_withdrawal_amount` field in the `/currencies` endpoint.

### 2020-OCT-05

-   Authed users subscribed to the Websocket [Full](https://developer.chrome.com/exchange/websocket-feed/channels#full-channel) or [User](https://developer.chrome.com/exchange/websocket-feed/channels#user-channel) channel will now receive their order fee rates on match messages. Details can be found in documentation for the Full channel.

### 2020-OCT-02

-   Addition of cancel\_code field on canceled withdrawals.

### 2020-SEP-17

-   Addition of an endpoint to provide estimates of network fees for crypto withdrawals.
-   Addition of a parameter for crypto withdrawals to specify if the network fee should be added / deducted from the requested amount.
-   ‘fee’ and ‘subtotal’ fields added to responses for crypto withdrawals.

### 2020-SEP-14

-   The candles endpoint no longer has custom rate limits. It now shares the same rate limit with every other public endpoint.

### 2020-SEP-03

-   The maximum number of open orders (i.e. limit orders + stop orders) per product per profile will be 500. Profiles that exceed this threshold will be unable to place new orders on that product until the number of open orders is below 500.

### 2020-JUN-18

-   Users can retrieve historical [deposits](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/get-all-transfers) and [withdrawals](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/get-all-transfers).

### 2020-JUN-17

-   Generate an address for crypto deposits. See reference [here](https://developer.chrome.com/api-reference/exchange-api/rest-api/coinbase-accounts/generate-crypto-address).

### 2020-JUN-15

-   Expose `min_market_funds`, `max_market_funds` fields in the `/products` endpoint.

### 2020-JUN-12

-   Users can retrieve information regarding their transfer, buy, and sell limits at `/users/self/exchange-limits`. Refer to the [Limits](https://developer.chrome.com/api-reference/exchange-api/rest-api/users/get-user-exchange-limits) API for more information.

### 2020-APR-27

-   Fill execution reports will show fee rates associated with the user’s order. Refer to the FIX ExecutionReport API for details on format.

### 2020-FEB-20

-   Execution Reports from Order Status Requests will return `ClOrdID`, if it is supplied, even if the order isn’t found.

### 2020-FEB-10

-   Activate messages on the Websocket feed will no longer expose `taker_fee_rate`.

### 2019-DEC-16

-   Rate limiting changing from a per user basis to per profile basis.

### 2019-SEP-30

-   Order Status Request no longer allows the wildcard option.
-   Order Status Request returns pending and done orders when you use OrderID or ClOrdID.
-   Scheduled disconnects are on Mondays and Thursdays at 11 AM Pacific Time.