# order types

Coinbase Prime supports a variety of order types to meet different trading strategies and execution requirements. Each order type has specific parameters and behaviors that determine how and when the order is executed. Understanding these order types is crucial for effective trading on the Prime platform.

## Market Orders

Market orders are executed immediately at the best available price in the market. They provide the fastest execution but may result in price slippage, especially for large orders or in volatile markets. Market orders can partially execute and cancel when market conditions prevent full execution. **Key Characteristics:**

-   Immediate execution or cancellation at current market prices
-   No price guarantee - execution price depends on market liquidity
-   Best for quick entry/exit when price is less critical than speed
-   Can partially execute and cancel if full execution is not possible

**Slippage Details:**

-   Market orders are submitted as limit IOC orders with a 5% slippage collar
-   Any unfilled size is retried against the latest order book with a fresh collar
-   Orders are capped at 5 seconds total, after which they are either filled, partially filled, or canceled

**Overwithholding Protection** Prime implements a 5% overwithholding safeguard on market quote sells to prevent issues where market movements or liquidity constraints could result in selling more cryptocurrency than you actually hold in your trading balance. This means:

-   If you hold 100 XRP valued at 3 USD each (total value: 300 USD), you can only place a quote sell order for up to 285 USD (95% of 300 USD)
-   This protection prevents scenarios where price slippage or market volatility during execution would require selling more XRP than available

This overwithholding protection can be bypassed by using a limit order type.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ETH-USD")
    .side(OrderSide.BUY)
    .type(OrderType.MARKET)
    .baseQuantity("0.003")
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.003",
    Side = OrderSide.BUY,
    ProductId = "ETH-USD",
    Type = OrderType.MARKET,
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.003",
        Side:          "BUY",
        ProductId:     "ETH-USD",
        Type:          "MARKET",
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.003",
    side="BUY",
    product_id="ETH-USD",
    type="MARKET",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.003",
    side: OrderSide.BUY,
    productId: "ETH-USD",
    type: OrderType.MARKET,
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## Time in Force Options

All order types support various time-in-force options that determine how long the order remains active:

-   **GOOD\_UNTIL\_CANCELLED (GTC)**: Order stays on the books until cancelled
-   **GOOD\_UNTIL\_DATE\_TIME (GTD)**: Order expires at a specific date/time (requires `expiry_time`)
-   **IMMEDIATE\_OR\_CANCEL (IOC)**: Order begins executing immediately at submission or is cancelled
-   **FILL\_OR\_KILL (FOK)**: Order is fully executed (filled) immediately at submission or is cancelled

## Limit Orders

Limit orders allow you to specify the maximum price you’re willing to pay (for buy orders) or the minimum price you’re willing to accept (for sell orders). These orders are only executed if the market price is at or better than the specified limit price. **Key Characteristics:**

-   Price protection - orders only execute at or better than the specified price
-   Will not execute immediately if market price isn’t better than the limit price
-   Requires `limit_price` parameter
-   Supports various time-in-force options (GTC, GTD, IOC, FOK)
-   Can be combined with `post_only` flag to ensure the order adds liquidity

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ETH-USD")
    .side(OrderSide.SELL)
    .type(OrderType.LIMIT)
    .baseQuantity("0.003")
    .limitPrice("2000.00")
    .timeInForce(TimeInForceType.GOOD_UNTIL_CANCELLED)
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.003",
    LimitPrice = "2000.00",
    Side = OrderSide.SELL,
    ProductId = "ETH-USD",
    Type = OrderType.LIMIT,
    TimeInForce = TimeInForceType.GOOD_UNTIL_CANCELLED,
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.003",
        LimitPrice:    "2000.00",
        Side:          "SELL",
        ProductId:     "ETH-USD",
        Type:          "LIMIT",
        TimeInForce:   "GOOD_UNTIL_CANCELLED",
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.003",
    limit_price="2000.00",
    side="SELL",
    product_id="ETH-USD",
    type="LIMIT",
    time_in_force="GOOD_UNTIL_CANCELLED",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.003",
    limitPrice: "2000.00",
    side: OrderSide.SELL,
    productId: "ETH-USD",
    type: OrderType.LIMIT,
    timeInForce: TimeInForceType.GOOD_UNTIL_CANCELLED,
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## Stop-Limit Orders

