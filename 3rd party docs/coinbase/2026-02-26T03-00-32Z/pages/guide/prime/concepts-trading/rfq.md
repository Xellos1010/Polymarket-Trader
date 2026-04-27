# rfq

RFQ (Request For Quote) enables off-order-book trading by routing trades directly to connected market makers. The primary benefits for API clients include price holds and guaranteed full execution at agreed-upon prices. The RFQ workflow consists of three steps:

1.  **Create a quote** - Submit a quote request to market makers
2.  **Evaluate the quote** - Assess the returned price and terms
3.  **Accept or expire** - Accept the quote within the time limit or let it expire

RFQ quotes are held for a maximum of 2.5 seconds, starting when the market maker submits their quote. Actual available time may be less depending on network latency and geographic location. This duration is optimized to minimize slippage versus the Central Limit Order Book (CLOB). Since RFQ trades occur off the order book, the orders and l2\_data websockets are not applicable to RFQ orders.

## Creating a Quote

To create a quote, use the [Create Quote Request](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/create-quote-request) REST API or [Quote Request (R)](https://developer.chrome.com/prime/fix-api/messages#quote-request-r) via FIX. **Requirements:**

-   Quantity must be specified in base units (quote units are not yet supported)
-   A marketable limit price is required:
    -   For BUY orders: limit price must be above the current mid price
    -   For SELL orders: limit price must be below the current mid price

Not all product pairs supported via the CLOB are available via RFQ. For full product availability, as well as RFQ-specific minimum and maximum order sizes, see [List Portfolio Products](https://developer.chrome.com/api-reference/prime-api/rest-api/products/list-portfolio-products). Quote creation should be reserved for situations where you or your users intend to execute a trade, not for recurring price data polling. Misuse may result in access restrictions to this endpoint.

-   Python
    
-   TS/JS
    

```
import uuid
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, CreateQuoteRequest
from prime_sdk.enums import OrderSide
def main():
    credentials = Credentials.from_env("PRIME_CREDENTIALS")
    client = Client(credentials)
    orders_service = OrdersService(client)
    request = CreateQuoteRequest(
        product_id="BTC-USD",
        side=OrderSide.BUY,
        client_quote_id=str(uuid.uuid4()),
        limit_price="150000",
        base_quantity="0.01"
    )
    try:
        response = orders_service.create_quote(request)
        print(response)
    except Exception as e:
        print(f"failed to create quote: {e}")
if __name__ == "__main__":
    main()

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
require('dotenv').config();
const {
    CoinbasePrimeClient,
    CoinbasePrimeCredentials,
    OrdersService,
    OrderSide,
} = require('@coinbase-sample/prime-sdk-ts');
const creds = JSON.parse(process.env.PRIME_CREDENTIALS);
const portfolioId = process.env.PORTFOLIO_ID;
const createQuote = async (orderService, portfolioId) => {
    const quoteRequest = {
        portfolioId,
        productId: 'BTC-USD',
        side: OrderSide.Buy,
        clientOrderId: crypto.randomUUID(), // This maps to clientQuoteId in the RFQ
        limitPrice: '150000',
        baseQuantity: '0.01',
    };
    console.log('Creating RFQ quote: ', quoteRequest);
    const response = await orderService.createQuote(quoteRequest);
    return response;
};
const credentials = new CoinbasePrimeCredentials(
    creds.AccessKey,
    creds.SecretKey,
    creds.Passphrase
);
const client = new CoinbasePrimeClient(credentials);
const ordersService = new OrdersService(client);
createQuote(ordersService, portfolioId)
    .then((response) => {
        console.dir(response, { depth: null });
    })
    .catch((err) => {
        console.dir(err, { depth: null });
    });

```

The quote response includes several critical parameters that require immediate evaluation:

-   **`best_price`** - The quoted price from market makers
-   **`order_total`** - Total cost for the transaction
-   **`quote_id`** - Required for accepting the quote in the next step

## Accepting a Quote

If you decide to proceed with the quoted price, quickly follow up with an [Accept Quote](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/accept-quote) request. You must include the `quote_id` from the previous response.

-   Python
    
-   TS/JS
    

```
import uuid
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.orders import OrdersService, AcceptQuoteRequest
from prime_sdk.enums import OrderSide
def main():
    credentials = Credentials.from_env("PRIME_CREDENTIALS")
    client = Client(credentials)
    orders_service = OrdersService(client)
    request = AcceptQuoteRequest(
        product_id="BTC-USD",
        side=OrderSide.BUY,
        client_order_id=str(uuid.uuid4()),
        quote_id="QUOTE_ID_HERE"
    )
    try:
        response = orders_service.accept_quote(request)
        print(response)
    except Exception as e:
        print(f"failed to accept quote: {e}")
if __name__ == "__main__":
    main()

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
require('dotenv').config();
const {
    CoinbasePrimeClient,
    CoinbasePrimeCredentials,
    OrdersService,
    OrderSide,
} = require('@coinbase-sample/prime-sdk-ts');
const creds = JSON.parse(process.env.PRIME_CREDENTIALS);
const portfolioId = process.env.PORTFOLIO_ID;
const acceptQuote = async (orderService, portfolioId, quoteId) => {
    const acceptRequest = {
        portfolioId,
        productId: 'BTC-USD',
        side: OrderSide.Buy,
        clientOrderId: crypto.randomUUID(),
        quoteId: quoteId,
    };
    console.log('Accepting quote: ', acceptRequest);
    const response = await orderService.acceptQuote(acceptRequest);
    return response;
};
const credentials = new CoinbasePrimeCredentials(
    creds.AccessKey,
    creds.SecretKey,
    creds.Passphrase
);
const client = new CoinbasePrimeClient(credentials);
const ordersService = new OrdersService(client);
// You would typically get this quoteId from a previous createQuote call
const exampleQuoteId = 'your-quote-id-here';
acceptQuote(ordersService, portfolioId, exampleQuoteId)
    .then((response) => {
        console.dir(response, { depth: null });
    })
    .catch((err) => {
        console.dir(err, { depth: null });
    });

```

After quote acceptance, RFQ orders are ledgered and may be looked up via the **Get Order by ID**: Poll the [Get Order by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/get-order-by-order-id) endpoint.