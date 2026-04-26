# get all orders

## Pending Orders

Orders with a “pending” status have fewer fields in the response.

-   Pending limit orders do not have `stp`, `time_in_force`, `expire_time`, and `post_only`.
-   Pending market orders have the same fields as a pending limit order minus `price` and `size`, and no market specific fields (`funds`, `specified_funds`).
-   Pending stop orders have the same fields as a pending limit order and no stop specific fields (`stop`, `stop_price`).

Order Type

Does Not Have These Fields

Pending Limit Order

`stp`, `time_in_force`, `expire_time`, `post_only`

Pending Market Order

`stp`, `time_in_force`, `expire_time`, `post_only`, `price`, `size`, `funds`, `specified_funds`

Pending Stop Order

`stp`, `time_in_force`, `expire_time`, `post_only`, `stop`, `stop_price`

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

## Order Status and Settlement

Orders which are no longer resting on the order book, are marked with the `done` status. There is a small window between an order being `done` and `settled`. An order is settled when all of the fills have settled and the remaining holds (if any) have been removed.

## Polling

For high-volume trading it is strongly recommended that you maintain your own list of open orders and use one of the streaming market data feeds to keep it updated. You should poll the open orders endpoint once when you start trading to obtain the current state of any open orders. `executed_value` is the cumulative match `size` \* `price` and is only present for orders placed after 2016-05-20.

This request is paginated. See [Pagination](https://developer.chrome.com/exchange/rest-api/pagination) for more information.

#### Authorizations

#### Query Parameters

Filter results by a specific profile\_id

Filter results by a specific product\_id

sortedBy

enum<string>

default:created\_at

Sort criteria for results.

Available options

:

`created_at`,

`price`,

`size`,

`order_id`,

`side`,

`type`

Ascending or descending order, by `sortedBy`

Available options

:

`desc`,

`asc`

Filter results by minimum posted date

Filter results by maximum posted date

Used for pagination. Sets start cursor to `before` date.

Used for pagination. Sets end cursor to `after` date.

limit

integer<int64>

default:100

required

Limit on number of results to return.

Array with order statuses to filter by.

Available options

:

`open`,

`pending`,

`rejected`,

`done`,

`active`,

`received`,

`all`

Market type which the order was traded in.

#### Response

book the order was placed on

side

enum<string>

default:buy

required

Available options

:

`buy`,

`sell`

type

enum<string>

default:limit

required

Available options

:

`limit`,

`market`,

`stop`

if true, forces order to be `maker` only

created\_at

string<date-time>

required

timestamp at which order was placed

fees paid on current filled amount

amount (in base currency) of the order that has been filled

status

enum<string>

default:open

required

Available options

:

`open`,

`pending`,

`rejected`,

`done`,

`active`,

`received`,

`all`

true if funds have been exchanged and settled

price per unit of base currency

amount of base currency to buy/sell

profile\_id that placed the order

amount of quote currency to spend (for market orders)

Available options

:

`GTC`,

`GTT`,

`IOC`,

`FOK`

timestamp at which order expires

timestamp at which order was done

reason order was done (filled, rejected, or otherwise)

reason order was rejected by engine

Available options

:

`loss`,

`entry`

price (in quote currency) at which to execute the order

market type where order was traded

maximum visible quantity for iceberg order

order id for the visible order for iceberg order

stop limit price for TPSL order