Stop-limit orders combine the features of stop orders and limit orders. They are triggered when the market price reaches a specified stop price, at which point they become limit orders with a specified limit price. **Key Characteristics:**

-   Requires both `stop_price` and `limit_price` parameters
-   No limit on max stop price as long as it’s +50bps best ask for buys or -50bps best bid for sells
-   Limit price must be above stop price for buys; below stop price for sells
-   Limit price must be within 100bps of the stop price
-   Order becomes a limit order when stop price is reached
-   Provides price protection through the limit price
-   Useful for risk management and automated trading strategies.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ETH-USD")
    .side(OrderSide.SELL)
    .type(OrderType.STOP_LIMIT)
    .baseQuantity("0.003")
    .stopPrice("3000.00")
    .limitPrice("3001.00")
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.003",
    StopPrice = "3000.00",
    LimitPrice = "3001.00",
    Side = OrderSide.SELL,
    ProductId = "ETH-USD",
    Type = OrderType.STOP_LIMIT,
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.003",
        StopPrice:     "3000.00",
        LimitPrice:    "3001.00",
        Side:          "SELL",
        ProductId:     "ETH-USD",
        Type:          "STOP_LIMIT",
        ExpiryTime:    time.Now().UTC().Add(24 * time.Hour).Format(time.RFC3339),
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import datetime
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.003",
    stop_price="3000.00",
    limit_price="3001.00",
    side="SELL",
    product_id="ETH-USD",
    type="STOP_LIMIT",
    expiry_time=(datetime.datetime.now() + datetime.timedelta(days=1)).isoformat() + "Z",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.003",
    stopPrice: "3000.00",
    limitPrice: "3001.00",
    side: OrderSide.SELL,
    productId: "ETH-USD",
    type: OrderType.STOP_LIMIT,
    expiryTime: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(),
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## TWAP Orders (Time-Weighted Average Price)

TWAP orders are designed to execute large orders over time to minimize market impact. The order is broken down into smaller pieces and executed at regular intervals throughout a specified time period. **Key Characteristics:**

-   Requires `start_time`, `expiry_time`, and `limit_price` parameters
-   The minimum size of a TWAP order is at least $100 notional per time bucket.
-   Executes over a specified time period to minimize market impact
-   Useful for large orders that could move the market
-   Requires either `base_quantity` or `quote_value`

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ETH-USD")
    .side(OrderSide.BUY)
    .type(OrderType.TWAP)
    .baseQuantity("0.5")
    .limitPrice("3100.00")
    .startTime(Instant.now().toString())
    .expiryTime(Instant.now().plus(4, ChronoUnit.HOURS).toString())
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.5",
    LimitPrice = "3100.00",
    Side = OrderSide.BUY,
    ProductId = "ETH-USD",
    Type = OrderType.TWAP,
    StartTime = DateTimeOffset.UtcNow.ToString("o"),
    ExpiryTime = DateTimeOffset.UtcNow.AddHours(4).ToString("o"),
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.5",
        LimitPrice:    "3100.00",
        Side:          "BUY",
        ProductId:     "ETH-USD",
        Type:          "TWAP",
        StartTime:     time.Now().UTC().Format(time.RFC3339),
        ExpiryTime:    time.Now().UTC().Add(4 * time.Hour).Format(time.RFC3339),
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import datetime
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.5",
    limit_price="3100.00",
    side="BUY",
    product_id="ETH-USD",
    type="TWAP",
    start_time=datetime.datetime.now().isoformat() + "Z",
    expiry_time=(datetime.datetime.now() + datetime.timedelta(hours=4)).isoformat() + "Z",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.5",
    limitPrice: "3100.00",
    side: OrderSide.BUY,
    productId: "ETH-USD",
    type: OrderType.TWAP,
    startTime: new Date().toISOString(),
    expiryTime: new Date(Date.now() + 4 * 60 * 60 * 1000).toISOString(),
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## VWAP Orders (Volume-Weighted Average Price)

VWAP orders execute based on market volume patterns, aiming to achieve execution prices that track the volume-weighted average price of the market. These orders are particularly useful for institutional trading where minimizing market impact is critical. **Key Characteristics:**

