# trading

## Listing Product Pairs

Tradable product pairs are returned by [List Products](https://developer.chrome.com/api-reference/prime-api/rest-api/products/list-portfolio-products). This endpoint’s response includes all enabled products for a given portfolio, along with crucial details such as minimum/maximum order increments and order book precision. This can be used to validate order parameters, ensure compliance with product constraints, and power trading interfaces or workflows.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
ProductsService productsService = PrimeServiceFactory.createProductsService(client);
ListPortfolioProductsRequest request = new ListPortfolioProductsRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .build();
ListPortfolioProductsResponse response = productsService.listPortfolioProducts(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var productsService = new ProductsService(client);
var request = new ListPortfolioProductsRequest("PORTFOLIO_ID_HERE");
var response = productsService.ListPortfolioProducts(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
productsService := products.NewProductsService(client)
request := &products.ListProducts{
    PortfolioId: "PORTFOLIO_ID_HERE",
}
response, err := productsService.ListProducts(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.products import ProductsService, ListProducts
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
products_service = ProductsService(client)
request = ListProducts(
    portfolio_id="PORTFOLIO_ID_HERE",
)
response = products_service.list_products(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl list-products --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const productsService = new ProductsService(client);
productsService.listProducts({
    portfolioId: 'PORTFOLIO_ID_HERE'
}).then(async (response) => {
    console.log('Products: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Creating a Trade

Trading via the Prime REST API is conducted through the [Create Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-order) endpoint. When an order is created, the **product** (for example, `BTC-USD`) and the **portfolio ID** must be specified. The portfolio ID determines which trading balance will be used for debits and credits, and can be obtained from [List Portfolios](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/list-portfolios) or from the URL of the Prime web UI. Any order algorithm supported in the Prime UI is also available through the Create Order API. Prime supports specifying orders in both base units (e.g., “10 ETH”) and quote units (e.g., “100,000 USD of ETH”).

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
    .productId("ADA-USD")
    .side(OrderSide.BUY)
    .type(OrderType.MARKET)
    .baseQuantity("10.0")
    .clientOrderId(UUID.randomUUID().toString())
    .build());
CreateOrderResponse orderResponse = ordersService.createOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "5",
    LimitPrice = "0.32",
    Side = OrderSide.BUY,
    ProductId = "ADA-USD",
    Type = OrderType.LIMIT,
    ExpiryTime = new DateTimeOffset(DateTime.UtcNow.AddMinutes(5)).ToString("o"),
    ClientOrderId = Guid.NewGuid().ToString()
};
var createOrderResponse = orderService.CreateOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "5",
        LimitPrice:    "0.32",
        Side:          "BUY",
        ProductId:     "ADA-USD",
        Type:          "LIMIT",
        ExpiryTime:    time.Now().UTC().Add(5 * time.Minute).Format(time.RFC3339),
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
    base_quantity="5",
    limit_price="0.32",
    side="BUY",
    product_id="ADA-USD",
    type="LIMIT",
    expiry_time=(datetime.datetime.now() + datetime.timedelta(minutes=5)).isoformat() + "Z",
    client_order_id=str(uuid.uuid4()),
)
response = orders_service.create_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const ordersService = new OrdersService(client);
const today = new Date();
ordersService.createOrder({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "5",
    limitPrice: "0.32",
    side: OrderSide.BUY,
    productId: "ADA-USD",
    type: OrderType.LIMIT,
    expiryTime: date.setDate(date.getDate() + 1),
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

An [Order Preview](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-preview) can also be submitted, which is useful in scenarios where a preview screen is needed, or to obtain a general idea of trading conditions. This is not a hold.

## Tracking an order

When an order is successfully created, the API returns an order ID. This ID can be used to check the current status or attempt to cancel the order. There are multiple ways to track order status:

-   **Orders WebSocket**: Subscribe to real-time updates for order status and fills via the [Orders Websocket](https://developer.chrome.com/prime/websocket-feed/channels#orders-channel)
-   **Get Order by ID**: Poll the [Get Order by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id) endpoint to retrieve detailed information about the order, including price and quantity details.

### Order State

Orders progress through different statuses during their lifecycle. Here are the possible order states:

1.  **`PENDING`** - The order has been accepted by Prime but has not yet been placed on the order book
2.  **`OPEN`** - The order is active on the order book and available for execution. All order types, including market orders, start in this state before any fills occur. The order remains open until it reaches a terminal state (filled, cancelled, failed, or expired)
3.  **`FILLED`** - The order has been completely executed for the full requested quantity
4.  **`CANCELLED`** - The order has been cancelled either by client request or by the system. System cancellations may occur when market orders with Immediate or Cancel (`IMMEDIATE_OR_CANCEL`) instructions cannot be fully executed
5.  **`FAILED`** - The order could not be executed and was rejected by the system. This typically occurs with Fill or Kill (`FILL_OR_KILL`) orders that cannot be immediately filled in their entirety
6.  **`EXPIRED`** - The order was not fully executed before reaching its specified expiration time. This applies to time-limited orders, including Good Until Datetime (`GOOD_UNTIL_DATE_TIME`) limit orders and algorithmic orders such as TWAP/VWAP.

Examples of using Get Order by ID are shown below.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
GetOrderByOrderIdRequest request = new GetOrderByOrderIdRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .orderId("ORDER_ID_HERE")
    .build();
GetOrderByOrderIdResponse response = ordersService.getOrderByOrderId(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new GetOrderByOrderIdRequest("PORTFOLIO_ID_HERE", "ORDER_ID_HERE");
var response = ordersService.GetOrderByOrderId(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.GetOrderRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    OrderId: "ORDER_ID_HERE",
}
response, err := ordersService.GetOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, GetOrderRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = GetOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    order_id="ORDER_ID_HERE",
)
response = orders_service.get_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const ordersService = new OrdersService(client);
ordersService.getOrder({
    portfolioId: 'PORTFOLIO_ID_HERE',
    orderId: 'ORDER_ID_HERE'
}).then(async (response) => {
    console.log('Order: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

### Orders WebSocket

**Highly recommended for both REST and FIX integrations** - The Orders WebSocket provides real-time order status updates and fills without the need for polling status from REST. This is the most efficient way to monitor order lifecycle events as they occur. The WebSocket follows the same order state flow detailed above. To subscribe to order updates, you’ll need your **Service Account ID**, which can be obtained from the Prime UI. For comprehensive information about WebSocket implementation, authentication, and message formats, refer to the [WebSocket Feed Overview](https://developer.chrome.com/prime/websocket-feed/overview).

## Order Fills

An order can be split into multiple fills. Each fill contains specific details, such as venue that executed the fill. These fills are all tied to the same order ID.

-   [List Order Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-order-fills) retrieves fills specific to a given order ID.
-   [List Portfolio Fills](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-fills) retrieves all latest fills across a portfolio, which can also be subsequently filtered

Examples of using list Order Fills are shown below.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
ListOrderFillsRequest request = new ListOrderFillsRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .orderId("ORDER_ID_HERE")
.build();
ListOrderFillsResponse response = ordersService.listOrderFills(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new ListOrderFillsRequest("PORTFOLIO_ID_HERE", "ORDER_ID_HERE");
var response = ordersService.ListOrderFills(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.ListOrderFillsRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    OrderId: "ORDER_ID_HERE",
}
response, err := ordersService.ListOrderFills(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, ListOrderFillsRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = ListOrderFillsRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    order_id="ORDER_ID_HERE",
)
response = orders_service.list_order_fills(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-order-fills --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const ordersService = new OrdersService(client);
ordersService.listOrderFills({
    portfolioId: 'PORTFOLIO_ID_HERE',
    orderId: 'ORDER_ID_HERE',
}).then(async (response) => {
    console.log('Fills: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Cancelling an order

If an order is still open, a cancellation may be attempted via [Cancel Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/cancel-order). Note that a cancellation request does not guarantee a successful cancellation because the order status could change at any time (e.g., it may fill before the cancel request is processed).

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
OrdersService ordersService = PrimeServiceFactory.cancelOrdersService(client);
CancelOrderRequest request = new CancelOrderRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .orderId("ORDER_ID_HERE")
.build());
CancelOrderResponse orderResponse = ordersService.cancelOrder(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CancelOrderRequest("PORTFOLIO_ID_HERE", "ORDER_ID_HERE");
var cancelOrderResponse = orderService.CancelOrder(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.CancelOrderRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    OrderId: "ORDER_ID_HERE",
}
response, err := ordersService.CancelOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CancelOrderRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = CancelOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    order_id="ORDER_ID_HERE",
)
response = orders_service.cancel_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl cancel-order --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const ordersService = new OrdersService(client);
orderService.cancelOrder({
    portfolioId: 'PORTFOLIO_ID_HERE',
    orderId: 'ORDER_ID_HERE',
}).then(async (response) => {
    console.log('Order canceled: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Editing an order

Open orders can be modified using the [Edit Order](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/edit-order-beta) endpoint. This allows you to adjust parameters such as quantity, limit price, and expiry time without cancelling and recreating the order. Note that this feature is currently in beta.

-   Python
    

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, EditOrderRequest
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
orders_service = OrdersService(client)
request = EditOrderRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    order_id="ORDER_ID_HERE",
    orig_client_order_id="ORIGINAL_CLIENT_ORDER_ID",
    client_order_id=str(uuid.uuid4()),
    base_quantity="10",
    limit_price="0.35",
)
response = orders_service.edit_order(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).