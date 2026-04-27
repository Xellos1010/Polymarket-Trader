# get order preview

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
OrdersService ordersService = PrimeServiceFactory.createOrdersService(client);
GetOrderPreviewRequest request = new GetOrderPreviewRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .productId("ADA-USD")
    .side(OrderSide.BUY)
    .type(OrderType.MARKET)
    .baseQuantity("10.0")
    .build();
GetOrderPreviewResponse orderResponse = ordersService.getOrderPreview(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new GetOrderPreviewRequest("PORTFOLIO_ID_HERE")
{
    BaseQuantity = "5",
    LimitPrice = "0.32",
    Side = OrderSide.BUY,
    ProductId = "ADA-USD",
    Type = OrderType.LIMIT,
    ExpiryTime = new DateTimeOffset(DateTime.UtcNow.AddMinutes(5)).ToString("o"),
};
var getOrderPreviewResponse = orderService.GetOrderPreview(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
ordersService := orders.NewOrdersService(client)
request := &orders.GetOrderPreviewRequest{
    Order: &model.Order{
        PortfolioId:   "PORTFOLIO_ID_HERE",
        BaseQuantity:  "5",
        LimitPrice:    "0.32",
        Side:          "BUY",
        ProductId:     "ADA-USD",
        Type:          "LIMIT",
        ExpiryTime:    time.Now().UTC().Add(5 * time.Minute).Format(time.RFC3339),
    },
}
response, err := ordersService.GetOrderPreviewRequest(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = GetOrderPreviewRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    base_quantity="5",
    limit_price="0.32",
    side="BUY",
    product_id="ADA-USD",
    type="LIMIT",
    expiry_time=(datetime.datetime.now() + datetime.timedelta(minutes=5)).isoformat() + "Z",
)
response = prime_client.get_order_preview(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-order-preview --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const ordersService = new OrdersService(client);
const today = new Date();
ordersService.createOrderPreview({
    portfolioId: "PORTFOLIO_ID_HERE",
    baseQuantity: "5",
    limitPrice: "0.32",
    side: OrderSide.BUY,
    productId: "ADA-USD",
    type: OrderType.LIMIT,
    expiryTime: date.setDate(date.getDate() + 1),
}).then(async (response) => {
    console.log('Order Preview: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

The ID of the portfolio that owns the order

#### Body

-   UNKNOWN\_ORDER\_SIDE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   BUY: Buy order
-   SELL: Sell order

Available options

:

`BUY`,

`SELL`

-   UNKNOWN\_TIME\_IN\_FORCE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   GOOD\_UNTIL\_DATE\_TIME: Expires at a certain date/time
-   GOOD\_UNTIL\_CANCELLED: Order stays on the books until cancelled
-   IMMEDIATE\_OR\_CANCEL: Order is executed immediately at submission or is cancelled
-   FILL\_OR\_KILL: Order is executed immediately and fully at submission or is cancelled

Available options

:

`GOOD_UNTIL_DATE_TIME`,

`GOOD_UNTIL_CANCELLED`,

`IMMEDIATE_OR_CANCEL`,

`FILL_OR_KILL`

Specifies whether the order is treated as a post only order.

The maximum order size that will show up on venue order books (in quote currency).

The maximum order size that will show up on venue order books (in base currency).

-   UNKNOWN\_PEG\_OFFSET\_TYPE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   PEG\_OFFSET\_TYPE\_PRICE: Offset specified in price units
-   PEG\_OFFSET\_TYPE\_BPS: Offset specified in basis points (BPS)
-   PEG\_OFFSET\_TYPE\_DEPTH: Offset specified in depth

Available options

:

`PEG_OFFSET_TYPE_PRICE`,

`PEG_OFFSET_TYPE_BPS`,

`PEG_OFFSET_TYPE_DEPTH`

#### Response

The ID of the portfolio that owns the order

Example:

`"3e1fe27e-26fe-46d8-b118-c752a2ae6b47"`

The ID of the product being traded by the order

-   UNKNOWN\_ORDER\_SIDE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   BUY: Buy order
-   SELL: Sell order

Available options

:

`BUY`,

`SELL`

Order size in base asset units (either `base_quantity` or `quote_value` is required)

Order size in quote asset units, i.e. the amount the user wants to spend (when buying) or receive (when selling); the quantity in base units will be determined based on the market liquidity and indicated `quote_value`. Either `base_quantity` or `quote_value` is required

The limit price (required for TWAP, VWAP, LIMIT, and STOP\_LIMIT orders)

The start time of the order in UTC (only applies to TWAP orders.)

Example:

`"2021-05-31T09:59:59.000Z"`

The expiry time of the order in UTC (TWAP, VWAP, LIMIT and STOP\_LIMIT GTD only). Required for TWAP and VWAP orders if historical\_pov is unspecified

Example:

`"2021-05-31T10:59:59.000Z"`

-   UNKNOWN\_TIME\_IN\_FORCE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   GOOD\_UNTIL\_DATE\_TIME: Expires at a certain date/time
-   GOOD\_UNTIL\_CANCELLED: Order stays on the books until cancelled
-   IMMEDIATE\_OR\_CANCEL: Order is executed immediately at submission or is cancelled
-   FILL\_OR\_KILL: Order is executed immediately and fully at submission or is cancelled

Available options

:

`GOOD_UNTIL_DATE_TIME`,

`GOOD_UNTIL_CANCELLED`,

`IMMEDIATE_OR_CANCEL`,

`FILL_OR_KILL`

Indicate the total commission paid on this order in quote currency - only applicable if the order has any fills

How much slippage is expected

Current best bid for order book

Current best ask for order book

Indicate expected average filled price based on the current order book

The estimated participation rate for a TWAP/VWAP order. This field can be specified instead of expiry time, and will be used to compute the expiry time of the order based on historical participation rate.

The maximum order size that will show up on venue order books.

The maximum order size that will show up on venue order books (in quote currency).

The maximum order size that will show up on venue order books (in base currency).