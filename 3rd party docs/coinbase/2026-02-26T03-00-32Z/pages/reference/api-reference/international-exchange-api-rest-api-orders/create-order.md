# create order

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OrdersService ordersService = IntxServiceFactory.createOrdersService(client);
CreateOrderRequest request = new CreateOrderRequest.Builder()
    .instrument("BTC-PERP")
    .side("BUY")
    .size("0.0001")
    .orderType("LIMIT")
    .price("50000")
    .tif("GTC")
    .clientOrderId("1234567890")
    .build();
CreateOrderResponse response = ordersService.createOrder(request);

```

For more information, please visit the [INTX Java SDK](https://github.com/coinbase-samples/intx-sdk-java).

```
var ordersService = new OrdersService(client);
var request = new CreateOrderRequest(
    Instrument: "BTC-PERP",
    Side: "BUY",
    Size: "0.0001",
    OrderType: "LIMIT",
    Price: "50000",
    Tif: "GTC",
    ClientOrderId: "1234567890",
);
var response = ordersService.CreateOrder(request);

```

For more information, please visit the [INTX .NET SDK](https://github.com/coinbase-samples/intx-sdk-dotnet).

```
ordersSvc := orders.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Instrument: "BTC-PERP",
    Side: "BUY",
    Size: "0.0001",
    OrderType: "LIMIT",
    Price: "50000",
    Tif: "GTC",
    ClientOrderId: "1234567890",
}
response, err := ordersSvc.CreateOrder(context.Background(), request)

```

For more information, please visit the [INTX Go SDK](https://github.com/coinbase-samples/intx-sdk-go).

```
client = IntxClient()
request = CreateOrderRequest(
    instrument="BTC-PERP",
    side="BUY",
    size="0.0001",
    order_type="LIMIT",
    price="50000",
    tif="GTC",
    client_order_id="1234567890",
)
response = client.create_order(request)

```

For more information, please visit the [INTX Python SDK](https://github.com/coinbase-samples/intx-sdk-py).

```
const ordersService = new OrdersService(client);
ordersService.getOrder({
    clientOrderId: 'CLIENT_ORDER_ID_HERE',
    instrument: 'ETH-PERP',
    side: OrderSide.BUY,
    size: '0.001',
    orderType: OrderType.LIMIT,
    tif: TimeInForce.GTC,
    price: '4000',
}).then(async (response) => {
    console.log('Order Created: ', response);
})

```

For more information, please visit the [INTX TS SDK](https://github.com/coinbase-samples/intx-sdk-ts).

```
intxctl create-order --help

```

For more information, please visit the [INTX CLI](https://github.com/coinbase-samples/intx-cli).

#### Authorizations

The Client ID that owns the API Key for the request

The pass phrase affiliated with the API Key

A HMAC SHA-256 signature using the API Key secret on the string TIMESTAMP, METHOD, REQUEST\_PATH, BODY

The timestamp of when the request is being made

#### Body

A unique identifier assigned by the client. Must meet one of the following constraints:

-   18 characters long using all ASCII characters
-   20 characters long using alphanumeric characters and dash (-)
-   31 characters long using hexadecimal characters (0-9, a-f, lowercase and no preceding 0s)
-   A UUID using standard hyphenation and all lowercase (e.g., 36a9d3ee-32b7-460e-979a-121735af4824)

The side of the transaction

Available options

:

`BUY`,

`SELL`

The amount in base asset units

The time in force applied to an order. A Good Till Cancel (GTC) or Good Till Time (GTT) can last up to 30 days. An Immediate or Cancel (IOC) attempts to fill as much of the order as possible and/or cancels immediately when no resting orders meet or improve the order's price limit. A Fill or Kill (FOK) is canceled without filling if it does not completely fill immediately. Required for all order types except market orders.

Available options

:

`GTC`,

`IOC`,

`GTT`,

`FOK`

The name, ID, or UUID of the instrument the order wants to transact

The type of order being submitted

Available options

:

`LIMIT`,

`MARKET`,

`STOP_LIMIT`,

`STOP`,

`TAKE_PROFIT_STOP_LOSS`

The max or min price limit in quote asset units to buy or sell at (respectively). Only used for limit and stop limit order types.

The market price that activates a stop order

The limit price at which the TP/SL stop leg order will be placed. Only used for TP/SL order type.

The expiration time required for orders with the time in force set to GTT. Must not go beyond 30 days of the current time. Uses ISO-8601 format (e.g., 2023-03-16T23:59:53Z).

The ID or UUID of the portfolio the order belongs to (uses default portfolio if not defined)

Specifies the behavior for self match handling. None disables the functionality, new cancels the newest order, and both cancels both orders.

Available options

:

`NONE`,

`AGGRESSING`,

`RESTING`,

`BOTH`,

`DECREMENT_AND_CANCEL`

Only submit the order if it will rest on the order book

Only submit the order if it will close an existing order

The algorithmic trading strategy to use for the order

#### Response

A unique identifier assigned by the exchange

Example:

`43877033468085760`

A unique identifier assigned by the client

The side of the transaction

Available options

:

`BUY`,

`SELL`

The unique identifier of the instrument the order wants to transact in

The UUID of the instrument the order wants to transact in

Example:

`"359f66d8-4235-47c3-9733-0fbfe2cfaa0a"`

The name of the instrument the order wants to transact in

The unique identifier of the portfolio the order was submitted under

The UUID of the portfolio the order was submitted under

Example:

`"b80ec69b-1229-4bcf-a6a8-c506ffd74c20"`

The type of the order

Available options

:

`LIMIT`,

`MARKET`,

`STOP_LIMIT`,

`STOP`,

`TAKE_PROFIT_STOP_LOSS`

The max or min price limit in quote asset units to buy or sell at (respectively). Only used for limit and stop limit order types.

The market price that activates a stop order

The limit price at which the TP/SL stop leg order will be placed. Only used for TP/SL order type.

The amount in base asset units

The time in force applied to an order. A Good Till Cancel (GTC) can last up to 30 days. An Immediate or Cancel (IOC) attempts to fill as much of the order as possible and/or cancels immediately when no resting orders meet or improve the order's price limit. A Fill or Kill (FOK) is canceled without filling if it does not completely fill immediately. This will not be populated for assignments

Available options

:

`GTC`,

`IOC`,

`GTT`,

`FOK`

The expiration time for orders with the time in force set to GTT. Uses ISO-8601 format (e.g., 2023-03-16T23:59:53Z).

Example:

`"2023-03-16T23:59:53.000Z"`

Specifies the behavior for self match handling. None disables the functionality, new cancels the newest order, and both cancels both

Available options

:

`NONE`,

`AGGRESSING`,

`RESTING`,

`BOTH`,

`DECREMENT_AND_CANCEL`

The most recent type of event that happened to the order

Available options

:

`NEW`,

`TRADE`,

`CANCELED`,

`REPLACED`,

`PENDING_CANCEL`,

`REJECTED`,

`PENDING_NEW`,

`EXPIRED`,

`PENDING_REPLACE`,

`STOP_TRIGGERED`

Example:

`"2023-03-16T23:59:53.000Z"`

Example:

`"2023-03-16T23:59:53.000Z"`

The type of the order

Available options

:

`WORKING`,

`DONE`

The amount of the order remaining open on the exchange

The traded quantity on the order

The average price that the order has traded at so far

The exchange fee affiliated with the trade (only for trade events)

Indicates that the order was submitted with the `post_only` instruction

Indicates that the order was submitted with the `close_only` instruction

Specifies the algorithmic trading strategy for the order

A text message that gets populated for canceled orders