# trading

## Orders and Order Types

**Orders** are the fundamental building blocks of trading on Coinbase Exchange. Understanding the different order types and their behaviors is crucial for building effective Crypto-as-a-Service (CaaS) trading strategies.

## Product Pairs

Before placing orders, you need to know which trading pairs are available. Product pairs represent the available markets for trading. To list product pairs, as well as important metadata about order size requirements, run the following:

-   Go
    

```
package main
import (
    "context"
    "encoding/json"
    "fmt"
    "log"
    "github.com/coinbase-samples/core-go"
    "github.com/coinbase-samples/exchange-sdk-go/client"
    "github.com/coinbase-samples/exchange-sdk-go/credentials"
    "github.com/coinbase-samples/exchange-sdk-go/products"
)
func main() {
    credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
    if err != nil {
        log.Fatalf("unable to read credentials from environment: %v", err)
    }
    httpClient, err := core.DefaultHttpClient()
    if err != nil {
        log.Fatalf("unable to load default http client: %v", err)
    }
    client := client.NewRestClient(credentials, httpClient)
    productsSvc := products.NewProductsService(client)
    request := &products.ListProductsRequest{}
    response, err := productsSvc.ListProducts(context.Background(), request)
    if err != nil {
        log.Fatalf("unable to list products: %v", err)
    }
    output, err := json.MarshalIndent(response, "", "  ")
    if err != nil {
        log.Fatalf("error marshaling response to JSON: %v", err)
    }
    fmt.Println(string(output))
}

```

### Limit Orders

**Limit orders** allow you to specify both the price and size of your trade, providing precise control over execution:

-   **Price control**: Order executes at your specified price or better
-   **Guaranteed rate**: Never pay more (for buys) or receive less (for sells) than your limit price
-   **Queue position**: Orders are filled based on price-time priority
-   **Maximum open orders**: Each profile can place a maximum of 500 open orders on a product

### Market Orders

**Market orders** prioritize immediate execution over price control:

-   **Immediate execution**: Order executes immediately against available liquidity
-   **Always taker**: Market orders always consume liquidity and pay taker fees
-   **No price guarantee**: Execution price depends on current market conditions
-   **Slippage risk**: Large orders may execute across multiple price levels

## Order Sizing: Size vs Funds

When placing orders, you can specify the order amount using two different parameters:

-   **Size**: Amount in base currency (e.g., BTC in BTC-USD pair)
-   **Funds**: Amount in quote currency (e.g., USD in BTC-USD pair)
-   **Market orders**: Can use either `size` or `funds`, but not both

### Stop Orders

**Stop orders** enable automated risk management and entry strategies:

-   **Stop-loss orders**: Automatically exit positions when price moves against you
-   **Stop-entry orders**: Enter positions when price breaks through key levels
-   **Trigger mechanism**: Becomes a market order when stop price is reached

### Advanced Order Features

#### Post-Only Orders

**Post-only orders** guarantee maker status and liquidity provision:

-   **Maker guarantee**: Order rejected if any part would execute immediately
-   **Fee optimization**: Always qualify for maker fees or rebates

#### Self-Trade Prevention

Configure how your orders behave when they would match against each other:

-   **Decrease and Cancel (DC)**: Reduce newer order size, cancel if it would fully execute
-   **Cancel Oldest (CO)**: Cancel the older resting order
-   **Cancel Newest (CN)**: Cancel the newer incoming order
-   **Cancel Both (CB)**: Cancel both orders

#### Time in Force Options

Control how long your orders remain active:

-   **Good Till Cancel (GTC)**: Order remains active until filled or manually canceled
-   **Good Till Time (GTT)**: Order expires at specified time
-   **Immediate or Cancel (IOC)**: Fill immediately available quantity, cancel remainder
-   **Fill or Kill (FOK)**: Fill entire order immediately or cancel completely

To create an order, try the following:

-   Go
    
-   TS/JS
    

examples/listCoinbaseAccounts/cmd.go

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
ordersSvc := accounts.NewOrdersService(client)
request := &orders.CreateOrderRequest{
    Type:        "market",
    Side:        "buy",
    ProductId:   "BTC-USD",
    ClientOid:   "UUID",
    Funds:       "10",
}
response, err := ordersSvc.CreateOrder(context.Background(), request)

```

To learn more about this SDK, please visit the [Exchange Go SDK](https://github.com/coinbase-samples/exchange-sdk-go).

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

## Listing Orders

Retrieve your current open orders. Only open or unsettled orders are returned by default:

-   **Default behavior**: Returns only open orders, settled orders are excluded
-   **Order states**: Orders may change state between request and response
-   **Pending orders**: Have limited fields in response (missing `stp`, `time_in_force`, `expire_time`, `post_only`)

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
if err != nil {
    log.Fatalf("unable to read credentials from environment: %v", err)
}
httpClient, err := core.DefaultHttpClient()
if err != nil {
    log.Fatalf("unable to load default http client: %v", err)
}
client := client.NewRestClient(credentials, httpClient)
ordersSvc := orders.NewOrdersService(client)
request := &orders.ListOrdersRequest{}
response, err := ordersSvc.ListOrders(context.Background(), request)
if err != nil {
    log.Fatalf("unable to list orders: %v", err)
}
output, err := json.MarshalIndent(response, "", "  ")
if err != nil {
    log.Fatalf("error marshaling response to JSON: %v", err)
}
fmt.Println(string(output))

```