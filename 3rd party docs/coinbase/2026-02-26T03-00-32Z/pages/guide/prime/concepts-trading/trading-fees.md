# trading fees

Trading fees on Coinbase Prime are applied at the time of order execution using a negotiated basis points rate. However, the exact methodology for fee calculation depends on several factors, including order type, base or quote quantity specification, and other order parameters. Below is a comprehensive breakdown of how fees are calculated and applied. **Note**: All fee amounts shown in the examples on this page are illustrative only and do not reflect the actual negotiated fee rates you would receive as a Prime client.

## All In vs Cost Plus Pricing

Prime offers two fee structures that determine how trading costs are presented in API responses: **All In Pricing**: Uses a single consolidated basis points rate for all trading activity. The total fee is delivered exclusively via the `commission` field. **Cost Plus Pricing**: Separates fees into two components:

-   **`commission`**: The Prime trading fee
-   **`exchange_fee`**: External exchange fees passed directly through to the client

Under Cost Plus pricing, you must monitor both the `commission` and `exchange_fee` fields to calculate your total trading costs. The examples below demonstrate All In Pricing structure. For Cost Plus pricing, both `commission` and `exchange_fee` fields would contain values, and both must be included when calculating total trading costs.

## Fee Calculations

**Fee Currency**: Fees are always charged in the quote currency of the trading pair, regardless of order type or direction. For example, in an ETH-USD pair, fees are always charged in USD (the quote currency). This is also true for crypto-to-crypto orders - in an ETH-BTC pair, fees are taken in BTC. **Order Denomination**: Prime supports specifying orders in both base units (e.g., “1 ETH”) and quote units (e.g., “500 USD of ETH”). While fees are always charged in quote currency, how you denominate your order affects the user experience and how fees impact your final amounts. Before examining the specifics, consider each of the following order scenarios:

1.  **Quote buy**: “I wish to buy 500 USD of ETH”
2.  **Base buy**: “I wish to buy 1 ETH”
3.  **Quote sell**: “I wish to sell 500 USD of ETH”
4.  **Base sell**: “I wish to sell 1 ETH”

In each scenario, the user experience is considered to determine how Coinbase Prime applies its trading fee. The examples below show actual REST API responses when retrieving order status. For more information on order tracking, visit [Trading Basics](https://developer.chrome.com/prime/concepts/trading/trading).

## Quote Buy

In a quote buy scenario (e.g., “I wish to buy 500 USD of ETH”), the trading fee is deducted directly from the specified quote amount. This approach ensures users know exactly how much they will spend when placing an order. Quote purchases typically result in fractional cryptocurrency amounts due to Prime’s support for high decimal precision. This design optimizes the user experience by treating the quote value as the maximum expenditure. The example below highlights the most important fields in a quote buy order response:

-   **`quote_value`** - The original order size in quote units from your order request
-   **`filled_value`** - The actual amount of quote currency used to purchase the asset (after fees)
-   **`commission`** - The Coinbase Prime trading fee for this order
-   **`filled_quantity`** - The amount of base currency (i.e. BTC) actually purchased
-   **`average_filled_price`** - The average execution price before fees
-   **`net_average_filled_price`** - The effective price per unit including fees

```
{
    "id": "c2b13cb4-888c-4dff-a8cf-6311bebfbd0d",
    "user_id": "2900b9b2-9b3c-5e4e-a35d-5b72453fa907",
    "portfolio_id": "314dbd76-4459-41cd-ba9a-dccdd86b44e2",
    "product_id": "BTC-USD",
    "side": "BUY",
    "client_order_id": "1751547966404173000",
    "type": "MARKET",
    "base_quantity": "",
    "quote_value": "10",
    "limit_price": "",
    "start_time": null,
    "expiry_time": null,
    "status": "FILLED",
    "time_in_force": "IMMEDIATE_OR_CANCEL",
    "created_at": "2025-07-03T13:06:06.582862Z",
    "filled_quantity": "0.00009135",
    "filled_value": "9.98",
    "average_filled_price": "109302.48443842565603",
    "commission": "0.02",
    "exchange_fee": "",
    "historical_pov": "",
    "stop_price": "",
    "net_average_filled_price": "109469.0749863163656267",
    "user_context": "",
    "client_product_id": "",
    "post_only": false,
    "order_edit_history": [],
    "is_raise_exact": false,
    "display_size": "",
    "edit_history": [],
    "display_quote_size": "",
    "display_base_size": ""
}

```

## Base Orders (Buy & Sell)

Base orders allow you to specify an exact amount of cryptocurrency to buy or sell. The key principle for base orders is that you always get exactly what you request - fees are handled transparently in the quote currency.

### Base Buy Example

“I wish to buy 1 ETH” → You receive exactly 1 ETH, and fees are added to the total USD cost.

### Base Sell Example

“I wish to sell 1 ETH” → You sell exactly 1 ETH, and fees are deducted from the USD proceeds. In both cases, the important fields to monitor are:

-   **`base_quantity`** - The exact amount of cryptocurrency you requested to buy/sell
-   **`filled_quantity`** - The actual amount of cryptocurrency bought/sold (matches your request)
-   **`commission`** - The Coinbase Prime trading fee charged in quote currency
-   **`filled_value`** - The quote currency amount transacted (see below)
-   **`average_filled_price`** - The average market execution price before fees
-   **`net_average_filled_price`** - The effective price per unit including fees

The `filled_value` field has different fee semantics depending on order side:

-   **Base Buy**: `filled_value` does **not** include fees.
-   **Base Sell**: `filled_value` **includes** the fee deduction. This is the net amount of quote currency you receive.

```
{
    "id": "5fba091b-6c2e-4e84-8100-ad7700b9cd9f",
    "user_id": "58459a9f-918d-58ec-92c0-bc9621e0cc47",
    "portfolio_id": "314dbd76-4459-41cd-ba9a-dccdd86b44e2",
    "product_id": "BTC-USD",
    "side": "BUY",
    "client_order_id": "2bcf1d1b-b770-46ef-9e95-bb70ab2ff7a1",
    "type": "MARKET",
    "base_quantity": "0.001",
    "quote_value": "",
    "limit_price": "",
    "start_time": null,
    "expiry_time": null,
    "status": "FILLED",
    "time_in_force": "IMMEDIATE_OR_CANCEL",
    "created_at": "2025-07-02T20:02:06.322393Z",
    "filled_quantity": "0.001",
    "filled_value": "109.55",
    "average_filled_price": "109550",
    "commission": "0.27",
    "exchange_fee": "",
    "historical_pov": "",
    "stop_price": "",
    "net_average_filled_price": "109820",
    "user_context": "",
    "client_product_id": "",
    "post_only": false,
    "order_edit_history": [],
    "is_raise_exact": false,
    "display_size": "",
    "edit_history": [],
    "display_quote_size": "",
    "display_base_size": ""
}

```

## Quote Sell

Quote sell orders work similarly to quote buy orders, but with one additional feature: **Size Equals Total After Fees (`is_raise_exact`)** Quote sells support an optional `is_raise_exact` boolean parameter that controls how fees affect your proceeds:

-   **`is_raise_exact: true`** - You receive exactly the requested quote amount. For example, selling “100 USD of XRP” with `is_raise_exact: true` means you receive exactly 100 USD, and Prime automatically increases the amount of XRP sold to cover trading fees.
-   **`is_raise_exact: false` or omitted** - You receive the requested quote amount minus trading fees. For example, selling “100 USD of XRP” without `is_raise_exact` means you will receive 100 USD minus fees.