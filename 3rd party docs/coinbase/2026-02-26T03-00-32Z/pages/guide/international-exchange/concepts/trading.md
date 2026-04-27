# trading

Trading on Coinbase International Exchange (INTX) is conducted through the Orders API. This guide covers order creation, management, modification, and cancellation workflows.

## Creating an Order

Use [Create Order](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/create-order) to place trades. Each order requires an instrument, side, size, type, and time-in-force specification.

```
import uuid
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import CreateOrderRequest
from intx_sdk.enums import OrderSide, OrderType, TimeInForce
client = IntxServicesClient.from_env()
request = CreateOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    client_order_id=str(uuid.uuid4()),
    instrument="BTC-PERP",
    side=OrderSide.BUY.value,
    size="0.1",
    type=OrderType.LIMIT.value,
    price="50000",
    tif=TimeInForce.GTC.value
)
response = client.orders.create_order(request)
print(response)

```

To learn more about this SDK, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

## Order Types

INTX supports multiple order types for different trading strategies:

Order Type

Description

`LIMIT`

Execute at specified price or better

`MARKET`

Execute immediately at best available price

`STOP`

Trigger a market order when stop price is reached

`STOP_LIMIT`

Trigger a limit order when stop price is reached

### Limit Order Example

```
request = CreateOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    client_order_id=str(uuid.uuid4()),
    instrument="ETH-PERP",
    side=OrderSide.BUY.value,
    size="1.0",
    type=OrderType.LIMIT.value,
    price="3000",
    tif=TimeInForce.GTC.value
)
response = client.orders.create_order(request)

```

### Market Order Example

```
request = CreateOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    client_order_id=str(uuid.uuid4()),
    instrument="ETH-PERP",
    side=OrderSide.SELL.value,
    size="1.5",
    type=OrderType.MARKET.value,
    tif=TimeInForce.IOC.value
)
response = client.orders.create_order(request)

```

### Stop-Limit Order Example

```
request = CreateOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    client_order_id=str(uuid.uuid4()),
    instrument="BTC-PERP",
    side=OrderSide.SELL.value,
    size="0.5",
    type=OrderType.STOP_LIMIT.value,
    stop_price="48000",
    stop_limit_price="47500",
    tif=TimeInForce.GTC.value
)
response = client.orders.create_order(request)

```

## Listing Open Orders

Retrieve active orders using [List Open Orders](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/list-open-orders). Orders can be filtered by instrument, side, order type, and other parameters.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import ListOpenOrdersRequest
client = IntxServicesClient.from_env()
request = ListOpenOrdersRequest(
    portfolio="PORTFOLIO_ID_HERE"
)
response = client.orders.list_open_orders(request)
print(response)

```

## Getting Order Details

Retrieve details for a specific order using [Get Order Details](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/get-order-details).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import GetOrderDetailsRequest
client = IntxServicesClient.from_env()
request = GetOrderDetailsRequest(
    portfolio="PORTFOLIO_ID_HERE",
    order_id="your-order-id"
)
response = client.orders.get_order_details(request)
print(response)

```

## Modifying an Order

Modify open orders using [Modify Open Order](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/modify-open-order). You can adjust price, stop price, or size without cancelling and recreating the order.

```
import uuid
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import ModifyOpenOrderRequest
client = IntxServicesClient.from_env()
request = ModifyOpenOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    id="your-order-id",
    client_order_id=str(uuid.uuid4()),
    price="51000"
)
response = client.orders.modify_open_order(request)
print(response)

```

## Cancelling an Order

Cancel a single order using [Cancel Order](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/cancel-order).

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import CancelOrderRequest
client = IntxServicesClient.from_env()
request = CancelOrderRequest(
    portfolio="PORTFOLIO_ID_HERE",
    id="your-order-id"
)
response = client.orders.cancel_order(request)
print(response)

```

## Cancelling Multiple Orders

Cancel multiple orders at once using [Cancel Orders](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/cancel-orders). This can cancel all orders or filter by instrument, side, or type.

```
from intx_sdk import IntxServicesClient
from intx_sdk.services.orders import CancelOrdersRequest
from intx_sdk.enums import OrderSide
client = IntxServicesClient.from_env()
# Cancel all BUY orders for BTC-PERP
request = CancelOrdersRequest(
    portfolio="PORTFOLIO_ID_HERE",
    instrument="BTC-PERP",
    side=OrderSide.BUY.value
)
response = client.orders.cancel_orders(request)
print(response)

```