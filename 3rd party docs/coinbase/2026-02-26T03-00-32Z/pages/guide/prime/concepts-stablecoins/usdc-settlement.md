# usdc settlement

USDC can be used as a settlement currency, effectively allowing it to be used as a quote currency on existing USD order books. This enables you to trade using USDC while accessing the deep liquidity of Prime’s USD order books.

## Tradable Products

All USD product pairs (e.g. BTC-USD, ETH-USD, SOL-USD, etc.) are eligible for USDC settlement. This means that USDC can be substituted as the settlement currency in either direction:

-   **Buy + USDC**: Spend USDC to buy the base asset (e.g. spend USDC to buy BTC)
-   **Sell + USDC**: Sell the base asset and receive USDC (e.g. sell BTC and receive USDC)

When [creating an order](https://developer.chrome.com/prime/concepts/trading/trading#creating-a-trade) via REST, you can specify USDC as the settlement currency by including `settl_currency=USDC` in your order request. This forces a product like BTC-USD to be effectively treated as BTC-USDC while still using the BTC-USD order book’s existing liquidity. Coinbase handles the USD to USDC conversion in parallel with order placement, which means there is no added latency to your order execution. USDC settlement also works with trade financing when your financing agreement includes support for borrowing USDC.

### REST API

When creating an order via the REST API, include the `settl_currency` parameter:

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
from prime_sdk.enums import OrderSide, OrderType
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    product_id="ETH-USD",
    side=OrderSide.BUY,
    type=OrderType.MARKET,
    base_quantity="0.1",
    settl_currency="USDC",  # Settle in USDC instead of USD
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

### FIX Protocol

USDC settlement is also supported via FIX using tag 120 (`SettlCurrency`). Include this tag in your New Order Single message to specify USDC as the settlement currency.

## Tracking USDC Settlement Orders

When [retrieving order details](https://developer.chrome.com/prime/concepts/trading/trading#tracking-an-order), you’ll receive additional fields to help reconcile USDC vs USD orders:

-   **REST API**: The `client_product_id` field is included in the order response
-   **FIX**: The `SettlCurrency` field is included in the Execution Report (ER)
-   **WebSocket**: The `settl_currency` field is included in order updates

These fields are the key identifiers you must use to reconcile orders settled in USDC versus those settled in USD.

### REST API Response Example

When retrieving an order that was settled in USDC, the response will include the `client_product_id` field showing the USDC pair. For example, an order on ETH-USD with USDC settlement will show `client_product_id` as “ETH-USDC”:

```
{
   "id": "uuid",
   "user_id": "uuid",
   "portfolio_id": "uuid",
   "product_id": "ETH-USD",
   "side": "BUY",
   "client_order_id": "uuid",
   "type": "LIMIT",
   "base_quantity": "1",
   "quote_value": "",
   "limit_price": "4290",
   "start_time": null,
   "expiry_time": null,
   "status": "FILLED",
   "time_in_force": "GOOD_UNTIL_CANCELLED",
   "created_at": "2025-08-21T23:56:40.671979Z",
   "filled_quantity": "1",
   "filled_value": "4224.51",
   "average_filled_price": "4224.5150007111",
   "commission": "10.56",
   "exchange_fee": "",
   "historical_pov": "",
   "stop_price": "",
   "net_average_filled_price": "4235.07",
   "user_context": "",
   "client_product_id": "ETH-USDC",
   "post_only": false,
   "order_edit_history": [],
   "is_raise_exact": false,
   "display_size": "",
   "edit_history": [],
   "display_quote_size": "",
   "display_base_size": ""
}

```