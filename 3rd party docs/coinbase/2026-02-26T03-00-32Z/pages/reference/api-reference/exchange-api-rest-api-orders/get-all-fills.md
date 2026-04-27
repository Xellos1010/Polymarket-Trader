# get all fills

Get a list of recent fills of the API key’s profile.

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

## Settlement and Fees

Fees are recorded in two stages. Immediately after the matching engine completes a match, the fill is inserted into our datastore. Once the fill is recorded, a settlement process settles the fill and credit both trading counterparties. The `fee` field indicates the fees charged for this individual fill.

### Liquidity

The `liquidity` field indicates if the fill was the result of a liquidity provider or liquidity taker. `M` indicates Maker and `T` indicates Taker.

Fills are returned sorted by descending `trade_id` from the largest `trade_id` to the smallest `trade_id`. The `CB-BEFORE` header has this first trade ID so that future requests using the `cb-before` parameter fetch fills with a greater trade ID (newer fills). See [Pagination](https://developer.chrome.com/exchange/rest-api/pagination) for more information.

#### Authorizations

#### Query Parameters

limit to fills on a specific order. Either `order_id` or `product_id` is required.

limit to fills on a specific product. Either `order_id` or `product_id` is required.

limit

integer<int64>

default:100

Limit on number of results to return.

Used for pagination. Sets start cursor to `before` id.

Used for pagination. Sets end cursor to `after` id.

Market type which the order was filled in.

Available options

:

`spot`,

`rfq`

Search by minimum posted date time and is inclusive of time provided. Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.

Search by maximum posted date time and is inclusive of time provided. Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.

#### Response

id of trade that created the fill

book the order was placed on

profile\_id that placed the order

liquidity

enum<string>

default:M

required

price per unit of base currency

amount of base currency to buy/sell

fees paid on current filled amount

created\_at

string<date-time>

required

side

enum<string>

default:buy

required

Available options

:

`buy`,

`sell`

true if funds have been exchanged and settled

true if funds have been exchanged and settled

funding currency which the order was filled in

market type which the order was filled in