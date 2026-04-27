# orders

Advanced Trade API offers order management with the Orders API (`/orders`). You can configure orders with the [Create Order](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/create-order) API and view orders with [List Orders](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/list-orders).

## Fulfillment Policies

Order type **fulfillment policies** (`GTC`, `GTD`, `IOC`, etc.) correspond to the **time-in-force** policy for that order type.

-   **Good Till Canceled** (`gtc`): orders remain open on the book until canceled.
-   **Good Till Date** (`gtd`): orders are valid till a specified date or time.
-   **Immediate Or Cancel** (`ioc`): orders instantly cancel the remaining size of the limit order instead of opening it on the book.

## Order Types

### Market Orders

Market orders are used to `BUY` or `SELL` a desired product at the given market price. To buy a product, provide a `quote_size` or `base_size`; to sell, provide a `base_size`.

##### Possible Market Order Types

-   `market_market_ioc`
-   `market_market_fok` (Perpetuals only)

### Limit Orders

Limit orders are triggered based on the instructions around quantity and price: `base_size` represents the quantity of your base currency to spend; `limit_price` represents the maximum price at which the order should be filled.

##### Possible Limit Order Types

-   `limit_limit_gtc`
-   `limit_limit_gtd`
-   `sor_limit_ioc`

### Stop Orders

Stop orders are triggered based on the movement of the last trade price. The last trade price is the last price at which an order was filled.

##### Possible Stop Order Types

-   `stop_limit_stop_limit_gtc`
-   `stop_limit_stop_limit_gtd`

### Fill or Kill (FOK) Orders

Fill or Kill orders will only be posted to the order book if they would be immediately and completely filled. This results in a Taker Order.

##### Possible Fill or Kill Order Types

-   `limit_limit_fok`

### Bracket Orders

Bracket orders are an order type that allows selling with a limit price while mitigating potential losses in volatile markets. A single `SELL` order can specify the limit price (in quote currency) for momentum trading strategies, and a trigger price is used to automatically trigger a sale to reduce risk exposure in case the market moves against you. Note: As soon as a fill occurs for the order at one of the specified price levels, the other side is automatically disabled. Execution of downside protection is not guaranteed during high market volatility.

##### Possible Bracket Order Types

-   `trigger_bracket_gtc`
-   `trigger_bracket_gtd`

### Attached Take Profit/Stop Loss Orders

An Attached Take Profit/Stop Loss (TP/SL) order allows you to set profit and loss levels on order configuration. Include `attached_order_configuration` in the [Create Order request body](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/create-order) with `trigger_bracket_gtc` to create a TP/SL order. Do not include size as the attached TP/SL order will have the same size as the originating order. Example Attached TP/SL PostOrder request body:

```
{
  "client_order_id": "YOUR_CLIENT_ORDER_ID",
  "product_id": "ETH-USDC",
  "side": "BUY",
  "order_configuration": {
    "limit_limit_gtc": {
      "baseSize": "0.01",
      "limitPrice": "1500"
    }
  },
  "attached_order_configuration": {
    "trigger_bracket_gtc": {
      "limit_price": "1600",
      "stop_trigger_price": "1300"
    }
  }
}

```