-   Requires `start_time`, `expiry_time`, and `limit_price` parameters
-   The minimum size of a VWAP order is at least $100 notional per time bucket.
-   Executes based on market volume patterns
-   Useful for institutional and large-scale trading.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ETH-USD")
    .side(OrderSide.SELL)
    .type(OrderType.VWAP)
    .baseQuantity("0.5")
    .limitPrice("3100.00")
    .startTime(Instant.now().toString())
    .expiryTime(Instant.now().plus(6, ChronoUnit.HOURS).toString())
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.5",
    LimitPrice = "3100.00",
    Side = OrderSide.SELL,
    ProductId = "ETH-USD",
    Type = OrderType.VWAP,
    StartTime = DateTimeOffset.UtcNow.ToString("o"),
    ExpiryTime = DateTimeOffset.UtcNow.AddHours(6).ToString("o"),
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.5",
        LimitPrice:    "3100.00",
        Side:          "SELL",
        ProductId:     "ETH-USD",
        Type:          "VWAP",
        StartTime:     time.Now().UTC().Format(time.RFC3339),
        ExpiryTime:    time.Now().UTC().Add(6 * time.Hour).Format(time.RFC3339),
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import datetime
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.5",
    limit_price="3100.00",
    side="SELL",
    product_id="ETH-USD",
    type="VWAP",
    start_time=datetime.datetime.now().isoformat() + "Z",
    expiry_time=(datetime.datetime.now() + datetime.timedelta(hours=6)).isoformat() + "Z",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.5",
    limitPrice: "3100.00",
    side: OrderSide.SELL,
    productId: "ETH-USD",
    type: OrderType.VWAP,
    startTime: new Date().toISOString(),
    expiryTime: new Date(Date.now() + 6 * 60 * 60 * 1000).toISOString(),
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## RFQ Orders (Request for Quote)

RFQ orders allow you to request quotes from liquidity providers before executing a trade. This order type is useful for getting competitive pricing on large trades or less liquid assets. **Key Characteristics:**

-   Requests quotes from multiple liquidity providers
-   The RFQ quote price is valid for ~2.5 seconds
-   Allows comparison of pricing before execution
-   May result in better pricing through competition vs CLOB
-   Requires either `base_quantity` or `quote_value`

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("SOL-USD")
    .side(OrderSide.BUY)
    .type(OrderType.RFQ)
    .baseQuantity("0.1")
    .clientOrderId(UUID.randomUUID().toString())
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "0.1",
    Side = OrderSide.BUY,
    ProductId = "SOL-USD",
    Type = OrderType.RFQ,
    ClientOrderId = Guid.NewGuid().ToString()
};
var response = ordersService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "0.1",
        Side:          "BUY",
        ProductId:     "SOL-USD",
        Type:          "RFQ",
        ClientOrderId: uuid.New().String(),
    },
}
response, err := ordersService.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateOrderRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CreateOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="0.1",
    side="BUY",
    product_id="SOL-USD",
    type="RFQ",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "0.1",
    side: OrderSide.BUY,
    productId: "SOL-USD",
    type: OrderType.RFQ,
    clientOrderId: uuidv4()
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

## Order Parameters Summary

Parameter

Required For

Description

`product_id`

All

The trading pair (e.g., “BTC-USD”)

`side`

All

BUY or SELL

`type`

All

Order type (MARKET, LIMIT, etc.)

`client_order_id`

All

Unique client-generated identifier

`stp_id`

All

Self-Trade prevention (Stp) Id

`base_quantity`

Most

Order size in base asset units

`quote_value`

Most

Order size in quote asset units

`limit_price`

LIMIT, TWAP, VWAP, STOP\_LIMIT

Maximum/minimum execution price

`stop_price`

STOP\_LIMIT

Price that triggers the order

`start_time`

TWAP, VWAP

When the order should start executing

`expiry_time`

TWAP, VWAP, GTD orders

When the order expires

`time_in_force`

LIMIT, STOP\_LIMIT

How long the order remains active

`post_only`

LIMIT

Ensures order adds liquidity

`display_quote_size`

LIMIT

This is the maximum order size that will show up on venue order books. Specifying a value here effectively makes a LIMIT order into an “iceberg” style order.

`display_base_size`

LIMIT

This is the maximum order size that will show up on venue order books. Specifying a value here effectively makes a LIMIT order into an “iceberg” style order.

Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).